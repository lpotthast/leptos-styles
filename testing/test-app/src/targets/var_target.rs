use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::{CssColor, CssColorName, CssCustomProperty, css_custom_property, var},
    property::{BackgroundColorProperty, ColorProperty},
};

css_custom_property!(ACCENT_COLOR: CssColor = "--AccentColor");

/// Renders styles built with checked custom properties and `css::var`.
///
/// Defines a CSS custom property on the same element, references it via `var()`
/// in one declaration, and exercises the fallback branch with an undefined
/// custom property in another. The browser test asserts the exact serialized
/// `var(...)` syntax to catch regressions in the value writer.
#[component]
pub fn VarTarget() -> impl IntoView {
    let missing_color = CssCustomProperty::<CssColor>::new("--MissingColor");
    let styles = Styles::builder()
        .with(ACCENT_COLOR, CssColor::Named(CssColorName::Fuchsia))
        .with(
            ColorProperty,
            var(&ACCENT_COLOR, CssColor::Named(CssColorName::Black)),
        )
        .with(
            BackgroundColorProperty,
            var(&missing_color, CssColor::Named(CssColorName::CurrentColor)),
        )
        .build();

    view! {
        <section>
            <h2>"var"</h2>
            <div id="var-target" style=styles>"var"</div>
        </section>
    }
}
