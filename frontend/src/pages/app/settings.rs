use babs::{models::Game, respone::ResponseBody};
use wasm_bindgen::JsValue;
use web_sys::{console::log_1, window};
use yew::{classes, html, Component, Context, Html, Properties};

use crate::{
    components::{
        alert::{Alert, Severity},
        button::{Button, ButtonVariant},
        icon::Icon,
        spinner::Spinner,
    },
    env,
    layouts::game_layout::GameLayout,
    service::fetch::Fetch,
};

pub struct Settings {
    game: Game,
    status: Status,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsProps {
    pub game_id: String,
}

pub enum Msg {
    MakeReq(String),
    Response(Game),
    CopyGameId(String),
    DeleteGame(String),
    Error(String),
}

pub enum Status {
    Fetching,
    Finished,
    Failed(String),
}

impl Component for Settings {
    type Message = Msg;
    type Properties = SettingsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let game_id = ctx.props().game_id.clone();
        ctx.link().send_message(Msg::MakeReq(game_id));

        Settings {
            game: Game::default(),
            status: Status::Finished,
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
            }
            Msg::Response(game) => {
                self.game = game;
                self.status = Status::Finished;
            }
            Msg::CopyGameId(game_id) => {
                let _navigator = window().unwrap().navigator();
                log_1(&JsValue::from_str(&game_id));
            }
            Msg::DeleteGame(game_id) => {}
            Msg::Error(message) => self.status = Status::Failed(message),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();
        let copy_id = self.game.id.to_string();
        let delete_id = self.game.id.to_string();

        html! {
            <GameLayout id={game_id.clone()}>
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
                                <div class={classes!("flex", "flex-wrap", "w-full")}>
                                   <div class={classes!("flex", "flex-wrap", "w-1/3")}>
                                        <p class={classes!("font-bold", "text-base", "w-full")}>
                                            {"Game id:"}
                                        </p>
                                        <div class={classes!("flex", "w-full", "justify-between")}>
                                            <p class={classes!("pt-1", "font-normal", "text-base")}>
                                                {game_id.clone()}
                                            </p>
                                            <Button
                                                dense={true}
                                                title="Kopieer Level Id"
                                                onclick={ctx.link().callback(move |_| Msg::CopyGameId(copy_id.clone()))}
                                            >
                                                <Icon name="content_copy" />
                                            </Button>
                                        </div>
                                   </div>
                                </div>
                                <div class={classes!("flex", "w-full", "justify-end", "mt-16")}>
                                    <Button
                                        variant={ButtonVariant::Contained}
                                        class="flex items-center !text-white bg-red-600"
                                        onclick={ctx.link().callback(move |_| Msg::DeleteGame(delete_id.clone()))}
                                    >
                                        <Icon name="delete" class="mr-2" /> {"Verwijderen"}
                                    </Button>
                                </div>
                            </>
                        }
                    }
                }}
            </GameLayout>
        }
    }
}
