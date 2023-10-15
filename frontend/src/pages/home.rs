use std::collections::HashMap;

use babs::models::Game;
use babs::respone::ResponseBody;
use yew::{Component, html, classes, Html, Context};
use yew_router::prelude::Link;

use crate::MainRoute;
use crate::components::{
    drawer::Drawer,
    icon::Icon,
    list::List, 
    list_item::ListItem, 
    list_item_button::ListItemButton, 
    list_item_icon::ListItemIcon,
    app_bar::AppBar, toolbar::Toolbar
};
use crate::service::fetch::Fetch;

pub struct Home {
    games: Vec<Game>
}

pub enum Msg {
    MakeReq,
    Resp(Vec<Game>),
    Error(&'static str),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
       ctx.link().send_message(Msg::MakeReq);
    
        Home { games: Vec::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                ctx.link().send_future(async {                    
                    let headers = HashMap::from([
                        ("Authorization", "Test 123")
                    ]);

                    match Fetch::get("http://localhost:8080/api/game", headers).await {
                        Ok(body) => {
                            if let Ok(response) = serde_wasm_bindgen::from_value::<ResponseBody<Vec<Game>>>(body) {
                                return Msg::Resp(response.data);
                            }

                            Msg::Error("Failed to fetch games")
                        },
                        Err(_) => Msg::Error("Failed to fetch games")
                    }
                })
                
            },
            Msg::Resp(games) => { self.games = games },
            Msg::Error(_message) => {
                
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("min-h-screen", "bg-zinc-800", "text-white", "flex")}>
                <AppBar>
                    <Toolbar class="border-b border-zinc-500 border-solid">
                        <Link<MainRoute> to={MainRoute::App} classes={classes!("font-medium", "text-xl", "flex", "items-center")}>
                            <Icon name="database" class="mr-4"/>
                            {"Bonk Inc Backend System"}
                        </Link<MainRoute>>
                    </Toolbar>
                </AppBar>
                <nav class={classes!("h-full")}>
                    <Drawer>
                        <Toolbar class="mt-14 [min-h-48px]">
                            <p class={classes!("font-medium", "flex")}>
                                <Icon name="list" class="mr-2"/>
                                {"Games"}
                            </p>
                        </Toolbar>
                        <hr class={classes!("w-11/12", "border-zinc-500", "mx-auto")}/>
                        <List>
                            <ListItem>
                                <ListItemButton>
                                    <ListItemIcon>
                                        <Icon name="joystick" />
                                    </ListItemIcon>
                                    <p>{"test test"}</p>
                                </ListItemButton>
                            </ListItem>
                        </List>
                    </Drawer>
                </nav>
                <main style="width: calc(100% - 240px);" class={classes!("grow", "mt-14")}>
                    
                </main>
            </div>
        }
    }
}