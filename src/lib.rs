#![feature(lazy_cell)]

use serde::{Serialize,Deserialize};

use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
pub mod fallback;
pub mod error_template;
pub mod sidebar;
pub mod body;
pub mod windows;
pub mod user_info;
pub mod client_state;
use client_state::ClientState;
#[cfg(feature="ssr")]
pub mod env_vars;
#[cfg(feature="ssr")]
pub mod backend_utils;
#[cfg(feature="ssr")]
pub mod server_state;
pub mod login;
pub mod user_msg;
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet id="leptos" href="/pkg/abysschart.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="AbyssChart"/>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|| view!{
                        <ContextIsland>
                        <Outlet/>
                        <user_msg::UserMsg/>
                        <body::SetBody/>
                        <sidebar::SideBar/>
                        <windows::WindowAggregator/>
                        </ContextIsland>
                    }>
                        <Route path="" view=|| view!{<HomePage/>}/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view!{
        <login::Login/>
    }
}

#[island]
pub fn ContextIsland(children:Children) -> impl IntoView {
    let rw_client_state = create_rw_signal(ClientState::default());
    provide_context(rw_client_state);
    children()
}



// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            #[cfg(debug_assertions)]
            console_error_panic_hook::set_once();
            leptos::leptos_dom::HydrationCtx::stop_hydrating();
        }
    }
}