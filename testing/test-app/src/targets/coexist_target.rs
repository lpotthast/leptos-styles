use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Combines `style=styles` with a `style:` directive on the same element.
///
/// `Styles` manages only the properties it declares, so its reactive updates
/// must not clear the directive's `opacity`.
#[component]
pub fn CoexistTarget() -> impl IntoView {
    let (color, set_color) = signal("navy");
    let styles = Styles::new().with(move || UncheckedDeclaration::new("color", color.get()));

    view! {
        <section>
            <h2>"coexist"</h2>
            <div id="coexist-target" style=styles style:opacity="0.5">
                "coexist"
            </div>
            <button id="coexist-set-maroon" on:click=move |_| set_color.set("maroon")>
                "Set maroon"
            </button>
        </section>
    }
}
