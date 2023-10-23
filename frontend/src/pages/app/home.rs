use serde::{Deserialize, Serialize};
use yew::{Component, html, classes, Context, Html};

use crate::{
    service::fetch::Fetch, 
    components::stats_card::StatsCard
};

pub struct Home {
    pub username: String
}

pub enum Msg {
    MakeReq,
    Response(String),
    Failed
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);

        Home { username: String::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                ctx.link().send_future(async {
                    let info_message = Fetch::get("https://sso.bonk.group/application/o/userinfo/", Some(true)).await;
                    if info_message.is_err() {
                        return Msg::Failed;
                    }

                    let info_response: UserInfo = serde_wasm_bindgen::from_value(info_message.unwrap()).unwrap();
                    Msg::Response(info_response.nickname)
                });
            },
            Msg::Response(username) => {
                self.username = username;
            },
            Msg::Failed => todo!(),
        }

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("p-4")}>
                <h1 class={classes!("text-3xl", "font-medium")}>
                    {format!("Welkom, {}!", self.username)}
                </h1>
                <section class={classes!("mt-8")}>
                    <h2 class={classes!("text-xl", "font-medium")}>
                        {"Statistieken"}
                    </h2>
                    <div class={classes!("flex", "flex-wrap", "w-full", "mt-6")}>
                       <StatsCard name="Games" value={5} icon="joystick" class="ml-0" />
                       <StatsCard name="Scores" value={5} icon="scoreboard" />
                    </div>
                </section>
            </div>
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub nickname: String
}