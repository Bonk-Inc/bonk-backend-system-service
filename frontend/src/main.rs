use providers::authetication_provider::AuthenticationProvider;
use yew::{Component, Context, Html, html};
use yew_router::prelude::*;

use crate::pages::{
    authenticate::Authenticate,
    home::Home
};

pub mod components;
pub mod models;
pub mod pages;
pub mod providers;
pub mod service;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/authenticate")]
    Authenticate,
    #[at("/")]
    App,
}

fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::App => html! {
            <AuthenticationProvider>
                <Home/>
            </AuthenticationProvider>
        },
        MainRoute::Authenticate => html! { <Authenticate/> }
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
                <Switch<MainRoute> render={switch} />
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}