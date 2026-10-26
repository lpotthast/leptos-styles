use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Passes `Styles` through several component layers in the Leptos renderer,
/// as the README describes.
///
/// Each layer extends the inherited `Styles` with its own entry, and the leaf
/// renders the result with `style=styles`. The browser test checks that the
/// declarations keep the order of the calls (`color`, `padding`, `margin`).
#[component]
pub fn DrillingTarget() -> impl IntoView {
    view! {
        <section>
            <h2>"drilling"</h2>
            <DrillingMid styles=UncheckedDeclaration::new("color", "purple") />
        </section>
    }
}

#[component]
fn DrillingMid(#[prop(into, optional)] styles: Styles) -> impl IntoView {
    view! { <DrillingLeaf styles=styles.with(UncheckedDeclaration::new("padding", "5px")) /> }
}

#[component]
fn DrillingLeaf(#[prop(into, optional)] styles: Styles) -> impl IntoView {
    view! {
        <div id="drilling-target" style=styles.with(UncheckedDeclaration::new("margin", "10px"))>
            "drilling"
        </div>
    }
}
