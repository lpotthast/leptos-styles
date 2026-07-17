use leptos::prelude::*;
use leptos_styles::Styles;

/// Renders a target that toggles between a purely static `Styles` value and a
/// reactive one, depending on a signal.
///
/// Verifies that the `IntoStyle` state machine correctly transitions between
/// the `Static` and `Reactive` branches: switching directions should keep the
/// rendered `style` attribute in sync without leaking the previous branch's
/// state, and reactive updates after a transition should still propagate.
#[component]
pub fn TransitionTarget() -> impl IntoView {
    let (frozen, set_frozen) = signal(false);
    let (color, set_color) = signal(Some("seagreen".to_string()));

    let styles = move || {
        if frozen.get() {
            Styles::builder()
                .with_unchecked("display", "grid")
                .with_unchecked("color", "frozen")
                .build()
        } else {
            Styles::builder()
                .with_unchecked("display", "grid")
                .with_optional_unchecked("color", color)
                .build()
        }
    };

    view! {
        <section>
            <h2>"transition"</h2>
            <div id="transition-target" style=styles>"transition"</div>
            <button
                id="transition-freeze-static"
                on:click=move |_| set_frozen.set(true)
            >
                "Freeze static"
            </button>
            <button
                id="transition-enable-reactive"
                on:click=move |_| set_frozen.set(false)
            >
                "Enable reactive"
            </button>
            <button
                id="transition-clear"
                on:click=move |_| set_color.set(None)
            >
                "Clear color"
            </button>
            <button
                id="transition-set-color"
                on:click=move |_| set_color.set(Some("seagreen".to_string()))
            >
                "Set seagreen"
            </button>
        </section>
    }
}
