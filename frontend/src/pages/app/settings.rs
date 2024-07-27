use babs::{models::Game, respone::ResponseBody};
use yew::{classes, html, Component, Context, Html, Properties};

use crate::{
    env,
    components::{alert::{Alert, Severity},spinner::Spinner},
    layouts::game_layout::GameLayout, 
    service::fetch::Fetch
};

pub struct Settings {
    game: Game,
    status: Status
}

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsProps {
    pub game_id: String,
}

pub enum Msg {
    MakeReq(String),
    Response(Game),
    Error(String)
}

pub enum Status {
    Fetching,
    Finished,
    Failed(String)
}

impl Component for Settings {
    type Message = Msg;
    type Properties = SettingsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let game_id = ctx.props().game_id.clone();
        ctx.link().send_message(Msg::MakeReq(game_id));

        Settings {
            game: Game::default(),
            status: Status::Finished
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq(game_id) => {
                self.status = Status::Fetching;

                ctx.link().send_future(async move {
                    let game_url: String = format!("{}/api/game/{}", env::APP_API_URL, game_id);

                    let game = Fetch::get(&game_url, Some(true)).await;
                    if game.is_err() {
                        return Msg::Error("Error fetching game".to_string());
                    }

                    let game_data: ResponseBody<Game> =
                        serde_wasm_bindgen::from_value(game.unwrap()).unwrap();

                    Msg::Response(game_data.data)
                });
            },
            Msg::Response(game) => {
                self.game = game;
                self.status = Status::Finished;
            },
            Msg::Error(message) => self.status = Status::Failed(message),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();

        html! {
            <GameLayout id={game_id}>
                {match &self.status {
                    Status::Fetching => {
                        html! {
                            <div class={classes!("flex", "justify-center", "items-center", "h-full")}>
                                <Spinner class="w-20 h-20" />
                            </div>
                        }
                    },
                    _ => {
                        html! {
                            <>
                                {if let Status::Failed(message) = &self.status {
                                        html! {
                                            <div class={classes!("absolute", "w-80", "z-50", "top-20", "left-[40%]")}>
                                                <Alert severity={Severity::Error}>{message.clone()}</Alert>
                                            </div>
                                        }
                                } else { html!() }}
                                <h1 class={classes!("mt-4", "text-2xl", "font-medium", "mb-4")}>
                                    {"Instellingen"}
                                </h1>
                            </>
                        }
                    }
                }}
            </GameLayout>
        }
    }
}