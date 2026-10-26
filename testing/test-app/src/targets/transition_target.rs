use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders a target that toggles between a purely static `Styles` value and a
/// reactive one, depending on a signal.
///
/// Switches the rendering between a static and a reactive `Styles` in both
/// directions. The managed properties stay in sync, the previous state does
/// not leak, and reactive updates still arrive after each switch.
#[component]
pub fn TransitionTarget() -> impl IntoView {
    let (frozen, set_frozen) = signal(false);
    let (color, set_color) = signal(Some("seagreen"));

    let styles = move || {
        if frozen.get() {
            Styles::new()
                .with(UncheckedDeclaration::new("display", "grid"))
                .with(UncheckedDeclaration::new("color", "olive"))
        } else {
            Styles::new()
                .with(UncheckedDeclaration::new("display", "grid"))
                .with(move || {
                    color
                        .get()
                        .map(|color| UncheckedDeclaration::new("color", color))
                })
        }
    };

    view! {
        <section>
            <h2>"transition"</h2>
            <div id="transition-target" style=styles>"transition"</div>
            <button id="transition-freeze-static" on:click=move |_| set_frozen.set(true)>
                "Freeze static"
            </button>
            <button id="transition-enable-reactive" on:click=move |_| set_frozen.set(false)>
                "Enable reactive"
            </button>
            <button id="transition-clear" on:click=move |_| set_color.set(None)>
                "Clear color"
            </button>
            <button id="transition-set-color" on:click=move |_| set_color.set(Some("seagreen"))>
                "Set seagreen"
            </button>
        </section>
    }
}
