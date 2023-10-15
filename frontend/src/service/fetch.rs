use std::collections::HashMap;

use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, RequestRedirect, Request, Response};

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

pub async fn fetch(url: &str, method: String, headers: HashMap<&str, &str>) -> Result<JsValue, JsValue> {
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

    for (name, value) in headers {
        let _ = request.headers().set(name, value);
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

pub struct Fetch();

impl Fetch {
    async fn fetch(url: &str, method: Method, headers: HashMap<&str, &str>) -> Result<JsValue, JsValue> {
        let method = match method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        };
        fetch(url, method.to_string(), headers).await
    }

    pub async fn get(url: &str, headers: HashMap<&str, &str>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::GET, headers).await
    }

    pub async fn post(url: &str, headers: HashMap<&str, &str>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::POST, headers).await
    }

    pub async fn put(url: &str, headers: HashMap<&str, &str>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::PUT, headers).await
    }

    pub async fn patch(url: &str, headers: HashMap<&str, &str>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::PATCH, headers).await
    }

    pub async fn delete(url: &str, headers: HashMap<&str, &str>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::DELETE, headers).await
    }
}