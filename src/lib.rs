#![feature(lazy_cell)]

use serde::{Serialize,Deserialize};

use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
pub mod fallback;
pub mod error_template;
pub mod sidebar;

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum BackgroundImage{
    Homepage,
    None,
}
impl BackgroundImage{
    /// Will output a line of css "background-image: url(...);"
    pub fn to_css(&self) -> &'static str {
        match &self {
            Self::Homepage => "background-image: url(/backgrounds/homepage.png)",
            Self::None => {leptos::logging::error!("None");""},

        }
       }
}
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct RwBackgroundImage(RwSignal<BackgroundImage>);
#[component]
pub fn App() -> impl IntoView {
provide_meta_context();
    provide_context(RwBackgroundImage(create_rw_signal(BackgroundImage::Homepage)));

    view! {
        <Stylesheet id="leptos" href="/pkg/abysschart.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="AbyssChart"/>
        <Router>
            <main>
            <SetBody/>
                <Routes>
                    <Route path="" view=move || view!{
                        <button class="h-20 w-20 bg-white" on:click=move |_| panic!("")/>
                        <sidebar::SideBar/>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn SetBody() -> impl IntoView {
    let bg = expect_context::<RwBackgroundImage>();
    let body_style = move || format!("{};background-repeat: repeat;",bg.0().to_css());
    view!{
        <Body attr:style=body_style()/>
    }
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