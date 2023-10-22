use serde::{Deserialize, Serialize};
use yew::{Component, html, Html, Context, classes};

use crate::{layouts::main_layout::MainLayout, service::fetch::Fetch};

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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MainLayout>
                <div class={classes!("p-4")}>
                    <h1 class={classes!("text-2xl", "font-medium")}>
                        {format!("Welkom, {}!", self.username)}
                    </h1>
                </div>
            </MainLayout>
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub nickname: String
}