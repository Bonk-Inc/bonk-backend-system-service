use yew::{Component, html, Html, Context};
use yew_router::{Routable, Switch};

use crate::{
    layouts::main_layout::MainLayout, 
    pages::app::{home::Home, game::Game, scores::Scores}
};

pub struct AppBase;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/app/home")]
    Home,
    #[at("/app/game/:id")]
    Game { id: String },
    #[at("/app/game/:game_id/score")]
    Scores { game_id: String}
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html!(<Home />),
        AppRoute::Game { id } => html!(<Game id={id} />),
        AppRoute::Scores { game_id } => html!(<Scores game_id={game_id} />)
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