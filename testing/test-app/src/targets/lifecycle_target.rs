use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders a `Styles` whose closure switches between two properties, inside a
/// style closure that can remove the `Styles` from the element.
///
/// Switching the property must remove the old property. Hiding the `Styles`
/// resets the element, which removes every property and the empty attribute.
#[component]
pub fn LifecycleTarget() -> impl IntoView {
    let (shown, set_shown) = signal(true);
    let (background, set_background) = signal(false);
    let styles = Styles::new()
        .with(UncheckedDeclaration::new("display", "grid"))
        .with(move || {
            if background.get() {
                UncheckedDeclaration::new("background-color", "teal")
            } else {
                UncheckedDeclaration::new("color", "teal")
            }
        });

    view! {
        <section>
            <h2>"lifecycle"</h2>
            <div id="lifecycle-target" style=move || shown.get().then(|| styles.clone())>
                "lifecycle"
            </div>
            <button id="lifecycle-switch-property" on:click=move |_| set_background.set(true)>
                "Switch to background-color"
            </button>
            <button id="lifecycle-hide" on:click=move |_| set_shown.set(false)>
                "Hide styles"
            </button>
            <button id="lifecycle-show" on:click=move |_| set_shown.set(true)>
                "Show styles"
            </button>
        </section>
    }
}
