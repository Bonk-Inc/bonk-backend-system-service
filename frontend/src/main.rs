use yew::{Component, Context, Html, html};
use yew_router::prelude::*;

use crate::pages::{
    authenticate::Authenticate,
    home::Home
};

pub mod components;
pub mod models;
pub mod pages;
pub mod service;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/authenticate")]
    Authenticate,
    #[at("/")]
    App,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::App => html! { <Home/> },
        Route::Authenticate => html! { <Authenticate/> }
    }
}

pub struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {  }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}