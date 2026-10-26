use leptos::prelude::*;

use crate::targets::{
    CalcTarget, CheckedDeclarationTarget, CoexistTarget, DrillingTarget, EmptyTarget,
    FallbackTarget, HydrateTarget, LayoutTarget, LifecycleTarget, ManagedTarget, NormalizedTarget,
    OverlapTarget, ParseTarget, QuotedTarget, ReactiveTarget, ShorthandTarget, SingleTarget,
    StaticTarget, TransitionTarget, TypedValueTarget, UnitsTarget, UpdateTarget, VarTarget,
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
            <FallbackTarget />
            <TypedValueTarget />
            <CheckedDeclarationTarget />
            <DrillingTarget />
            <TransitionTarget />
            <ManagedTarget />
            <EmptyTarget />
            <VarTarget />
            <CalcTarget />
            <UnitsTarget />
            <ParseTarget />
            <LayoutTarget />
            <CoexistTarget />
            <LifecycleTarget />
            <QuotedTarget />
            <OverlapTarget />
            <UpdateTarget />
            <ShorthandTarget />
            <SingleTarget />
        </main>
    }
}
