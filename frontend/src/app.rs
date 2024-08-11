use yew::{html, Component, Context, Html};
use yew_router::{Routable, Switch};

use crate::{
    layouts::main_layout::MainLayout,
    pages::app::{
        game::Game, home::Home, levels::Levels, score_form::ScoreForm, scores::Scores,
        settings::Settings,
    },
};

pub struct AppBase;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/app/home")]
    Home,
    #[at("/app/game/:id")]
    Game { id: String },
    #[at("/app/game/:game_id/level")]
    Levels { game_id: String },
    #[at("/app/game/:game_id/score")]
    Scores { game_id: String },
    #[at("/app/game/:game_id/settings")]
    Settings { game_id: String },
    #[at("/app/score/add")]
    ScoreAdd,
    #[at("/app/score/:score_id")]
    ScoreEdit { score_id: String },
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html!(<Home />),
        AppRoute::Game { id } => html!(<Game id={id} />),
        AppRoute::Levels { game_id } => html!(<Levels game_id={game_id} />),
        AppRoute::Scores { game_id } => html!(<Scores game_id={game_id} />),
        AppRoute::Settings { game_id } => html!(<Settings game_id={game_id} />),
        AppRoute::ScoreAdd => html!(<ScoreForm />),
        AppRoute::ScoreEdit { score_id } => html!(<ScoreForm score_id={score_id} />),
    }
}

impl Component for AppBase {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AppBase {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <MainLayout>
                <Switch<AppRoute> render={switch} />
            </MainLayout>
        }
    }
}
