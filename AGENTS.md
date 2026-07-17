# Repository guidance

## Product contract

- `leptos-styles` is a prop-drillable, reactive owner of one complete inline `style` attribute.
- The default feature set includes `typed-css`. Its normal authoring surface is declaration-first:
  `with(PaddingProperty.declare(Padding::all(px(16))))` and the matching `add` methods.
  Always-present reactive declarations use `with_reactive(move || ...)` / `add_reactive(move || ...)`;
  optional sources use the optional variants.
- Preserve the `leptos-css` checked boundary. Store `CheckedDeclaration` intact; never recover a
  property name and store it beside a generic value.
- The same value grammar may serve several properties. The property selector says which
  declaration is built; the value type says only which values are accepted.
- Checked APIs stay fail-closed. Raw names and strings are allowed only through methods ending in
  `_unchecked` or through `StyleEntry::parse` / `Styles::parse_css`.
- Do not restore implicit raw tuple conversions, a `PropertyName`-to-property bridge, or the old
  `Style` plus `CssValue` pairing.
- A reactive complete declaration may change both property and value. Resolve it exactly once per
  serialization pass and use that same snapshot for priority selection and output.

## Commands

Day-to-day work uses `just`. `just verify` is the release-facing check and covers the default
checked API plus the no-default unchecked-only shape.

| Task | Default checked API | No default features |
|---|---|---|
| Format | `just fmt` | same |
| Native check | `just check` | `just check-no-default` |
| WASM check | `just check-wasm` | `just check-wasm-no-default` |
| Clippy | `just clippy` | `just clippy-no-default` |
| Unit tests | `just test-lib` | `just test-lib-no-default` |
| Doc tests | `just test-doc` | `just test-doc-no-default` |
| Checked integration tests | `just test-typed-css` | n/a |
| Browser test | `just browser-test` | n/a |
| Full verification | `just verify` | included |

Other useful commands:

- `just browser-test-visible` opens Chrome; `just browser-test-pause` also pauses for inspection.
- `just serve-test-app` serves the fixture at `127.0.0.1:4200`.
- `just deny` runs the blocking supply-chain checks. `just semver-check` remains non-gating until
  version 0.1.0 provides a registry baseline.
- When changing optional-source conversions, also check the nightly shapes directly:
  `cargo check --all-targets --features nightly` and
  `cargo check --all-targets --no-default-features --features nightly`.

Reproduce a reported top-level failure before narrowing it, and rerun that same command after the
fix. For API removal or migration work, finish with an audit for deleted symbols and implicit
escape hatches rather than stopping at green tests.

## Feature model and compatibility

- `typed-css` is enabled by default and pulls in `leptos-css`. With default features disabled,
  the checked builders and typed re-exports are absent and `leptos-css` must not enter the graph.
- `nightly` forwards to Leptos and optional `leptos-css`. Reactive sources use explicit
  `move || signal.get()` closures on stable and nightly; do not add direct `Signal` conversion impls that can overlap
  when Leptos features are unified downstream.
- MSRV is Rust 1.89.0. Do not use newer standard-library APIs without updating `Cargo.toml`, the
  README, and CI together.
- Keep `leptos = 0.8.19` in lockstep with `leptos-css`; duplicate Leptos versions are a CI failure.
- Public items require doc comments. CI builds docs with `RUSTDOCFLAGS=-D warnings`.
- `clippy::pedantic` is denied. Prefer satisfying a lint to extending the allow-list.

## Module boundaries

- `style_entry.rs` owns a complete declaration and its static/reactive source.
  `StyleDeclaration` is either checked or explicitly unchecked. `StyleEntry::resolve` produces a
  borrowed-or-owned declaration snapshot.
- `style_list.rs` owns inline storage, reactivity caching, and `Normal` / `Fallback` priority
  metadata. It does not decide current property conflicts.
- `styles.rs` owns the public builder/chaining API, merge grouping, and the single
  `write_style_string` serialization path.
- `convert.rs` owns reactive and optional closure conversions for complete declarations and
  unchecked values. New source shapes belong there.
- `into_style.rs` owns Leptos `IntoStyle`, attribute integration, and static/reactive render state.
- `lib.rs` enumerates the intentionally small public surface and feature-gated re-exports.

Use the narrowest visibility that works (`pub(crate)` by default). Put validation and structural
invariants on the type that enforces them, and do not revalidate a guarantee after it crosses a
module boundary. Avoid generic `utils`, `helpers`, `common`, or file-size-driven module splits.

Doc comments describe their own layer. They may name direct collaborators or public shared
vocabulary, but should not explain themselves through an up-layer caller or leak a callee's private
representation.

## Rendering and merge invariants

- `Styles::merge(other)` makes `self` higher priority and assigns `other` a new fallback group.
  Existing fallback order in `other` remains ordered below that group.
- During serialization, resolve every present declaration once, choose the lowest group for each
  resolved property, then render all entries from the winning group in insertion order.
- Duplicates in one group are intentional and are not deduplicated. Debug builds warn only for
  duplicate statically-known normal-priority properties; reactive properties are unknown until
  rendering.
- Unchecked standard property names are trimmed and ASCII-lowercased. Unchecked custom properties
  beginning with `--` preserve case after trimming. Checked declarations require no normalization.
- Do not implement `Display` for `Styles` or `StyleEntry`; it could read signals outside a reactive
  context. `to_style_string` is the explicit materialization API.
- `Styles` owns the whole style attribute. Combining `style=styles` with `style:foo`, imperative DOM
  writes, or third-party mutations means the next managed update may overwrite those values.
- `is_reactive()` is cached on insertion. Static containers install no `RenderEffect`; reactive
  containers reuse their output buffers. Preserve transitions in both directions and reset/rebuild
  behavior.

## Tests

- Unit tests live beside their module. Keep fixtures at that module's abstraction level.
- `tests/typed_css.rs` is the checked-boundary integration suite. It must cover precise
  property/value pairing, optional sources, heterogeneous declarations, dynamic-property
  declarations, merge priority, and raw escape hatches.
- `testing/test-app` is a separate workspace used only by browser tests. Each scenario has one
  component under `testing/test-app/src/targets` and one matching assertion in
  `tests/ui_tests/test_styles.rs`.
- Browser coverage is the contract for SSR, hydration, reactive updates, style-attribute removal,
  static/reactive state transitions, external attribute reconciliation, merge fallback, and typed
  serialization. Do not rely only on native tests for changes to those paths.
- Keep the test app small and dedicated to this crate.
