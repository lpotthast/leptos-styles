use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders a fully static `Styles` value.
///
/// Mixes a whitespace-padded, mixed-case property name with a clean one to
/// confirm that property normalization runs end-to-end through Leptos rendering.
#[component]
pub fn StaticTarget() -> impl IntoView {
    let styles = Styles::new()
        .with(UncheckedDeclaration::new(" Display ", "grid"))
        .with(UncheckedDeclaration::new("color", "red"));

    view! {
        <section>
            <h2>"static"</h2>
            <div id="static-target" style=styles>"static"</div>
        </section>
    }
}
