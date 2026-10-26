use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders two style layers where the higher-priority layer can resolve to
/// `None`, allowing a lower-priority fallback to take over.
///
/// `Set fallback orange` sets the color of the higher layer again, and `Clear fallback
/// color` sets it to `None`. The browser test checks that the fallback `gray`
/// renders only while the higher layer has no color.
///
/// The fallback also declares the `padding` shorthand, and the higher layer
/// declares the `padding-top` longhand. The fallback renders first, so the
/// longhand wins.
#[component]
pub fn FallbackTarget() -> impl IntoView {
    let (color, set_color) = signal(Some("orange"));
    let styles = Styles::new()
        .with(move || {
            color
                .get()
                .map(|color| UncheckedDeclaration::new("color", color))
        })
        .with(UncheckedDeclaration::new("padding-top", "0px"))
        .with_fallback(
            Styles::new()
                .with(UncheckedDeclaration::new(" Color ", "gray"))
                .with(UncheckedDeclaration::new("padding", "16px")),
        );

    view! {
        <section>
            <h2>"fallback"</h2>
            <div id="fallback-target" style=styles>"fallback"</div>
            <button id="clear-fallback-color" on:click=move |_| set_color.set(None)>
                "Clear fallback color"
            </button>
            <button id="set-fallback-orange" on:click=move |_| set_color.set(Some("orange"))>
                "Set fallback orange"
            </button>
        </section>
    }
}
