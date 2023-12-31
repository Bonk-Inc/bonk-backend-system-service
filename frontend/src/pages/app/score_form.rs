use std::str::FromStr;

use babs::{
    models::{Game, Score},
    respone::ResponseBody,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, window, UrlSearchParams};
use yew::{classes, html, Component, Context, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    components::{
        button::{Button, ButtonSize, ButtonVariant},
        checkbox::Checkbox,
        form_control::FormControl,
        select::Select,
        text_field::TextField,
    },
    service::fetch::Fetch, app::AppRoute,
};

pub struct ScoreForm {
    score: Score,
    game_id: Uuid,
    games: Vec<Game>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ScoresFormProps {
    #[prop_or_default]
    pub score_id: Option<String>,
}

pub enum Msg {
    FetchScore(String),
    //FetchGames,
    SaveScore,
    ChangeHidden,
    Cancelled,
    //Response(Vec<Game>),
    Error(String),
    ScoreSaved,
    SetScore(Score),
    ChangeUsername(String),
    ChangeScore(String),
    ChangeGame(String),
}

impl Component for ScoreForm {
    type Message = Msg;
    type Properties = ScoresFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        let search_string = window().unwrap().location().search().unwrap_or_default();
        let search_params = UrlSearchParams::new_with_str(&search_string).unwrap();
        let game_id = search_params.get("game").unwrap_or_default();

        //ctx.link().send_message(Msg::FetchGames);

        if let Some(score_id) = &ctx.props().score_id {
            ctx.link().send_message(Msg::FetchScore(score_id.clone()))
        }

        ScoreForm {
            score: Score::default(),
            game_id: Uuid::from_str(&game_id).unwrap_or_default(),
            games: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Msg::FetchGames => ctx.link().send_future(async {
            //     match Fetch::get("http://localhost:8080/api/game", Some(true)).await {
            //         Ok(body) => {
            //             if let Ok(response) =
            //                 serde_wasm_bindgen::from_value::<ResponseBody<Vec<Game>>>(body)
            //             {
            //                 return Msg::Response(response.data);
            //             }

            //             Msg::Error("Failed to fetch games".to_string())
            //         }
            //         Err(_) => Msg::Error("Failed to fetch games".to_string()),
            //     }
            // }),
            Msg::FetchScore(score_id) => {
                ctx.link().send_future(async move {
                    let url = format!("http://localhost:8080/api/score/{}", score_id);
                    let scores = Fetch::get(&url, Some(true)).await;
                    if scores.is_err() {
                        return Msg::Error("Error fetching Score".to_string());
                    }

                    let response: ResponseBody<Score> = serde_wasm_bindgen::from_value(scores.unwrap()).unwrap();
                    Msg::SetScore(response.data)
                });
            },
            Msg::SetScore(score) => self.score = score,
            Msg::SaveScore => {
                self.score.game_id = self.game_id;

                let score_id = ctx.props().score_id.clone();
                let body = serde_json::to_string(&self.score).unwrap();

                ctx.link().send_future(async move {
                    if let Some(id) = &score_id {
                        let url = format!("http://localhost:8080/api/score/{}", id);

                        return match Fetch::put(&url, &body, Some(true)).await {
                            Ok(_) => Msg::ScoreSaved,
                            Err(e) => Msg::Error(e.as_string().unwrap())
                        };
                    }

                    match Fetch::post("http://localhost:8080/api/score/", &body, Some(true)).await {
                        Ok(_) => Msg::ScoreSaved,
                        Err(e) => Msg::Error(e.as_string().unwrap())
                    }
                })
            }
            //Msg::Response(games) => self.games = games,
            
            Msg::ChangeUsername(username) => self.score.username = username,
            Msg::ChangeHidden => self.score.is_hidden = !self.score.is_hidden,
            Msg::ChangeGame(game) => {
                self.score.game_id = Uuid::from_str(&game).unwrap();
            }
            Msg::ChangeScore(score) => {
                let num_score = score.parse::<i32>();
                if num_score.is_err() {
                    ctx.link()
                        .send_message(Msg::Error("Score must be a number".to_owned()))
                }

                self.score.highscore = num_score.expect("score");
            }
            Msg::ScoreSaved | Msg::Cancelled => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Scores { game_id: self.game_id.to_string() })
            },
            Msg::Error(_) => todo!(),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let score = self.score.clone();

        html! {
            <div class={classes!("w-full", "flex", "justify-center", "bg-zinc-600")} style="height: calc(100vh - 56px);">
                <div class={classes!("p-4", "w-1/2", "bg-zinc-800")}>
                    <h1 class={classes!("font-medium", "text-lg", "mb-4", "mt-2")}>
                        {"Score details"}
                    </h1>
                    <form class={classes!("flex", "flex-wrap")}>
                        // <Select
                        //     id={"score-level"}
                        //     name={"score-level"}
                        //     class="mb-2"
                        //     full_width={true}
                        //     label={"Game"}
                        //     required={true}
                        //     onchange={ctx.link().callback(|e: Event| {
                        //         let target = e.target().unwrap();
                        //         let input = target.dyn_ref::<HtmlSelectElement>().unwrap();

                        //         Msg::ChangeGame(input.value())
                        //     })}
                        // >
                        //     { for self.games.iter().map(|g| html!(<option value={g.id.to_string()} >{g.name.clone()}</option>)) }
                        // </Select>
                        <TextField
                            id={"score-username"}
                            class="mb-4 !w-2/3 pr-3"
                            name={"score-name"}
                            label={"Username"}
                            value={score.username}
                            required={true}
                            onchange={ctx.link().callback(|e: Event| {
                                let target = e.target().unwrap();
                                let input = target.dyn_ref::<HtmlInputElement>().unwrap();

                                Msg::ChangeUsername(input.value())
                            })}
                        />
                        <TextField
                            id={"score-highscore"}
                            class="mb-4 !w-1/3 pl-3"
                            name={"score-highscore"}
                            label={"Score"}
                            value={score.highscore.to_string()}
                            required={true}
                            onchange={ctx.link().callback(|e: Event| {
                                let target = e.target().unwrap();
                                let input = target.dyn_ref::<HtmlInputElement>().unwrap();

                                Msg::ChangeScore(input.value())
                            })}
                        />
                        <FormControl full_width={true}>
                            <label class={classes!("inline-flex", "items-center", "cursor-pointer")}>
                                <Checkbox
                                    id="score-hidden"
                                    class="!justify-start px-0 mr-3"
                                    checked={score.is_hidden}
                                    onchange={ctx.link().callback(|_| Msg::ChangeHidden)}
                                />
                                {"Hidden"}
                            </label>
                        </FormControl>
                        <hr class={classes!("w-full", "mt-8", "mb-5", "border-zinc-500")} />
                        <div class="w-full flex justify-end">
                            <div class="flex">
                                <Button
                                    class="mr-2 text-blue-400 border-zinc-500"
                                    variant={ButtonVariant::Outlined}
                                    size={ButtonSize::Large}
                                    onclick={ctx.link().callback(|_| Msg::Cancelled)}
                                >
                                    {"Cancel"}
                                </Button>
                                <Button
                                    class="bg-blue-400"
                                    variant={ButtonVariant::Contained}
                                    size={ButtonSize::Large}
                                    onclick={ctx.link().callback(|_| Msg::SaveScore)}
                                >
                                    {"Create"}
                                </Button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}
