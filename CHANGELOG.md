# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Version 0.2.0 redesigns the public API. Every change is breaking unless it is listed under "Added".

### Changed

- The checked declarations come from `typed-css` 0.2, which continues `leptos-css` under a new name. Its property
  constants, such as `property::PADDING`, accept the common parts of their grammar directly. Each property also has a
  method for each keyword that it accepts on its own, such as `DISPLAY.grid()`. `typed-css` is a required
  dependency, so the `typed-css` feature and the build without default features are removed.
- `leptos_styles::css` is now the `typed-css` crate itself, so it contains every item of that crate, such as
  `css::property::PADDING` and `css::Declaration`. The re-exports `leptos_styles::leptos_css`, `CheckedDeclaration`,
  and the root `css_custom_property!` are removed. Use `css::custom_property!` instead.
- `Styles::with` accepts a declaration, a `Styles`, an `Option` of either, or a closure that returns one of them. It
  replaces `with_optional`, `with_reactive`, and the `add*` methods. `with_all` replaces `with_declarations`.
- `StylesBuilder` and `Styles::builder()` are removed, because `Styles` itself is built by chaining `with`.
- `Styles::merge(other)` is renamed to `Styles::with_fallback(other)`, which accepts any value that converts into
  `Styles`. Layers of lower priority render first, so a shorthand in a fallback no longer overrides a longhand of a
  higher layer.
- CSS text enters through an `UncheckedDeclaration`, which `UncheckedDeclaration::new` and
  `UncheckedDeclaration::try_new` create. The text is checked for structure, so it cannot add other declarations or
  raise its priority. The `*_unchecked` methods, `IntoOptionalUncheckedStyleValue`, `IntoUncheckedPropertyName`,
  `UncheckedPropertyName`, `UncheckedStyleValue`, `StyleEntry::always_unchecked`, and `StyleEntry::reactive_unchecked`
  are removed.
- `Styles::parse_css` is renamed to `Styles::parse`. It and `FromStr` only split declarations at a `;` outside of
  strings, functions, and `url()`, so data URLs and quoted text parse correctly. Errors are
  `InvalidUncheckedDeclaration`, which names the rejected declaration and the broken rule. `ParseStyleEntryError`,
  `StyleEntry::parse`, and the `TryFrom<&str>` and `TryFrom<String>` conversions are removed.
- In the browser, `style=styles` sets each declared property through the element's style object instead of
  replacing the whole `style` attribute. So `Styles` works alongside `style:` directives, and an update no longer reads
  the attribute back. An update that changes no declaration writes nothing. Otherwise, the update starts at the first
  changed declaration, so a `style:` directive keeps a longhand of an unchanged shorthand. If an unchanged
  declaration shows no value before or after the update, `Styles` writes every declaration again, so the result
  matches server-side rendering.
  Hydration only changes the element if the server's attribute does not contain the client's declarations.
- `Styles` no longer implements Leptos' `Attribute`, `NextAttribute`, and `IntoAttributeValue`. Use `style=styles`.
- `Styles::is_reactive`, `Styles::write_style_string`, the render state `StylesState`, and the markers of
  `IntoStyleEntry` are no longer public. Use `Styles::to_style_string` to read the declarations as text.
- `Styles` no longer creates signals in the current reactive owner, so a `Styles` does not depend on that owner.
  The signals that its closures read still belong to their own owners.
- `IntoOptionalStyleDeclaration` is replaced by `IntoStyleEntry`, and `StyleEntry::static_property_name` is removed.
- The `nightly` feature is removed, because it only enabled the `nightly` feature of Leptos. Cargo enables that
  feature for every crate that uses Leptos once any crate enables it. With it, `Styles::with` accepts a signal like a
  closure.
- The crate requires Leptos 0.8.22 and no longer depends on `reactive_graph` directly.

### Added

- `StyleEntry::new` creates an entry from any value that `Styles::with` accepts, so one list can mix declarations,
  options, and closures.
- `UncheckedDeclaration` implements `Display` as `property:value;`, and `Hash`.
- `UncheckedDeclaration` and `StyleEntry` implement Leptos' `IntoStyle`, so they render with `style=` on their own.
- `Option`s of declarations, `Styles`, and entries convert into `StyleEntry` and `Styles`.
- `Styles` implements `FromIterator` and `Extend` for declarations, `Styles`, entries, and `Option`s of them.
- `Styles::with` accepts a whole `Styles`, or a closure that returns one, which switches a group of declarations at
  once. The fallback layers of the added `Styles` stay above the fallback layers of the receiving `Styles`, no
  matter in which order the calls happen.
- `UncheckedDeclaration::try_new` accepts borrowed text, such as a `&String`.
- `InvalidUncheckedDeclaration::declaration` returns the rejected declaration, and
  `InvalidUncheckedDeclaration::reason` returns an `InvalidUncheckedReason`, so code can tell the broken rules apart.

### Removed

- The warning about duplicate properties in debug builds, and the `tracing` dependency. A property may appear more
  than once on purpose.

## [0.1.0] - 2026-07-17

### Added

- Initial standalone `leptos-styles` crate for prop-drillable, reactive ownership of one complete inline `style`
  attribute in Leptos applications.
- A declaration-first checked API backed by the default `typed-css` feature, with re-exports of `leptos-css` property
  selectors, value grammars, declarations, and constructor helpers. The builder and chaining APIs accept individual
  static, reactive, and optional declarations or heterogeneous iterators while retaining complete `CheckedDeclaration`
  values.
- Explicit unchecked builders and raw CSS parsers for unsupported properties. Standard property names are normalized
  while custom-property case is preserved. Disabling default features provides this unchecked-only API without a
  `leptos-css` dependency.
- `Styles::merge` for priority-aware fallback layers. Higher-priority declarations win per resolved property, while
  chained fallback groups and same-group insertion order are preserved.
- Leptos `IntoStyle` integration for `style=styles`, covering SSR, hydration, client-side reactive updates, attribute
  removal, and transitions between static and reactive containers.
- Stable and nightly support through closure-based reactive sources. The `nightly` feature forwards to Leptos and,
  when enabled, `leptos-css`.
- A minimum supported Rust version of 1.89.0.

[Unreleased]: https://github.com/lpotthast/leptos-styles/compare/v0.1.0...HEAD

[0.1.0]: https://github.com/lpotthast/leptos-styles/releases/tag/v0.1.0
