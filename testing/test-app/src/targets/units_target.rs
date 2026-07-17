use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::{
        LengthPercentageAuto, Margin, NonNegativeLengthPercentage, Padding, Size, em, pct, rem, vh,
        vw,
    },
    property::{HeightProperty, MarginProperty, PaddingProperty, RightProperty, WidthProperty},
};

/// Renders styles using non-`px` CSS lengths to validate `CssLength` formatting
/// through the full `IntoStyle` pipeline.
///
/// Covers integer- and fractional-valued floats so any regression in numeric
/// formatting (trailing zeroes, locale dependence, etc.) shows up here.
#[component]
pub fn UnitsTarget() -> impl IntoView {
    let styles = Styles::builder()
        .with(WidthProperty.declare(Size::from(NonNegativeLengthPercentage::new(pct(50)))))
        .with(HeightProperty.declare(Size::from(NonNegativeLengthPercentage::new(vh(40)))))
        .with(PaddingProperty.declare(Padding::all(em(1.5))))
        .with(MarginProperty.declare(Margin::All(LengthPercentageAuto::from(rem(2)))))
        .with(RightProperty.declare(LengthPercentageAuto::from(vw(10))))
        .build();

    view! {
        <section>
            <h2>"units"</h2>
            <div id="units-target" style=styles>"units"</div>
        </section>
    }
}
