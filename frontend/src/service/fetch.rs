use babs::respone::ResponseBody;
use gloo::utils::window;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, RequestRedirect, Request, Response};

use crate::models::oauth::TokenResponse;

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

pub async fn fetch(url: &str, method: &str, access_token: Option<String>) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(&method);
    opts.mode(RequestMode::Cors);
    opts.redirect(RequestRedirect::Follow);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set(
        "Access-Control-Request-Headers",
        "Content-Type, Authorization",
    )?;
    request
        .headers()
        .set("Access-Control-Request-Method", &method)?;

    if access_token.is_some() {
        request.headers().set("Authorization", &format!("Bearer {}", &access_token.unwrap()))?;
    }

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    if !resp.ok() {
        return Err(resp.into());
    }

    // Convert a JS Promise into a Rust Future
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub async fn get_access_token() -> Option<String> {
    let local_storage = window().local_storage().unwrap();
    let refresh_token = local_storage.clone().unwrap().get_item("refresh_token").unwrap();
    if refresh_token.is_none() {
        return None;
    }
    
    let url = format!("http://localhost:8080/auth/refresh?token={}", refresh_token.unwrap());
    match fetch(&url, "GET", None).await {
        Ok(message) => {
            let response: ResponseBody<TokenResponse> = serde_wasm_bindgen::from_value(message).unwrap();
            let access_token = response.data.access_token;

            let _ = local_storage.unwrap().set_item("refresh_token", &response.data.refresh_token);
            let _ = window().session_storage().unwrap().unwrap().set_item("access_token", &access_token);
            
            Some(access_token)
        },
        Err(_) => None,
    }
}

pub struct Fetch;

impl Fetch {
    async fn fetch(url: &str, method: Method, authenication: bool) -> Result<JsValue, JsValue> {
        let method = match method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        };

        let mut access_token: Option<String> = None;
        if authenication {
            access_token = get_access_token().await;
        }

        fetch(url, method, access_token).await
    }

    pub async fn get(url: &str, authenication: Option<bool>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::GET, authenication.unwrap_or_default())
            .await
    }

    pub async fn post(url: &str, authenication: Option<bool>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::POST, authenication.unwrap_or_default())
            .await
    }

    pub async fn put(url: &str, authenication: Option<bool>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::PUT, authenication.unwrap_or_default())
            .await
    }

    pub async fn patch(url: &str, authenication: Option<bool>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::PATCH, authenication.unwrap_or_default())
            .await
    }

    pub async fn delete(url: &str, authenication: Option<bool>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::DELETE, authenication.unwrap_or_default())
            .await
    }
}