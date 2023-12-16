use babs::models::{Score, Game};
use yew::{html, Component, Context, Html, Properties, classes, Callback};

use crate::layouts::main_layout::MainLayout;

pub struct ScoreForm {
    score: Option<Score>,
    games: Vec<Game>
}

#[derive(Clone, PartialEq, Properties)]
pub struct ScoresFormProps {
    #[prop_or_default]
    pub score_id: Option<String>,
}

pub enum Msg {
    FetchScore(String),
    FetchGames
}

impl Component for ScoreForm {
    type Message = Msg;
    type Properties = ScoresFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchGames);
        
        if let Some(score_id) = &ctx.props().score_id {
            ctx.link().send_message(Msg::FetchScore(score_id.clone()))
        }

        ScoreForm { score: None, games: vec![] }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MainLayout>
                <p>{"test"}</p>
            </MainLayout>
        }
    }
}