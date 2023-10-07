use yew::{Component, Context, Html, html};
use yew_router::prelude::*;

use crate::pages::authenticate::Authenticate;

pub mod models;
pub mod pages;
pub mod service;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/authenticate")]
    Athenticate,
    #[at("/")]
    Home,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Test Test"}</h1> },
        Route::Athenticate => html! { <Authenticate/> }
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