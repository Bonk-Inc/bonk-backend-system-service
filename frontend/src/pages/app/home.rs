use babs::{models::Stats, respone::ResponseBody};
use serde::{Deserialize, Serialize};
use yew::{Component, html, classes, Context, Html};

use crate::{
    service::fetch::Fetch, 
    components::{
        stats_card::StatsCard,
        spinner::Spinner
    }
};

pub struct Home {
    pub username: String,
    pub stats: Stats,
    pub status: Status
}

pub enum Msg {
    MakeReq,
    Response((String, Stats)),
    Failed
}

pub enum Status {
    Fetching,
    Finished,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);

        Home { username: String::new(), stats: Stats::default(), status: Status::Fetching }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                ctx.link().send_future(async {
                    let info_message = Fetch::get("https://sso.bonk.group/application/o/userinfo/", Some(true)).await;
                    let stats = Fetch::get("http://localhost:8080/api/stats/all", Some(true)).await;
                    
                    if info_message.is_err() || stats.is_err() {
                        return Msg::Failed;
                    }

                    let info_response: UserInfo = serde_wasm_bindgen::from_value(info_message.unwrap()).unwrap();
                    let stats_data: ResponseBody<Stats> = serde_wasm_bindgen::from_value(stats.unwrap()).unwrap();
                    Msg::Response((info_response.nickname, stats_data.data))
                });
            },
            Msg::Response(data) => {
                self.username = data.0;
                self.stats = data.1;
                self.status = Status::Finished;
            },
            Msg::Failed => {
                self.status = Status::Finished;
            },
        }

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("p-4")}>
                {match self.status {
                    Status::Fetching => html! {
                        <div class={classes!("flex", "justify-center", "items-center", "h-full")}>
                            <Spinner class="w-20 h-20" />
                        </div>
                    },
                    Status::Finished => html! {
                        <>
                            <h1 class={classes!("text-3xl", "font-medium")}>
                                {format!("Welkom, {}!", self.username)}
                            </h1>
                            <section class={classes!("mt-8")}>
                                <h2 class={classes!("text-xl", "font-medium")}>
                                    {"Statistieken"}
                                </h2>
                                <div class={classes!("flex", "flex-wrap", "w-full", "mt-6")}>
                                <StatsCard name="Games" value={self.stats.games} icon="joystick" class="ml-0" />
                                <StatsCard name="Scores" value={self.stats.scores} icon="scoreboard" />
                                </div>
                            </section>
                        </>
                    },
                }}
            </div>
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub nickname: String
}