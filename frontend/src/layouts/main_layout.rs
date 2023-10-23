use babs::models::Game;
use babs::respone::ResponseBody;
use yew::{Component, html, classes, Html, Context, Properties, Children};
use yew_router::prelude::Link;

use crate::app::AppRoute;
use crate::components::spinner::Spinner;
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

pub struct MainLayout {
    state: State,
    games: Vec<Game>
}
pub enum Msg {
    MakeReq,
    Resp(Vec<Game>),
    Error(String),
}

pub enum State {
    Loading,
    Loaded
}

#[derive(Clone, PartialEq, Properties)]
pub struct MainLayoutProps {
    pub children: Children,
}

impl Component for MainLayout {
    type Message = Msg;
    type Properties = MainLayoutProps;

    fn create(ctx: &Context<Self>) -> Self {
       ctx.link().send_message(Msg::MakeReq);
    
        MainLayout { games: Vec::new(), state: State::Loaded }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                self.state = State::Loading;

                ctx.link().send_future(async {       
                    match Fetch::get("http://localhost:8080/api/game", Some(true)).await {
                        Ok(body) => {
                            if let Ok(response) = serde_wasm_bindgen::from_value::<ResponseBody<Vec<Game>>>(body) {
                                return Msg::Resp(response.data);
                            }

                            Msg::Error("Failed to fetch games".to_string())
                        },
                        Err(_) => Msg::Error("Failed to fetch games".to_string())
                    }
                })
                
            },
            Msg::Resp(games) => { 
                self.games = games;
                self.state = State::Loaded
            },
            Msg::Error(_message) => {
                
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("min-h-screen", "bg-zinc-800", "text-white", "flex")}>
                <AppBar>
                    <Toolbar class="border-b border-zinc-500 border-solid justify-between">
                        <Link<AppRoute> to={AppRoute::Home} classes={classes!("font-medium", "text-xl", "flex", "items-center")}>
                            <Icon name="database" class="mr-4"/>
                            {"Bonk Inc Backend System"}
                        </Link<AppRoute>>
                        <div>
                            
                        </div>
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
                        {match &self.state {
                            State::Loading => html! {
                                <div class={classes!("flex", "justify-center", "items-center", "h-full")}>
                                    <Spinner class="w-20 h-20" />
                                </div>
                            },
                            State::Loaded => {
                                html! {
                                    <List>
                                        { for self.games.iter().map(|g| self.render_game_item(g)) }
                                    </List>
                                }
                            },
                        }}
                    </Drawer>
                </nav>
                <main style="width: calc(100% - 240px);" class={classes!("grow", "mt-14")}>
                    {ctx.props().children.clone()}
                </main>
            </div>
        }
    }
}

impl MainLayout {
    fn render_game_item(&self, game: &Game) -> Html {
        html! {
            <ListItem>
                <ListItemButton>
                    <ListItemIcon>
                        <Icon name="joystick" />
                    </ListItemIcon>
                    {&game.name}
                </ListItemButton>
            </ListItem>
        }
    }
}