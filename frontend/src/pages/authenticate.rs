use babs::respone::ResponseBody;
use web_sys::{UrlSearchParams, window};
use yew::{Component, html, classes, Context, Html};
use yew_router::prelude::*;

use crate::{
    app::AppRoute,
    service::fetch::Fetch,
    models::oauth::TokenResponse,
    components::{
        alert::{Alert, Severity},
        spinner::Spinner,
        button::{Button, ButtonVariant},
        paper::{Paper, PaperElevation}
    }, 
};

pub struct Authenticate {
    state: LoginState
}

pub enum Msg {
    AlreadyAuthenticated,
    Authenticated,
    Login,
    SetAuthCode(String),
    SetError(String),
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
        let window = window().unwrap();
        let search_string = window.location().search().unwrap_or_default();
        let search_params = UrlSearchParams::new_with_str(&search_string);
        if let Ok(params) = search_params {
            let code = params.get("code");
            let state = params.get("state");

            if code.is_none() || state.is_none() {
                let local_storage = window.local_storage().unwrap().unwrap();
                let refresh_token = local_storage.get_item("refresh_token").unwrap();

                if let Some(token) = refresh_token {
                    ctx.link().send_future(async move {
                        let url = format!("http://localhost:8080/auth/refresh?token={}", token);
                        match Fetch::get(&url, None).await {
                            Ok(message) => {
                                let response: ResponseBody<TokenResponse> = serde_wasm_bindgen::from_value(message).unwrap();
                                let local_storage: Option<web_sys::Storage> = window.local_storage().unwrap();
                                let session_storage = window.session_storage().unwrap();
        
                                let _ = session_storage.unwrap().set_item("access_token", &response.data.refresh_token);
                                let _ = local_storage.unwrap().set_item("refresh_token", &response.data.refresh_token);
        
                                Msg::AlreadyAuthenticated
                            },
                            Err(_) => Msg::SetError("Error authenticating".to_string()),
                        }
                    });
                }

                return Authenticate { 
                    state: LoginState::Unauthenticated
                };
            }

            ctx.link().send_future(async move {
                let url = format!("http://localhost:8080/auth/login?code={}&state={}", code.unwrap(), state.unwrap());
                match Fetch::get(&url, None).await {
                    Ok(message) => {
                        let response: ResponseBody<TokenResponse> = serde_wasm_bindgen::from_value(message).unwrap();
                        let local_storage = window.local_storage().unwrap();
                        let session_storage = window.session_storage().unwrap();

                        let _ = session_storage.unwrap().set_item("access_token", &response.data.refresh_token);
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
                    match Fetch::get("http://localhost:8080/auth/authorize", None).await {
                        Ok(message) => {
                            let response: ResponseBody<String> = serde_wasm_bindgen::from_value(message).unwrap();
                            Msg::SetAuthCode(response.data)
                        },
                        Err(_) => Msg::SetError("Error authorizing".to_string()),
                    }
                });
            }
            Msg::SetAuthCode(state) => {
                let _ = window().unwrap().location().assign(&state);
            },
            Msg::SetError(error) => {
                self.state = LoginState::Failed(error);
            },
            Msg::Authenticated => {
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Home);
            }
            Msg::AlreadyAuthenticated => {
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Home);
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main class={classes!("min-h-screen", "bg-zinc-800", "flex", "justify-center", "items-center")}>
                <Paper class="w-[450px] py-10 px-8" elevation={PaperElevation::Elevated}>
                    <h1 class={classes!("text-center", "font-bold", "text-xl", "mb-8")}>
                        {"Inloggen in Bonk Inc. Backend System"}
                    </h1>
                    {match &self.state {
                        LoginState::Autheticanting => html!(<Spinner />),
                        _ => html! {
                            <>
                                if let LoginState::Failed(error) = &self.state {
                                    <Alert severity={Severity::Error}>{error.clone()}</Alert>
                                }
                                <Button 
                                    onclick={ctx.link().callback(|_| Msg::Login)} 
                                    variant={ButtonVariant::Outlined}
                                    class="border-zinc-500"
                                >
                                    {"Inloggen met Authentic"}
                                </Button>
                            </>
                        }
                    }}
                </Paper>
            </main>
        }
    }
}