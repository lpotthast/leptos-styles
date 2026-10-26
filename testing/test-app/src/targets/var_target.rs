use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::property::{BACKGROUND_COLOR, COLOR},
    css::{Color, CustomProperty, NamedColor, custom_property, var, var_or},
};

custom_property!(ACCENT_COLOR: Color = "--AccentColor");

/// Renders styles built with checked custom properties and `var()`.
///
/// Defines a CSS custom property on the same element, references it via `var()`
/// in one declaration, and exercises the fallback branch with an undefined
/// custom property in another.
#[component]
pub fn VarTarget() -> impl IntoView {
    let missing_color = CustomProperty::<Color>::new("--MissingColor");
    let styles = Styles::new()
        .with(ACCENT_COLOR.declare(NamedColor::Fuchsia))
        .with(COLOR.declare(var(&ACCENT_COLOR)))
        .with(BACKGROUND_COLOR.declare(var_or(&missing_color, NamedColor::CurrentColor)));

    view! {
        <section>
            <h2>"var"</h2>
            <div id="var-target" style=styles>"var"</div>
        </section>
    }
}
