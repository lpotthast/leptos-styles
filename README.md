# leptos-styles

[![crates.io](https://img.shields.io/crates/v/leptos-styles.svg)](https://crates.io/crates/leptos-styles)
[![docs.rs](https://docs.rs/leptos-styles/badge.svg)](https://docs.rs/leptos-styles)
[![CI](https://github.com/lpotthast/leptos-styles/actions/workflows/ci.yml/badge.svg)](https://github.com/lpotthast/leptos-styles/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/rust-1.89%2B-blue.svg)](https://www.rust-lang.org/)

`leptos-styles` is a prop-drillable, reactive inline-style container for Leptos. Components can extend a `Styles`
value through several layers, then render the accumulated declarations directly with `style=styles`.

The default API integrates [`leptos-css`](https://crates.io/crates/leptos-css) and preserves its central guarantee:
a checked property and its checked value grammar are never stored separately. A `Padding` can enter the container only
through `PaddingProperty`, while one shared value grammar such as `CssColor` can be selected for `ColorProperty` or
`BackgroundColorProperty` without losing which declaration was built.

## Features

- Declaration-first checked construction with `.with(Property.declare(value))` and
  `.add(Property.declare(value))`
- Owned, heterogeneous `CheckedDeclaration` values for building, storing, and prop drilling declarations
- Static, optional, and always-present reactive declarations in one container
- Priority-aware merging where a lower layer acts as a fallback for the current resolved property
- Direct Leptos `IntoStyle` integration for `style=styles`
- Explicit `_unchecked` methods and parsing escape hatches for unsupported CSS
- Small inline storage for common style counts

Cargo features:

- `typed-css` is enabled by default. It provides the checked declaration API and re-exports typed values and property
  selectors under `leptos_styles::css` and `leptos_styles::property`.
- `nightly` forwards nightly support to Leptos and to `leptos-css` when `typed-css` is enabled. Reactive sources use
  the same explicit closure syntax on stable and nightly.

## Installation

```bash
cargo add leptos-styles
```

To build only the unchecked string container without depending on `leptos-css`:

```bash
cargo add leptos-styles --no-default-features
```

`leptos-styles` is compatible with Leptos 0.8 and requires Rust 1.89 or newer.

## Checked quick start

Build styles in a caller, pass them through a component prop, and extend them in the receiving component:

```rust
#[cfg(feature = "typed-css")]
mod checked_example {
use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::{CssColor, CssColorName, Padding, px},
    property::{ColorProperty, PaddingProperty},
};

#[component]
fn Panel(
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <section style=styles.add(PaddingProperty.declare(Padding::all(px(16))))>
            "Content"
        </section>
    }
}

#[component]
fn Demo() -> impl IntoView {
    view! {
        <Panel styles=Styles::builder()
            .with(ColorProperty.declare(CssColor::Named(CssColorName::Fuchsia)))
            .build()
        />
    }
}
}
```

The selector names say which declaration is being built; the value types describe only the accepted value grammar:

```rust
#[cfg(feature = "typed-css")]
{
use leptos_styles::{
    Styles,
    css::rgb,
    property::{BackgroundColorProperty, ColorProperty},
};

let styles = Styles::builder()
    .with(ColorProperty.declare(rgb(255, 0, 0)))
    .with(BackgroundColorProperty.declare(rgb(255, 0, 0)))
    .build();

assert_eq!(
    styles.to_style_string(),
    "color:rgb(255, 0, 0);background-color:rgb(255, 0, 0);",
);
}
```

Always-present reactive values do not need to be wrapped in `Some`:

```rust
#[cfg(feature = "typed-css")]
{
use leptos::prelude::*;
use leptos_styles::{Styles, css::rgb, property::ColorProperty};

let owner = Owner::new();
owner.with(|| {
    let (color, set_color) = signal(rgb(255, 0, 0));
    let styles = Styles::builder()
        .with_reactive(move || ColorProperty.declare(color.get()))
        .build();

    assert_eq!(styles.to_style_string(), "color:rgb(255, 0, 0);");
    set_color.set(rgb(0, 0, 255));
    assert_eq!(styles.to_style_string(), "color:rgb(0, 0, 255);");
});
}
```

## API overview

The builder and chaining APIs mirror one another:

- `with(declaration)` / `add(declaration)` accept a prebuilt `CheckedDeclaration` or explicit `StyleEntry`.
- `with_optional(source)` / `add_optional(source)` accept an `Option` or a closure returning `Option`; a reactive
  source may change its complete declaration, including its property.
- `with_reactive(move || declaration)` / `add_reactive(move || declaration)` accept an always-present reactive
  declaration without requiring an artificial `Some(...)` wrapper. The closure form works on stable and nightly.
- Property selectors build checked declarations with `declare(value)` or `declare_global(keyword)`; `AllProperty`
  accepts its CSS-wide keyword through `declare(keyword)`.
- `with_declarations(iter)` / `add_declarations(iter)` add heterogeneous prebuilt declarations.
- `merge(other)` treats `other` as a lower-priority fallback layer.

Merge priority is resolved per property, so declarations absent from the higher-priority layer still fall through:

```rust
#[cfg(feature = "typed-css")]
{
use leptos_styles::{
    Styles,
    css::{CssColor, CssColorName, Padding, px},
    property::{ColorProperty, PaddingProperty},
};

let defaults = Styles::builder()
    .with(ColorProperty.declare(CssColor::Named(CssColorName::Blue)))
    .with(PaddingProperty.declare(Padding::all(px(16))))
    .build();
let local = Styles::builder()
    .with(ColorProperty.declare(CssColor::Named(CssColorName::Red)))
    .build();

assert_eq!(
    local.merge(defaults).to_style_string(),
    "color:red;padding:16px;",
);
}
```

Every reactive entry is resolved exactly once per serialization pass. The renderer snapshots the complete declaration
before calculating merge winners, so a source that changes from `color:red` to `background-color:red` cannot expose a
property from one evaluation and a value from another.

`Styles::to_style_string()` materializes the current declarations. For elements, prefer `style=styles`; this reuses
serialization buffers across reactive updates.

## Unchecked CSS escape hatches

Property coverage in `leptos-css` is intentionally incremental. Unsupported CSS remains available, but raw property
names and values must be visibly opted into:

```rust
use leptos_styles::{StyleEntry, Styles};

let styles = Styles::builder()
    .with_unchecked("display", "grid")
    .with_optional_unchecked("font-family", Some("system-ui"))
    .build()
    .add(StyleEntry::parse("contain: layout").unwrap());

assert_eq!(
    styles.to_style_string(),
    "display:grid;font-family:system-ui;contain:layout;",
);
```

`StyleEntry::parse(...)` parses one unchecked declaration, while `Styles::parse_css(...)`, `FromStr`, and `TryFrom`
parse a simple semicolon-separated declaration list. These are structural conveniences, not a full CSS parser or
validator. Add values that contain semicolons, such as data URLs, through the `_unchecked` builders instead. There are
deliberately no raw tuple conversions, and the `PropertyName` catalog is not an unchecked bridge into the checked API.
Public raw storage names are explicitly labeled `UncheckedPropertyName` and `UncheckedStyleValue`, and property-name
conversion goes through `IntoUncheckedPropertyName`.

## Rendering semantics

`Styles` implements Leptos' `IntoStyle` trait as a full `style="..."` attribute value.

- On SSR, it serializes to a semicolon-separated style string.
- On hydration, it reconciles against the element's current `style` attribute.
- On client-side rendering, tracked signals update the element reactively.
- Fully static containers avoid installing a reactive effect.
- `None` declarations are omitted. If every declaration resolves to `None`, the entire `style` attribute is removed.
- Standard unchecked property names are trimmed and ASCII-lowercased; unchecked custom properties beginning with `--`
  preserve their case.
- For each resolved property, the highest-priority merge layer wins. Entries within the same layer preserve insertion
  order, including intentional duplicates.

`Styles` owns the entire `style` attribute of the target element. A reactive update replaces the full value, so mixing
`style=styles` on one element with `style:foo=...`, imperative style mutations, or third-party attribute mutations can
overwrite the unmanaged value on the next update. `IntoStyle::reset()` removes the managed attribute and tears down
reactive subscriptions while leaving the state rebuildable.

## Related crates

- [`leptos-css`](https://github.com/lpotthast/leptos-css) defines the checked property selectors, value grammars, and
  owned `CheckedDeclaration` boundary used by the default API.
- [`leptos-classes`](https://github.com/lpotthast/leptos-classes) is the class-attribute counterpart to this crate.
- [`leptos-style`](https://github.com/RustForWeb/leptos-utils/tree/main/packages/leptos-style) (singular `style`) is a
  smaller string-based prop-drilling crate.

## Testing

Run the complete native, WASM, lint, documentation, and browser verification matrix with:

```bash
just verify
```

The browser test starts the crate-local Leptos frontend and exercises SSR, hydration, static/reactive transitions,
merge fallback behavior, and direct checked-declaration value updates, property replacement, custom properties, and
reset behavior in Chrome.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option. Contributions submitted for inclusion in the work are dual-licensed under the same terms.
