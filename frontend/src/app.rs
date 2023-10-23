use yew::{Component, html, Html, Context};
use yew_router::{Routable, Switch};

use crate::{
    layouts::main_layout::MainLayout, 
    pages::app::home::Home
};

pub struct AppBase;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/app/home")]
    Home,
    #[at("/app/games")]
    Games,
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html!(<Home />),
        AppRoute::Games => html! { <h1>{"Je Moeder"} </h1> },
    }
}

impl Component for AppBase {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AppBase { }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <MainLayout>
                <Switch<AppRoute> render={switch} />
            </MainLayout>
        }
    }
}