use leptos::{html, prelude::*};
use leptos_styles::Styles;

/// Renders a reactive style and exposes a button that overwrites the rendered
/// `style` attribute directly through the DOM, bypassing Leptos.
///
/// `Styles` owns the entire `style` attribute, so a subsequent reactive update
/// must reconcile against the live attribute and restore our value rather than
/// silently leaving the externally-written CSS in place. The browser test
/// stomps on the attribute and then mutates the signal to verify reconciliation.
#[component]
pub fn ManagedTarget() -> impl IntoView {
    let target_ref = NodeRef::<html::Div>::new();
    let (color, set_color) = signal(Some("indigo".to_string()));

    let styles = Styles::builder()
        .with_unchecked("display", "grid")
        .with_optional_unchecked("color", move || color.get())
        .build();

    let mutate_target = move |_| {
        if let Some(el) = target_ref.get() {
            el.set_attribute("style", "color:black;background-color:gold;")
                .expect("managed target style mutation should succeed");
        }
    };

    view! {
        <section>
            <h2>"managed"</h2>
            <div id="managed-target" node_ref=target_ref style=styles>"managed"</div>
            <button id="managed-mutate" on:click=mutate_target>
                "Mutate managed style"
            </button>
            <button
                id="managed-set-same-color"
                on:click=move |_| set_color.set(Some("indigo".to_string()))
            >
                "Set indigo again"
            </button>
            <button
                id="managed-set-crimson"
                on:click=move |_| set_color.set(Some("crimson".to_string()))
            >
                "Set crimson"
            </button>
            <button
                id="managed-set-indigo"
                on:click=move |_| set_color.set(Some("indigo".to_string()))
            >
                "Set indigo"
            </button>
        </section>
    }
}
