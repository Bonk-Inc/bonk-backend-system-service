use std::collections::HashMap;

use babs::respone::ResponseBody;
use wasm_bindgen::JsCast;
use yew_router::scope_ext::RouterScopeExt;
use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::{
    service::fetch::Fetch,
    models::oauth::TokenResponse,
    MainRoute
};

pub struct AuthenticationProvider;

#[derive(Clone, PartialEq, Properties)]
pub struct AuthenticationProviderProps {
    pub children: Children,
}

pub enum Msg {
    RefreshToken,
    RefreshFailed,
    RefreshSuccessful(TokenResponse),
}

impl Component for AuthenticationProvider {
    type Message = Msg;
    type Properties = AuthenticationProviderProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::RefreshToken);

        AuthenticationProvider {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RefreshToken => {
                ctx.link().send_future(async {
                    let local_storage = gloo::utils::window().local_storage().unwrap();
                    let refresh_token = local_storage.unwrap().get_item("refresh_token").unwrap();
                    if refresh_token.is_none() {
                        return Msg::RefreshFailed;
                    }

                    let url = format!("http://localhost:8080/auth/refresh?token={}", refresh_token.unwrap());
                    match Fetch::get(&url, HashMap::new()).await {
                        Ok(message) => {
                            let response: ResponseBody<TokenResponse> = serde_wasm_bindgen::from_value(message).unwrap();
                            Msg::RefreshSuccessful(response.data)
                        },
                        Err(_) => Msg::RefreshFailed,
                    }
                });
            },
            Msg::RefreshSuccessful(token_data) => {
                let document = gloo::utils::document().dyn_into::<web_sys::HtmlDocument>().unwrap();
                let local_storage = gloo::utils::window().local_storage().unwrap();
                
                let _ = document.set_cookie(&format!("access_token={};max-age={};path=/;samesite=strict", token_data.access_token, token_data.expires_in));
                let _ = local_storage.unwrap().set_item("refresh_token", &token_data.refresh_token);
            },
            Msg::RefreshFailed => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&MainRoute::Authenticate)
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
               {ctx.props().children.clone()}
            </>
        }
    }
}