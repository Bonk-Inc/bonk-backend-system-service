use std::collections::HashMap;

use babs::respone::ResponseBody;
use wasm_bindgen::JsCast;
use web_sys::UrlSearchParams;
use yew::{Component, html, classes, Context, Html};
use yew_router::prelude::*;

use crate::{
    service::fetch::Fetch,
    models::oauth::TokenResponse,
    MainRoute, components::{
        alert::{Alert, Severity},
        spinner::Spinner,
        button::{Button, ButtonVariant},
        paper::{Paper, PaperElevation}
    }
};

pub struct Authenticate {
    state: LoginState
}

pub enum Msg {
    Login,
    SetAuthCode(String),
    SetError(String),
    Authenticated
}

pub enum LoginState {
    Unauthenticated,
    Autherize(ResponseBody<String>),
    Autheticanting,
    Failed(String)
}

impl Component for Authenticate {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = gloo::utils::window();
        let search_string = window.location().search().unwrap_or_default();
        let search_params = UrlSearchParams::new_with_str(&search_string);
        if let Ok(params) = search_params {
            let code = params.get("code");
            let state = params.get("state");

            if code.is_none() || state.is_none() {
                return Authenticate { 
                    state: LoginState::Unauthenticated
                };
            }

            ctx.link().send_future(async {
                let url = format!("http://localhost:8080/auth/login?code={}&state={}", code.unwrap(), state.unwrap());
                match Fetch::get(&url, HashMap::new()).await {
                    Ok(message) => {
                        let response: ResponseBody<TokenResponse> = serde_wasm_bindgen::from_value(message).unwrap();
                        let document = gloo::utils::document().dyn_into::<web_sys::HtmlDocument>().unwrap();
                        let local_storage = gloo::utils::window().local_storage().unwrap();

                        let _ = document.set_cookie(&format!("access_token={};max-age={};path=/;samesite=strict", response.data.access_token, response.data.expires_in));
                        let _ = local_storage.unwrap().set_item("refresh_token", &response.data.refresh_token);

                        Msg::Authenticated
                    },
                    Err(_) => Msg::SetError("Error authenticating".to_string()),
                }
            });
        }

        Authenticate { 
            state: LoginState::Unauthenticated
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                ctx.link().send_future(async {
                    match Fetch::get("http://localhost:8080/auth/authorize", HashMap::new()).await {
                        Ok(message) => {
                            let response: ResponseBody<String> = serde_wasm_bindgen::from_value(message).unwrap();
                            Msg::SetAuthCode(response.data)
                        },
                        Err(_) => Msg::SetError("Error authorizing".to_string()),
                    }
                });

                false
            }
            Msg::SetAuthCode(state) => {
                let window = gloo::utils::window();
                let _ = window.location().assign(&state);

                true
            },
            Msg::SetError(error) => {
                self.state = LoginState::Failed(error);

                true
            },
            Msg::Authenticated => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&MainRoute::App);

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class={classes!("min-h-screen", "bg-zinc-800", "flex", "justify-center", "items-center")}>
                <Paper class="w-[450px] py-10" elevation={PaperElevation::Elevated}>
                    <h1 class={classes!("text-center", "font-bold", "text-xl", "mb-8")}>
                        {"Inloggen in Bonk Inc. Backend System"}
                    </h1>
                    {match &self.state {
                        LoginState::Failed(error) => html! { <Alert severity={Severity::Error}>{error.clone()}</Alert>},
                        LoginState::Autheticanting => html! { <Spinner /> },
                        _ => html! {
                            <Button 
                                onclick={ctx.link().callback(|_| Msg::Login)} 
                                variant={ButtonVariant::Outlined}
                                class="border-zinc-500"
                            >
                                {"Inloggen met Authentic"}
                            </Button>
                        }
                    }}
                </Paper>
            </main>
        }
    }
}