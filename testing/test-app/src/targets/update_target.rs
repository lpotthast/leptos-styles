use leptos::prelude::*;
use leptos_styles::{Styles, UncheckedDeclaration};

/// Renders four elements whose updates take the different paths of an update.
///
/// - A second `height` repeats a property before the first change, so the update writes every
///   declaration again.
/// - Two closures swap their properties after an unchanged `display`, so the update starts after
///   `display`.
/// - A rejected `padding-top` after an unchanged `padding` makes the update write every
///   declaration again, so that the `padding` keeps all four sides.
/// - A closure that returns a whole `Styles` switches a group of declarations at once.
#[component]
pub fn UpdateTarget() -> impl IntoView {
    let (enhanced, set_enhanced) = signal(false);
    let (swapped, set_swapped) = signal(false);
    let (top, set_top) = signal("1px");
    let (compact, set_compact) = signal(true);

    let repeated = Styles::new()
        .with(UncheckedDeclaration::new("height", "100px"))
        .with(move || {
            enhanced
                .get()
                .then(|| UncheckedDeclaration::new("height", "50px"))
        })
        .with(UncheckedDeclaration::new("color", "navy"));
    let reordered = Styles::new()
        .with(UncheckedDeclaration::new("display", "grid"))
        .with(move || {
            if swapped.get() {
                UncheckedDeclaration::new("gap", "4px")
            } else {
                UncheckedDeclaration::new("color", "navy")
            }
        })
        .with(move || {
            if swapped.get() {
                UncheckedDeclaration::new("color", "navy")
            } else {
                UncheckedDeclaration::new("gap", "4px")
            }
        });
    let group = Styles::new()
        .with(UncheckedDeclaration::new("display", "grid"))
        .with(move || {
            if compact.get() {
                Styles::new()
                    .with(UncheckedDeclaration::new("gap", "2px"))
                    .with(UncheckedDeclaration::new("padding", "4px"))
            } else {
                Styles::new().with(UncheckedDeclaration::new("gap", "8px"))
            }
        });
    let rejected = Styles::new()
        .with(UncheckedDeclaration::new("padding", "8px"))
        .with(move || UncheckedDeclaration::new("padding-top", top.get()));

    view! {
        <section>
            <h2>"update"</h2>
            <div id="update-repeated-target" style=repeated>"repeated"</div>
            <div id="update-reordered-target" style=reordered>"reordered"</div>
            <div id="update-rejected-target" style=rejected>"rejected"</div>
            <div id="update-group-target" style=group>"group"</div>
            <button id="update-toggle-enhanced" on:click=move |_| set_enhanced.update(|on| *on = !*on)>
                "Toggle the second height"
            </button>
            <button id="update-swap" on:click=move |_| set_swapped.set(true)>
                "Swap the properties"
            </button>
            <button id="update-expand" on:click=move |_| set_compact.set(false)>
                "Switch to the wide group"
            </button>
            <button id="update-reject" on:click=move |_| set_top.set("bogus")>
                "Set a rejected padding-top"
            </button>
        </section>
    }
}
