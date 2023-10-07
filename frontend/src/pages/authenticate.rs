use babs::respone::ResponseBody;
use wasm_bindgen::JsCast;
use web_sys::UrlSearchParams;
use yew::{Component, html, classes};

use crate::{service::fetch::Fetch, models::oauth::TokenResponse};

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

    fn create(ctx: &yew::Context<Self>) -> Self {
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
                match Fetch::get(&url).await {
                    Ok(message) => {
                        let response: ResponseBody<TokenResponse> = serde_wasm_bindgen::from_value(message).unwrap();
                        let document = gloo::utils::document().dyn_into::<web_sys::HtmlDocument>().unwrap();
                        let _ = document.set_cookie(format!("access_token={};max-age={}", response.data.access_token, response.data.expires_in).as_str());

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

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                ctx.link().send_future(async {
                    match Fetch::get("http://localhost:8080/auth/authorize").await {
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
                false
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <main class={classes!("min-h-screen", "bg-zinc-800", "flex", "justify-center", "items-center")}>
                <div class={classes!("bg-zinc-700", "text-slate-200", "w-[450px]", "px-8", "py-10", "shadow-lg")}>
                    <h1 class={classes!("text-center", "font-bold", "text-xl", "mb-8")}>
                        {"Inloggen in Bonk Inc. Backend System"}
                    </h1>
                    {match &self.state {
                        LoginState::Failed(error) => html! {
                            <div class={classes!("bg-red-600", "pr-2", "py-3", "mb-6", "font-bold", "rounded", "inline-flex", "items-center")}>
                                <i class={classes!("material-symbols-outlined", "px-4")}>{"error"}</i> {error}
                            </div>
                        },
                        LoginState::Autheticanting => html! {
                            <div class={classes!("inline-flex", "items-center")}>
                                <svg class="animate-spin ml-1 mr-3 h-5 w-5 text-blue-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                            </div>
                        },
                        _ => html! {
                            <button 
                                onclick={ctx.link().callback(|_| Msg::Login)}
                                class={classes!("text-center", "font-bold", "text-base", "block", "p-2", "w-full", "border", "border-solid", "rounded", "border-zinc-500", "transition-colors")}
                            >
                                {"Inloggen met Authentic"}
                            </button>
                        }
                    }}
                </div>
            </main>
        }
    }
}