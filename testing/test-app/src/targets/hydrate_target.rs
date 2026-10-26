use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders one value during SSR and another during hydration.
///
/// The browser test sees the hydrated `green` value, proving that
/// [`leptos_styles::Styles`] reconciles a server/client mismatch during
/// hydration rather than being frozen at SSR.
#[component]
pub fn HydrateTarget() -> impl IntoView {
    let color = if cfg!(feature = "ssr") {
        "red"
    } else {
        "green"
    };
    let styles = Styles::new()
        .with(UncheckedDeclaration::new("display", "grid"))
        .with(UncheckedDeclaration::new("color", color));

    view! {
        <section>
            <h2>"hydrate"</h2>
            <div id="hydrate-target" style=styles>"hydrate"</div>
        </section>
    }
}
