# leptos-styles

`leptos-styles` is a small utility crate for passing inline styles through Leptos component layers without flattening them into a plain `String` too early.

It is designed for component props like `#[prop(into, optional)] styles: Styles`, where intermediate components can keep extending the style list and the final element can still render it reactively with `style=styles`.

## Features

- Static and optional style entries in one container
- Reactive updates through Leptos signals
- Builder and chaining APIs
- `IntoStyle` integration for `style=styles`
- Typed CSS values for dimensions, colors, angles, times, and expressions
- Small inline storage for common style counts

## Example

```rust
use leptos::prelude::*;
use leptos_styles::{
    Style::*,
    Styles,
    css::{CssValue, pct, px},
};

#[component]
fn Panel(
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <section style=styles.add(Padding, px(16))>
            "Content"
        </section>
    }
}

#[component]
fn Demo() -> impl IntoView {
    let (accent, _) = signal(Some(CssValue::from("tomato")));
    let (offset, _) = signal(Some(CssValue::from(px(24))));

    view! {
        <Panel styles=Styles::builder()
            .with(Display, "grid")
            .with(Width, pct(100))
            .with_optional(Color, accent)
            .with_optional(Top, offset)
            .build()
        />
    }
}
```

## API Overview

- `Styles::new()` creates an empty container.
- `Styles::add(property, value)` appends one style entry and returns the updated value.
- `Styles::add_optional(property, value)` appends a style that can resolve to `None`.
- `Styles::add_entry(...)` and `Styles::add_all(...)` work with prebuilt `StyleEntry` values.
- `Styles::builder()` offers the same operations with `with(...)`, `with_optional(...)`, `with_entry(...)`, and `with_all(...)`.
- `Styles::merge(other)` combines lower-priority fallback styles into higher-priority styles. If the higher-priority value for a property resolves to `None`, the lower-priority value can render as a fallback.
- `Styles::to_style_string()` materializes the currently active styles into a `String`.

## Supported Inputs

`StyleEntry` and `Styles` support common conversions:

- `("color", "red")`
- `(Style::Width, px(320))`
- `("background-color", Some("tomato"))`
- `(Style::Top, Signal<Option<CssValue>>)`
- `(Style::Opacity, move || Some(CssValue::from("0.8")))`
- arrays of the forms above for `Styles`

## Rendering Semantics

`Styles` implements Leptos' `IntoStyle` trait as a full `style="..."` attribute value.

- On SSR, it serializes to a semicolon-separated style string.
- On hydration and client-side rendering, it updates the element reactively when tracked signals change.
- `None` values are omitted from the rendered style attribute.
- Standard CSS property names are normalized by trimming whitespace and lowercasing ASCII letters.
- CSS custom properties beginning with `--` preserve their original case.
- CSS `var()` helpers accept custom property names with or without a leading `--` and normalize them before rendering.
- `CssDimension::Auto` is not valid in dimension arithmetic and will panic if used with `+`, `-`, or unary `-`; use explicit string values when you need non-arithmetic CSS expressions involving `auto`.

`Styles` owns the complete `style` attribute for an element. Prefer `style=styles` on its own instead of mixing it with separate `style:*` directives on the same element.

If you need the final value outside the renderer path, use `to_style_string()`. For actual elements, prefer `style=styles`.

## Testing

The DOM lifecycle coverage for `leptos-styles` is exercised through a real browser integration test.
From the repository root, run:

```bash
cargo test --test browser_test -- --nocapture
```

This starts a dedicated crate-local Leptos test frontend with `cargo leptos serve`, then drives it through Chrome via
`chrome-for-testing-manager` and `thirtyfour`.
