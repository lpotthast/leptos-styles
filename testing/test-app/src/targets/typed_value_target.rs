use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::{CssColor, CssColorName, NonNegativeLengthPercentage, Size, px},
    property::{ColorProperty, WidthProperty},
};

/// Renders a target driven by checked property selectors and exact value grammars.
///
/// The unit tests cover serialization of typed values; this target verifies
/// the same path through Leptos' `IntoStyle` rendering and reactive updates.
#[component]
pub fn TypedValueTarget() -> impl IntoView {
    let (color, set_color) = signal(Some(CssColor::Named(CssColorName::Red)));

    let styles = Styles::builder()
        .with(WidthProperty.declare(Size::from(NonNegativeLengthPercentage::new(px(120)))))
        .with_reactive(move || {
            ColorProperty.declare(
                color
                    .get()
                    .unwrap_or(CssColor::Named(CssColorName::Transparent)),
            )
        })
        .build();

    view! {
        <section>
            <h2>"typed value"</h2>
            <div id="typed-value-target" style=styles>"typed"</div>
            <button
                id="set-typed-blue"
                on:click=move |_| set_color.set(Some(CssColor::Named(CssColorName::Blue)))
            >
                "Set typed blue"
            </button>
            <button
                id="set-typed-transparent"
                on:click=move |_| set_color.set(Some(CssColor::Named(CssColorName::Transparent)))
            >
                "Set typed transparent"
            </button>
        </section>
    }
}
