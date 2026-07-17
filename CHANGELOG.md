# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0]

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
- Stable and nightly support through closure-based reactive sources; the `nightly` feature forwards to Leptos and, when
  enabled, `leptos-css`.
- A minimum supported Rust version of 1.89.0.

[Unreleased]: https://github.com/lpotthast/leptos-styles/compare/v0.1.0...HEAD

[0.1.0]: https://github.com/lpotthast/leptos-styles/releases/tag/v0.1.0
