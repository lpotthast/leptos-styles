use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders styles whose property names need normalization.
///
/// `Touch-Action` should be trimmed and lowercased to `touch-action`, while
/// `--ThemeAccent` (a CSS custom property) must keep its original casing.
#[component]
pub fn NormalizedTarget() -> impl IntoView {
    let styles = Styles::builder()
        .with_unchecked(" Touch-Action ", "none")
        .with_unchecked(" --ThemeAccent ", "tomato")
        .build();

    view! {
        <section>
            <h2>"normalized"</h2>
            <div id="normalized-target" style=styles>"normalized"</div>
        </section>
    }
}
