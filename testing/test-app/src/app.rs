use leptos::prelude::*;
use leptos_styles::Styles;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (color, set_color) = signal(Some("tomato".to_string()));
    let (merge_color, set_merge_color) = signal(Some("orange".to_string()));
    let hydration_color = if cfg!(feature = "ssr") {
        "stale"
    } else {
        "fresh"
    };
    let normalized_styles = Styles::builder()
        .with(" Touch-Action ", "none")
        .with(" --ThemeAccent ", "tomato")
        .build();
    let merged_fallback_styles = Styles::builder()
        .with_optional("color", merge_color)
        .build()
        .merge(Styles::new().add(" Color ", "gray"));

    view! {
        <main id="app">
            <h1>"leptos-styles test app"</h1>

            <div
                id="static-target"
                style=Styles::builder()
                    .with(" Display ", "grid")
                    .with("color", "red")
                    .build()
            >
                "static"
            </div>

            <div
                id="hydrate-target"
                style=Styles::builder()
                    .with("display", "grid")
                    .with("color", hydration_color)
                    .build()
            >
                "hydrate"
            </div>

            <div
                id="reactive-target"
                style=Styles::builder()
                    .with("display", "grid")
                    .with_optional("color", color)
                    .build()
            >
                "reactive"
            </div>

            <button
                id="set-royalblue"
                on:click=move |_| set_color.set(Some("royalblue".to_string()))
            >
                "Set royalblue"
            </button>
            <button id="clear-color" on:click=move |_| set_color.set(None)>
                "Clear color"
            </button>

            <div id="normalized-target" style=normalized_styles>
                "normalized"
            </div>

            <div id="merge-fallback-target" style=merged_fallback_styles>
                "merge fallback"
            </div>
            <button id="clear-merge-color" on:click=move |_| set_merge_color.set(None)>
                "Clear merge color"
            </button>
            <button
                id="set-merge-orange"
                on:click=move |_| set_merge_color.set(Some("orange".to_string()))
            >
                "Set merge orange"
            </button>
        </main>
    }
}
