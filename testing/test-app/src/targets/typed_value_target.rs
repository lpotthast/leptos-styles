use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::property::{COLOR, WIDTH},
    css::{NamedColor, px},
};

/// Renders a target with checked properties and typed values.
///
/// The unit tests cover the serialization of typed values. This target checks
/// the same values through Leptos' `IntoStyle` rendering and reactive updates.
#[component]
pub fn TypedValueTarget() -> impl IntoView {
    let (color, set_color) = signal(NamedColor::Red);

    let styles = Styles::new()
        .with(WIDTH.declare(px(120)))
        .with(move || COLOR.declare(color.get()));

    view! {
        <section>
            <h2>"typed value"</h2>
            <div id="typed-value-target" style=styles>"typed"</div>
            <button id="set-typed-blue" on:click=move |_| set_color.set(NamedColor::Blue)>
                "Set typed blue"
            </button>
            <button
                id="set-typed-transparent"
                on:click=move |_| set_color.set(NamedColor::Transparent)
            >
                "Set typed transparent"
            </button>
        </section>
    }
}
