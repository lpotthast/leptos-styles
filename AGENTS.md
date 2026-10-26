# Repository guidance

`leptos-styles` collects the inline style of a Leptos element in one value, `Styles`, which components pass on and
add to. The declarations come from `typed-css`, which is always a dependency. `UncheckedDeclaration` is the escape
hatch for CSS that `typed-css` does not support.

## Product rules

- `Styles::with` is the one way to add a declaration. It accepts a declaration, a `Styles`, an `Option` of either, or
  a closure that returns one of them, through the `IntoStyleEntry` trait. A new kind of value gets a
  `From<_> for StyleEntry` impl, so that `with`, `StyleEntry::new`, `Option`s, closures, and the conversions into
  `Styles` all accept it. Do not add new methods for it.
- There is one rule for combining two `Styles`. A `Styles` added with `with` is nested, so its fallback layers stay
  below its own first layer and above the fallback layers of the receiving `Styles`. A closure that returns a
  `Styles` follows the same rule. Do not add a second way to combine them with other priorities.
- Keep a checked `typed_css::Declaration` whole. Never split it into a property name and a value that are stored
  separately.
- One grammar can serve several properties. The property constant decides which declaration is built, and the value
  type only decides which values are accepted.
- CSS text enters only as an `UncheckedDeclaration` or through `Styles::parse`. `UncheckedDeclaration` checks the
  structure of the text, not its meaning. It must reject everything that could end the declaration or raise its
  priority. That includes a `;` or `!` outside of strings, functions, and `url()`, unbalanced quotes, parentheses,
  brackets, or braces, comments, escapes outside of strings and `url()`, and unquoted `url()` contents that a browser
  would treat as a bad URL. The scanner reads forwards with the token rules of CSS Syntax Level 3. Never decide a
  token by looking backwards, because `#url(`, `@url(`, and `<!--url(` group differently than they look. When it is
  unclear how a browser tokenizes an input, reject the input. After changing the scanner, compare it with a real CSS
  tokenizer, such as `cssparser`, on generated input.
- Do not add conversions from strings or tuples of strings into declarations or `Styles`.
- Documentation leads with checked declarations from `leptos_styles::css`. Use `UncheckedDeclaration` and
  `Styles::parse` only in their own documentation and in the README section about CSS that `typed-css` does not
  support. Tests may use unchecked declarations freely.
- A reactive declaration may change both its property and its value. Read it exactly once per render, and use that
  same value for choosing the winning layer and for the output.

## Commands

Day-to-day work goes through `just`. `just verify` is the check before a change. CI also runs `just deny`, which
`just verify` leaves out because it needs the advisory database.

| Task                       | Command               |
|----------------------------|-----------------------|
| Format                     | `just fmt`            |
| Update, sort, and format   | `just tidy`           |
| Type-check natively        | `just check`          |
| Type-check for WASM        | `just check-wasm`     |
| Run Clippy                 | `just clippy`         |
| Run unit tests             | `just test-lib`       |
| Run doc tests              | `just test-doc`       |
| Run the integration tests  | `just test-integration` |
| Run the browser test       | `just browser-test`   |
| Build the documentation    | `just doc`            |
| Run the full verification  | `just verify`         |

Other useful commands:

- `just browser-test-visible` opens Chrome visibly, and `just browser-test-pause` also pauses for inspection.
- `just serve-test-app` serves the test app at `127.0.0.1:4200`.
- `just deny` runs the blocking supply-chain checks. `just semver-check` does not block. While the crate is below 1.0,
  a breaking release bumps the minor version.
- A release that depends on an unpublished `typed-css` version needs a local `[patch.crates-io]`, for example in an
  untracked `.cargo/config.toml`, until that version is published. Publish `typed-css` first.
- When you change `IntoStyleEntry`, also check the nightly build with `just check-nightly`.

Reproduce a reported failure with the same command before you narrow it down, and run that command again after the
fix. After removing or renaming API, search for the old names and for conversions that would let raw text in, instead
of stopping at passing tests.

## Features and compatibility

- `typed-css` is a required dependency with its `leptos` feature. `leptos_styles::css` re-exports it, so that a
  downstream crate always uses the same version.
- The crate has no features. A downstream crate may enable the `nightly` feature of Leptos, which lets signals be
  called like closures, so `IntoStyleEntry` accepts them as reactive values. `just check-nightly` builds that
  configuration. Do not forward `nightly` again, because Cargo already enables it for every crate that uses Leptos.
- Reactive values are closures such as `move || signal.get()` on stable. Do not add conversions from `Signal` types,
  because they can overlap with Leptos' own conversions when a downstream crate enables more Leptos features.
- The MSRV is Rust 1.89.0. Do not use newer standard-library APIs unless the MSRV is bumped in `Cargo.toml`, the
  README, and CI together.
- Keep `leptos = 0.8.22` in step with `typed-css`. Two Leptos versions in one lockfile fail CI.
- Every public item has a doc comment. CI builds the documentation with `RUSTDOCFLAGS=-D warnings`.
- Clippy's `pedantic` group is denied. Satisfy a lint instead of extending the allow-list.

## Modules

- `unchecked.rs` holds `UncheckedDeclaration` with the normalization of property names, the structural scanner, and
  the parser for declaration lists.
- `style_entry.rs` holds `StyleEntry`, one declaration that may be absent or reactive, or a nested `Styles`, and
  `IntoStyleEntry` with the markers for the kinds of values that `Styles::with` accepts. Its crate-private
  `StyleDeclaration` is either checked or unchecked.
- `styles.rs` holds the public `Styles` API, the storage of its entries with the layer of each, and
  `with_winning`. That function is the one place that reads the entries and picks the winning declarations, for
  both the string output and the DOM. Layer 0 has the highest priority.
- `into_style.rs` holds Leptos' `IntoStyle` for `Styles` and the render state, including the record of the
  declarations that `Styles` has set. `UncheckedDeclaration` and `StyleEntry` render through `Styles`, so they share
  that one implementation.
- `lib.rs` lists the small public API and re-exports `typed-css` as `css`.

A module holds one concept. Use the narrowest visibility that works, which is `pub(crate)` unless an item belongs to
the published API. Put validation on the type that guarantees the result, and trust that guarantee afterwards. Do not
create grab-bag modules such as `utils` or `helpers`.

## Rendering and fallback rules

- `Styles::with_fallback(other)` gives `other` a new layer below every layer of `self`. The layers inside `other` stay
  in their order below that new layer.
- A render reads every present declaration once. For each property, the layer of the highest priority that has a
  declaration of the property wins, and all of that layer's declarations of the property render in insertion order.
- Winning declarations render layer by layer, from the lowest priority to the highest. A shorthand in a fallback
  layer, such as `padding`, must never override a longhand of a higher layer, such as `padding-top`.
- A closure is stored as an `Arc<dyn Fn>`, not as a Leptos signal, so that a `Styles` does not depend on the owner
  that created it.
- A property may appear more than once within one layer on purpose. Do not deduplicate it or warn about it.
- An unchecked standard property name is trimmed and lowercased. An unchecked custom property name starting with `--`
  is trimmed and keeps its case. Checked declarations need no normalization.
- Do not implement `Display` for `Styles` or `StyleEntry`, because formatting would read signals outside a reactive
  context. `to_style_string` reads them explicitly.
- In the browser, an update that changes no declaration writes nothing. Otherwise, the update starts at the first
  changed declaration. It removes the properties from there on and sets them again in order. It writes every
  declaration again instead if an unchanged property shows no value before or after that, because the update then
  cannot tell whether it cleared part of an unchanged declaration, such as a `padding` whose `padding-top` it
  removed. Some shorthands, such as `padding: var(--space)` next to a `padding-top`, never show a value, so they
  always take the full rewrite. The check also covers a rejected value, a property that repeats an unchanged one, and
  a property that other code removed. Properties before the
  first change, and properties that `Styles` never declared, such as those of `style:` directives, stay untouched.
  When no declaration remains and the attribute is empty, remove the attribute.
- A nested `Styles` passes on each declaration with its rank inside the nested `Styles`. `resolve_ranked` builds the
  rank as the path of layers, so that a fallback inside a nested `Styles` stays below the layer of the entry.
- Hydration looks for the client's declarations in the server's attribute, at the start of a declaration, and only
  changes the element if they are missing. Other code, such as a `style:` directive, can write into the same
  attribute. A declaration that only the server rendered stays on the element, because `Styles` cannot tell it apart
  from the declarations of other code.
- `is_reactive()` is cached when an entry is added. A `Styles` without reactive declarations installs no render effect.
  Keep the transitions between static and reactive in both directions, and the reset and rebuild behavior, working.

## Tests

- Unit tests live next to their module.
- `tests/declarations.rs` tests the declarations through `Styles`. It covers the pairing of properties and
  values, optional and reactive declarations, mixed lists, declarations that change their property, fallback layers,
  and unchecked declarations.
- `testing/test-app` is a separate workspace that only the browser test uses. Each scenario has one component in
  `testing/test-app/src/targets` and one matching assertion in `tests/ui_tests/test_styles.rs`.
- Browser assertions compare the browser's parsed `element.style.cssText`. So expected strings use the browser's
  serialization, such as `color: rgb(255, 0, 0);`, and values that the browser rejects are absent.
- The browser test is the contract for server-side rendering, hydration, reactive updates, the removal of the `style`
  attribute, transitions between static and reactive styles, restoring properties after other code changed them,
  coexistence with `style:` directives, fallback layers, and checked values. Do not rely only on native tests when you
  change one of these paths.
- Keep the test app small and dedicated to this crate.

## Writing

These rules apply to everything written in the repository: the README, the CHANGELOG, this file, doc comments, and
code comments.

- Write complete sentences with a subject and a verb, in the active voice. Say who does what.
- Use one idea per sentence, and split long sentences.
- Use plain, common words. A reader should understand each sentence on the first read.
- Name things instead of writing "it" and "its", and avoid chains of possessives.
- Use connecting words such as "because", "so", and "so that". Use a colon only to introduce a list or an example.
- Do not use semicolons in prose. Split the sentence or join the parts with a connecting word. Semicolons in code are
  fine.
- Documentation describes what the code is and why. It does not describe history, such as what an item replaced. The
  CHANGELOG is the place for history.
- Only document what a reader cannot infer from a name. Document units, the meaning of absent values, constraints, and
  invariants.
- A doc comment starts with a sentence that says what the item is or does. Keep the examples short and runnable.
