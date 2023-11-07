#[cfg(feature = "ssr")]
mod ssr_imports {
    use abysschart::windows::CharacterData;
    pub use axum::{routing::get, Router};
    pub use abysschart::fallback::file_and_error_handler;
    pub use leptos::*;
    pub use leptos_axum::{generate_route_list, LeptosRoutes};
    pub use hmac::digest::KeyInit;

}




#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use ssr_imports::*;
    use abysschart::App;
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/favicon.ico", get(file_and_error_handler))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .layer(axum::Extension(abysschart::server_state::ServerState::new(
            std::sync::Arc::new(
                hmac::Hmac::new_from_slice((&*abysschart::env_vars::JWT_SECRET).as_ref())
                    .expect("Expecting valid Hmac<Sha256> from slice."),
            )
        )))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    logging::log!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// client-only stuff for Trunk
#[cfg(not(feature = "ssr"))]
pub fn main() {
    use abysschart::*;
    use leptos::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {  <App/> }
    });
}