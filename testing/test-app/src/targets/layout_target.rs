use leptos::prelude::*;
use leptos_styles::{
    Styles,
    css::property::{DISPLAY, GAP, GRID_TEMPLATE_COLUMNS, JUSTIFY_CONTENT, TRANSFORM},
    css::{GridRepeat, GridTemplate, TrackSize, TransformFunction, deg, fr, px, rem},
};

/// Renders the layout grammars (display, grid, alignment, and transforms).
///
/// The browser parses every declaration, so a serialization the browser rejects
/// would be missing from the asserted style.
#[component]
pub fn LayoutTarget() -> impl IntoView {
    let styles = Styles::new()
        .with(DISPLAY.grid())
        .with(
            GRID_TEMPLATE_COLUMNS.declare(GridTemplate::tracks([GridRepeat::auto_fill([
                TrackSize::minmax(px(120), fr(1)),
            ])])),
        )
        .with(GAP.declare(rem(1)))
        .with(JUSTIFY_CONTENT.center())
        .with(TRANSFORM.declare(TransformFunction::rotate(deg(3))));

    view! {
        <section>
            <h2>"layout"</h2>
            <div id="layout-target" style=styles>"layout"</div>
        </section>
    }
}
