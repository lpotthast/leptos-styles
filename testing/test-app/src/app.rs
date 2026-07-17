use leptos::prelude::*;

use crate::targets::{
    CalcTarget, CheckedDeclarationTarget, DrillingTarget, EmptyTarget, HydrateTarget,
    ManagedTarget, MergeFallbackTarget, NormalizedTarget, ParseCssTarget, ReactiveTarget,
    StaticTarget, TransitionTarget, TypedValueTarget, UnitsTarget, VarTarget,
};

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
    view! {
        <main id="app">
            <h1>"leptos-styles test app"</h1>
            <StaticTarget />
            <HydrateTarget />
            <ReactiveTarget />
            <NormalizedTarget />
            <MergeFallbackTarget />
            <TypedValueTarget />
            <CheckedDeclarationTarget />
            <DrillingTarget />
            <TransitionTarget />
            <ManagedTarget />
            <EmptyTarget />
            <VarTarget />
            <CalcTarget />
            <UnitsTarget />
            <ParseCssTarget />
        </main>
    }
}
