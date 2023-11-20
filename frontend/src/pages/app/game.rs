use babs::{models::GameStats, respone::ResponseBody};
use uuid::Uuid;
use yew::{classes, html, Component, Context, Html, Properties};
use yew_router::{
    history::Location,
    scope_ext::{LocationHandle, RouterScopeExt}
};

use crate::{
    service::fetch::Fetch, 
    components::{
        spinner::Spinner, 
        stats_card::StatsCard
    }, 
    layouts::game_layout::GameLayout,
};

pub struct Game {
    pub stats: GameStats,
    pub status: Status,
    pub _listener: LocationHandle,
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameProps {
    pub id: String,
}

pub enum Msg {
    MakeReq(String),
    Response(GameStats),
    Nothing,
    Failed,
}

pub enum Status {
    Fetching,
    Finished,
}

impl Component for Game {
    type Message = Msg;
    type Properties = GameProps;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.clone();
        ctx.link().send_message(Msg::MakeReq(id));

        let listener = ctx.link()
            .add_location_listener(ctx.link().callback(|location: Location| {
                let id = location.path().split('/')
                    .last()
                    .unwrap()
                    .to_string();

                if Uuid::try_parse(&id).is_ok() {
                    return Msg::MakeReq(id);
                }

                Msg::Nothing
            }))
            .unwrap();

        Game {
            stats: GameStats::default(),
            status: Status::Finished,
            _listener: listener
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq(id) => {
                self.status = Status::Fetching;

                ctx.link().send_future(async move {
                    let url = format!("http://localhost:8080/api/stats/game/{}", id);
                    let game_stats = Fetch::get(&url, Some(true)).await;
                    if game_stats.is_err() {
                        return Msg::Failed;
                    }

                    let stats_data: ResponseBody<GameStats> =
                        serde_wasm_bindgen::from_value(game_stats.unwrap()).unwrap();
                    Msg::Response(stats_data.data)
                })
            }
            Msg::Response(data) => {
                self.stats = data;
                self.status = Status::Finished
            }
            Msg::Failed => self.status = Status::Finished,
            Msg::Nothing => {},
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone();

        html! {
            <GameLayout id={id.clone()}>
                {match &self.status {
                    Status::Fetching => {
                        html! {
                            <div class={classes!("flex", "justify-center", "items-center", "h-full")}>
                                <Spinner class="w-20 h-20" />
                            </div>
                        }
                    },
                    Status::Finished => {
                        html! {
                            <div class={classes!("flex", "flex-wrap", "w-full")}>
                                <StatsCard name="Levels" value={0} icon="map" class="ml-0" />
                                <StatsCard name="Scores" value={self.stats.scores} icon="scoreboard" class="ml-0" />
                            </div>
                        }
                    },
                }}
            </GameLayout>
        }
    }
}
