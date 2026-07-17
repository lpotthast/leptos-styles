# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0]

### Added

- Initial standalone `leptos-styles` crate for prop-drillable, reactive ownership of one complete
  inline `style` attribute in Leptos applications.
- Declaration-first checked builder and chaining APIs: `with` / `add` accept complete
  `CheckedDeclaration` values, `with_reactive` / `add_reactive` accept always-present declaration closures, and their
  optional variants accept `Option` or closures returning `Option`.
- A default `typed-css` feature with re-exports of `leptos-css` checked declarations, property
  selectors, value grammars, CSS custom-property support, and constructor helpers. Disabling
  default features leaves an unchecked string container and parsing escape hatches without a
  `leptos-css` dependency.
- Iterator APIs (`with_declarations` and `add_declarations`) for heterogeneous prebuilt
  `leptos_css::CheckedDeclaration` values. Checked entries retain the complete declaration so property and value
  cannot be mismatched after validation.
- Explicit `with_unchecked` / `add_unchecked` APIs and optional variants for unsupported raw CSS.
  Raw property names and values do not implicitly cross the checked boundary; their public supporting types are
  explicitly named `UncheckedPropertyName`, `UncheckedStyleValue`, and `IntoUncheckedPropertyName`.
- `StyleEntry` support for optional static and reactive complete declarations, including sources
  whose property and value can both change. Each present declaration is resolved once per
  serialization pass and that snapshot is used for both priority selection and output.
- Optional-source conversions for `Option` and closures. Reactive sources use `move || ...` directly on stable and
  nightly, avoiding feature-dependent signal conversion behavior. The `nightly` feature forwards to Leptos and
  optional `leptos-css`.
- `From` impls for building `Styles` from one complete declaration or an array of declarations.
- `Styles::is_reactive` for inspecting whether the container subscribes to any reactive signal,
  cached on insertion for O(1) lookup.
- `Styles::merge` with priority-aware fallback: conflicting entries from the lower-priority
  side render only when no present higher-priority declaration resolves to the same property.
  Chained fallback groups preserve their order, and duplicate declarations within one winning
  group remain in insertion order.
- `StyleEntry::parse` and `Styles::parse_css` for fail-soft parsing of raw CSS declaration
  strings, plus matching `FromStr`, `TryFrom<&str>`, and `TryFrom<String>` impls returning
  `ParseStyleEntryError`.
- Property-name normalization: trim whitespace, lowercase ASCII for standard properties; CSS
  custom properties (those starting with `--`) preserve case after trimming. The rule is
  enforced inside `StyleEntry` constructors so call sites cannot bypass it.
- `IntoStyle` and attribute integration so `style=styles` works in Leptos views: fully static
  style lists skip `RenderEffect` setup, reactive containers install a single render effect
  that reuses output buffers across updates, and rebuilds support transitions between static and
  reactive containers.
- Debug-build duplicate-property detection: pushing the same property name twice into a
  `Styles` emits a `tracing::warn!` with a forced backtrace. Duplicates are intentionally not
  deduplicated at runtime, matching browser CSS semantics where later declarations win.
- Crate README with examples, supported inputs, and rendering semantics. `docs.rs` builds
  against both `x86_64-unknown-linux-gnu` and `wasm32-unknown-unknown` targets.
- Browser integration test harness driven by
  [`leptos-browser-test`](https://crates.io/crates/leptos-browser-test) and `browser-test`,
  exercising SSR, hydration, reactive updates, attribute removal and reconciliation,
  static/reactive transitions, merge fallback, and typed serialization in Chrome.
- A checked-boundary integration suite covering precise property/value pairing, optional sources,
  always-present reactive values, heterogeneous and dynamic-property declarations, merge priority, sealed-property
  enforcement, direct DOM declaration lifecycles, and explicit raw escape hatches.
- MSRV: Rust 1.89.0.

[Unreleased]: https://github.com/lpotthast/leptos-styles/compare/v0.1.0...HEAD

[0.1.0]: https://github.com/lpotthast/leptos-styles/releases/tag/v0.1.0
