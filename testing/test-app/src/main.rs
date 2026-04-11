#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use leptos_styles_test_app::app::*;
    use tracing_subscriber::{prelude::*, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_target(false),
        )
        .init();

    let conf = get_configuration(None).expect("leptos configuration should load");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("test app listener should bind");
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("test app server should start");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
