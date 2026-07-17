use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders one value during SSR and another during hydration.
///
/// The browser test sees the hydrated `"fresh"` value, proving the style
/// attribute is rewritten by the client-side runtime — i.e. that
/// [`leptos_styles::Styles`] participates in hydration rather than being
/// frozen at SSR.
#[component]
pub fn HydrateTarget() -> impl IntoView {
    let color = if cfg!(feature = "ssr") {
        "stale"
    } else {
        "fresh"
    };
    let styles = Styles::builder()
        .with_unchecked("display", "grid")
        .with_unchecked("color", color)
        .build();

    view! {
        <section>
            <h2>"hydrate"</h2>
            <div id="hydrate-target" style=styles>"hydrate"</div>
        </section>
    }
}
