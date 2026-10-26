use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::property::{HEIGHT, MARGIN, PADDING, RIGHT, WIDTH},
    css::{Auto, Margin, em, pct, rem, vh, vw},
};

/// Renders styles using non-`px` CSS lengths to validate length formatting
/// through the full `IntoStyle` pipeline.
///
/// Covers integer- and fractional-valued floats so any regression in numeric
/// formatting (trailing zeroes, locale dependence, etc.) shows up here.
#[component]
pub fn UnitsTarget() -> impl IntoView {
    let styles = Styles::new()
        .with(WIDTH.declare(pct(50)))
        .with(HEIGHT.declare(vh(40)))
        .with(PADDING.declare(em(1.5)))
        .with(MARGIN.declare(Margin::symmetric(rem(2), Auto)))
        .with(RIGHT.declare(vw(10)));

    view! {
        <section>
            <h2>"units"</h2>
            <div id="units-target" style=styles>"units"</div>
        </section>
    }
}
