use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::{LengthPercentageCalculation, Size, pct, px, rem, vh},
    property::{HeightProperty, WidthProperty},
};

/// Renders grammar-checked CSS expression values.
///
/// The browser test asserts the exact serialized form so regressions in the
/// typed calculation writer are caught end-to-end.
#[component]
pub fn CalcTarget() -> impl IntoView {
    let styles = Styles::builder()
        .with(
            WidthProperty,
            Size::Calculation(LengthPercentageCalculation::new(pct(100) - px(20))),
        )
        .with(
            HeightProperty,
            Size::Calculation(LengthPercentageCalculation::new(vh(50) + rem(1))),
        )
        .build();

    view! {
        <section>
            <h2>"calc"</h2>
            <div id="calc-target" style=styles>"calc"</div>
        </section>
    }
}
