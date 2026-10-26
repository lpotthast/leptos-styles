use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders quoted values and a data URL, which the server must escape in the
/// `style` attribute and hydration must take over unchanged.
#[component]
pub fn QuotedTarget() -> impl IntoView {
    let styles = Styles::new()
        .with(UncheckedDeclaration::new(
            "font-family",
            "\"Open Sans\", sans-serif",
        ))
        .with(UncheckedDeclaration::new(
            "background-image",
            "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg'/>\")",
        ));

    view! {
        <section>
            <h2>"quoted"</h2>
            <div id="quoted-target" style=styles>"quoted"</div>
        </section>
    }
}
