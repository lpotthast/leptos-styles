use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders a `Styles` constructed from a raw CSS declaration string via
/// [`Styles::parse_css`].
///
/// Verifies the string-parsing escape hatch survives the SSR/hydrate round
/// trip: the same parsed value is produced on the server and on the client,
/// and the resulting style attribute matches end-to-end.
#[component]
pub fn ParseCssTarget() -> impl IntoView {
    let styles = Styles::parse_css("color: rebeccapurple; padding: 8px;")
        .expect("hard-coded CSS literal should parse");

    view! {
        <section>
            <h2>"parse_css"</h2>
            <div id="parse-css-target" style=styles>"parse_css"</div>
        </section>
    }
}
