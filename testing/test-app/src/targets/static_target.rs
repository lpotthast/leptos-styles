use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders styles built statically from the [`Styles::builder`] API.
///
/// Mixes a typo'd whitespace-padded property name with a clean one to confirm
/// that property normalization runs end-to-end through Leptos rendering.
#[component]
pub fn StaticTarget() -> impl IntoView {
    let styles = Styles::builder()
        .with_unchecked(" Display ", "grid")
        .with_unchecked("color", "red")
        .build();

    view! {
        <section>
            <h2>"static"</h2>
            <div id="static-target" style=styles>"static"</div>
        </section>
    }
}
