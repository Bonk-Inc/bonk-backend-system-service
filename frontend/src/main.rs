use yew::{Component, Context, Html, html};
use yew_router::prelude::*;

use crate::{
    app::AppBase,
    pages::authenticate::Authenticate,
};

pub mod app;
pub mod components;
pub mod layouts;
pub mod models;
pub mod pages;
pub mod service;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Index,
    #[at("/authenticate")]
    Authenticate,
    #[at("/app/*")]
    App,
}

fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Index => html!(<Redirect<MainRoute> to={MainRoute::Authenticate} />),
        MainRoute::App => html!(<AppBase />),
        MainRoute::Authenticate => html!(<Authenticate/>)
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App { }
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