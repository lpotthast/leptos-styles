# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial standalone `leptos-styles` crate release for prop-drillable Leptos inline style handling.
- `Styles` builder API with `with`, `with_optional`, `with_entry`, and `with_all`.
- Chaining API with `add`, `add_optional`, `add_entry`, `add_all`, and `merge`.
- `StyleEntry` support for static, optional, and reactive style values.
- `IntoStyle` and attribute integration for rendering `style=styles` directly in Leptos views.
- Typed CSS values for lengths, dimensions, colors, angles, times, keywords, and expressions in [`css`].
- Large typed CSS property enum in [`Style`] plus the `TouchActionStyle` helper for spreadable Leptos style tuples.
- Debug-build duplicate property detection with `tracing` warnings.
- Crate README with examples, supported inputs, and rendering semantics.

### Changed

- The crate is now packaged as a single publishable library without the experimental `style!` proc-macro crate.
- Packaging metadata now includes a crate README and docs.rs target configuration for native and wasm builds.
- The `merge` implementation now satisfies the workspace clippy gate while preserving existing precedence behavior.
