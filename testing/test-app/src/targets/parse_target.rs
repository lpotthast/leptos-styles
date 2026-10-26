use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders a `Styles` constructed from a raw CSS declaration string via
/// [`Styles::parse`].
///
/// Checks that parsed CSS survives server-side rendering and hydration. The
/// server and the client parse the same declarations, so hydration keeps the
/// style attribute.
#[component]
pub fn ParseTarget() -> impl IntoView {
    let styles = Styles::parse("color: rebeccapurple; padding: 8px;")
        .expect("hard-coded CSS literal should parse");

    view! {
        <section>
            <h2>"parse"</h2>
            <div id="parse-target" style=styles>"parse"</div>
        </section>
    }
}
