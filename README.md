# leptos-styles

[![crates.io](https://img.shields.io/crates/v/leptos-styles.svg)](https://crates.io/crates/leptos-styles)
[![docs.rs](https://docs.rs/leptos-styles/badge.svg)](https://docs.rs/leptos-styles)
[![CI](https://github.com/lpotthast/leptos-styles/actions/workflows/ci.yml/badge.svg)](https://github.com/lpotthast/leptos-styles/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/rust-1.89%2B-blue.svg)](https://www.rust-lang.org/)

`leptos-styles` collects the inline style of a [Leptos](https://leptos.dev) element in one value, `Styles`. A
component takes `Styles` as a prop, adds its own declarations, and passes the result on or renders it with
`style=styles`. Declarations can be static, optional, or reactive, and a whole layer of styles can serve as a
fallback.

The declarations come from [`typed-css`](https://crates.io/crates/typed-css), which checks each value against its
property at compile time. `leptos_styles::css` re-exports it, so the properties are in `leptos_styles::css::property`.

## Installation

```bash
cargo add leptos-styles
```

`leptos-styles` works with Leptos 0.8 and requires Rust 1.89 or newer.

## Example

```rust
use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::{NamedColor, property::{COLOR, DISPLAY, PADDING}, px},
};

#[component]
fn Card(#[prop(into, optional)] styles: Styles) -> impl IntoView {
    // The card adds its padding to the styles that its parent passed in.
    view! { <section style=styles.with(PADDING.declare(px(16)))>"Content"</section> }
}

#[component]
fn Page() -> impl IntoView {
    let (highlighted, _set_highlighted) = signal(false);
    view! {
        <Card styles=Styles::new()
            .with(DISPLAY.grid())
            .with(move || highlighted.get().then(|| COLOR.declare(NamedColor::Crimson)))
        />
    }
}
```

## Adding declarations

`Styles::with` accepts these kinds of values:

- A declaration, such as `PADDING.declare(px(16))` or `DISPLAY.grid()`.
- A whole `Styles`, such as the styles of a variant of a component. Its declarations join the declarations of the
  receiving `Styles`. Its fallback layers stay above the fallback layers of the receiving `Styles`.
- An `Option` of a declaration or of a `Styles`. `None` adds nothing.
- A closure that returns one of these values, such as `move || COLOR.declare(color.get())` or
  `move || variant_styles(variant.get())`. The closure runs again whenever a signal that it reads changes. It may even
  return a declaration of another property each time, or `None` while the declaration is absent.

`Styles::with_all` adds several values of one kind. A `StyleEntry` holds any of these kinds, so a list of
`StyleEntry` values can mix them. An iterator of declarations, `Styles`, entries, or `Option`s of them collects into
`Styles`.

A declaration, an `Option` of one, a `StyleEntry`, and an array of any of them convert into `Styles`, so
`<Card styles=PADDING.declare(px(8))/>` works with a `#[prop(into)]` prop. A closure does not convert directly, so a
caller wraps it, as in `<Card styles=StyleEntry::new(move || ...)/>` or
`<Card styles=Styles::new().with(move || ...)/>`.

With the `nightly` feature of Leptos, a signal can be called like a closure. `Styles::with` then accepts a signal of a
declaration, or of an `Option` of one, as a reactive declaration.

```rust
use leptos::prelude::*;
use leptos_styles::{Styles, css::{property::{BACKGROUND_COLOR, COLOR}, rgb}};

let owner = Owner::new();
owner.with(|| {
    let (color, set_color) = signal(rgb(255, 0, 0));
    let styles = Styles::new()
        .with(move || COLOR.declare(color.get()))
        .with(BACKGROUND_COLOR.declare(rgb(255, 255, 255)));

    assert_eq!(styles.to_style_string(), "color:rgb(255 0 0);background-color:rgb(255 255 255);");
    set_color.set(rgb(0, 0, 255));
    assert_eq!(styles.to_style_string(), "color:rgb(0 0 255);background-color:rgb(255 255 255);");
});
```

## Fallback layers

`Styles::with_fallback` adds another `Styles` as a layer of lower priority. A declaration of the fallback layer
renders only while no declaration of a higher layer has the same property. This is useful for the default styles of a
component, which the caller can override property by property:

```rust
use leptos_styles::{Styles, css::{NamedColor, property::{COLOR, PADDING}, px}};

let defaults = Styles::new()
    .with(COLOR.declare(NamedColor::Blue))
    .with(PADDING.declare(px(16)));
let caller = Styles::new().with(COLOR.declare(NamedColor::Red));

assert_eq!(caller.with_fallback(defaults).to_style_string(), "padding:16px;color:red;");
```

Layers of lower priority render first. A shorthand in the defaults, such as `padding`, therefore never overrides a
longhand from the caller, such as `padding-top`.

The same property may appear more than once within one layer. Those declarations render in the order they were added,
so a later one can enhance an earlier one, as in `height:100vh;height:100dvh;`.

## Rendering

`Styles` renders with `style=styles`. An `UncheckedDeclaration` and a `StyleEntry` also render on their own, and they
behave like a `Styles` that holds only them.

- During server-side rendering, it writes all current declarations into the `style` attribute.
- During hydration, it takes over the server's declarations and only changes the element if the client's
  declarations are missing from the attribute. A declaration that only the server rendered stays, so the server and
  the client must agree on the declarations, as for any Leptos view.
- In the browser, it sets each property through the element's style object. It only touches the properties that it
  declares, so it works alongside `style:` directives on the same element.
- When a declaration changes, `Styles` sets it and every declaration after it again, in order, so that shorthands and
  longhands keep their meaning. If the browser cannot confirm that an earlier declaration is still intact, `Styles`
  writes every declaration again, which matches server-side rendering. An update that changes no declaration writes
  nothing.
- A `style:` directive that sets a longhand of a shorthand in `Styles`, such as `style:padding-top` next to `padding`,
  loses its value when `Styles` sets that shorthand again.
- When no declaration is present and nothing else styles the element, `Styles` removes the `style` attribute.
- A `Styles` without reactive declarations renders once and never subscribes to signals.

## CSS that `typed-css` does not support

`typed-css` covers a selection of modern properties. For a property or value that it does not support, an
`UncheckedDeclaration` holds the declaration as text. Create one with `UncheckedDeclaration::new(property, value)`, or
with `UncheckedDeclaration::try_new` for text that is only known at runtime. `Styles::parse` parses a list of
declarations, such as CSS copied from elsewhere:

```rust
use leptos_styles::{Styles, UncheckedDeclaration};

let styles = Styles::new()
    .with(UncheckedDeclaration::new("container-type", "inline-size"))
    .with(UncheckedDeclaration::new("background-image", "url(data:image/png;base64,AAAA)"));
assert_eq!(
    styles.to_style_string(),
    "container-type:inline-size;background-image:url(data:image/png;base64,AAAA);",
);

let parsed = Styles::parse("contain: layout; font-family: 'Open Sans'").unwrap();
assert_eq!(parsed.to_style_string(), "contain:layout;font-family:'Open Sans';");

// Text from users is checked, so it cannot add other declarations.
let error = UncheckedDeclaration::try_new("color", "red; position: fixed").unwrap_err();
assert_eq!(error.declaration(), "color:red; position: fixed");
assert!(UncheckedDeclaration::try_new("color", "red !important").is_err());
```

The checks keep the text inside one declaration, but they do not make that declaration harmless. If users choose the
property, they can still write `position: fixed`. Any `url()` in a value makes the browser request that URL. So
choose the property in code, and check values from users against what your application allows.

An unchecked declaration is not checked against any CSS grammar, so the browser may ignore a declaration with a typo.
The text is checked for structure instead, with the same token rules that browsers use. The property name must be an
identifier. The value must have balanced quotes, parentheses, brackets, and braces, and it must not contain a `;` or
`!` outside of strings, `url()`, and blocks. The documentation of `UncheckedDeclaration` lists every rule. The
property name is trimmed. A standard property name is also lowercased, while a custom property name keeps its case.

## Re-exports

`leptos_styles::css` is the `typed-css` crate, with its properties in `css::property`. Your crate does not need its
own dependency on `typed-css`, and the versions always match.

`leptos-styles` enables the `leptos` feature of `typed-css`, so a checked declaration also renders on its own, as in
`style=PADDING.declare(px(16))`.

## Related crates

- [`typed-css`](https://github.com/lpotthast/typed-css) provides the checked values, properties, and declarations.
- [`leptos-classes`](https://github.com/lpotthast/leptos-classes) does the same for the `class` attribute.
- [`leptos-style`](https://github.com/RustForWeb/leptos-utils/tree/main/packages/leptos-style), with a singular
  `style`, is a smaller crate for passing style strings through components.

## Testing

`just verify` changes no files. It runs the formatting check, the manifest sort check, the native and WASM checks,
Clippy, the tests, the browser test, and the documentation build. The browser test starts a small
Leptos app and checks server-side rendering, hydration, reactive updates, fallback layers, and the coexistence with
`style:` directives in Chrome.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option. Contributions submitted for inclusion in the work are dual-licensed under the same terms.
