use leptos::{html, prelude::*};
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders a reactive style and exposes a button that overwrites the rendered
/// `style` attribute directly through the DOM, bypassing Leptos.
///
/// `Styles` manages only the properties it declares. An update that changes no
/// declaration writes nothing. An update that changes a declaration writes
/// every declaration again while an unchanged one shows no value, which
/// restores the removed `display`. The externally written `background-color`
/// stays alone.
#[component]
pub fn ManagedTarget() -> impl IntoView {
    let target_ref = NodeRef::<html::Div>::new();
    let (color, set_color) = signal(Some("indigo"));
    let styles = Styles::new()
        .with(UncheckedDeclaration::new("display", "grid"))
        .with(move || {
            color
                .get()
                .map(|color| UncheckedDeclaration::new("color", color))
        });

    let mutate_target = move |_| {
        if let Some(el) = target_ref.get() {
            el.set_attribute("style", "color:black;background-color:gold;")
                .expect("managed target style mutation should succeed");
        }
    };

    view! {
        <section>
            <h2>"managed"</h2>
            <div id="managed-target" node_ref=target_ref style=styles>
                "managed"
            </div>
            <button id="managed-mutate" on:click=mutate_target>
                "Mutate managed style"
            </button>
            <button id="managed-set-same-color" on:click=move |_| set_color.set(Some("indigo"))>
                "Set indigo again"
            </button>
            <button id="managed-set-crimson" on:click=move |_| set_color.set(Some("crimson"))>
                "Set crimson"
            </button>
            <button id="managed-clear" on:click=move |_| set_color.set(None)>
                "Clear color"
            </button>
        </section>
    }
}
