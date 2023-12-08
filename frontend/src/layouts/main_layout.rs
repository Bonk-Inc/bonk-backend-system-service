use babs::models::Game;
use babs::respone::ResponseBody;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, window};
use yew::{Component, html, classes, Html, Context, Properties, Children};
use yew_router::prelude::{Link, Navigator};
use yew_router::scope_ext::{RouterScopeExt, LocationHandle};

use crate::app::AppRoute;
use crate::components::button::ButtonVariant;
use crate::components::text_field::TextField;
use crate::components::{
    app_bar::AppBar,
    button::Button,
    dialog::Dialog,
    drawer::Drawer,
    icon::Icon,
    list::List, 
    list_item::ListItem, 
    list_item_button::ListItemButton, 
    list_item_icon::ListItemIcon,
    spinner::Spinner,
    toolbar::Toolbar
};
use crate::models::game::GameDTO;
use crate::service::fetch::Fetch;

pub struct MainLayout {
    state: State,
    new_game_name: String,
    create_game_open: bool,
    games: Vec<Game>,
    _listener: LocationHandle
}

pub enum Msg {
    RequestGames,
    AddNewGame,
    CancelNewGame,
    SaveGame,
    GameSaved,
    Rerender,
    NavigateToGame(String),
    NameChange(Event),
    Resp(Vec<Game>),
    Error(String),
}

pub enum State {
    CreateNewGame,
    SavingGame,
    GameAdded,
    FetchingGames,
    GamesFetched
}

#[derive(Clone, PartialEq, Properties)]
pub struct MainLayoutProps {
    pub children: Children,
}

impl Component for MainLayout {
    type Message = Msg;
    type Properties = MainLayoutProps;

    fn create(ctx: &Context<Self>) -> Self {
       ctx.link().send_message(Msg::RequestGames);

       let listener = ctx.link()
            .add_location_listener(ctx.link().callback(|_| Msg::Rerender))
            .unwrap();
    
        MainLayout { 
            games: Vec::new(),
            create_game_open: false,
            state: State::GamesFetched,
            new_game_name: String::new(),
            _listener: listener
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Rerender => {}
            Msg::RequestGames => {
                self.state = State::FetchingGames;

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
                self.state = State::GamesFetched
            },
            Msg::Error(_message) => {
            
            }
            Msg::AddNewGame => { self.create_game_open = true },
            Msg::CancelNewGame => { self.create_game_open = false },
            Msg::SaveGame => { 
                self.state = State::SavingGame;

                let game = GameDTO { name: self.new_game_name.clone() };
                let body = serde_json::to_string(&game).unwrap();

                ctx.link().send_future(async move {
                    match Fetch::post("http://localhost:8080/api/game", &body, Some(true)).await {
                        Ok(_) => Msg::GameSaved,
                        Err(_) => Msg::Error("Failed to save game".to_string()),
                    }
                })
            },
            Msg::NameChange(e) => {
                let target = e.target().unwrap();
                let input = target.dyn_ref::<HtmlInputElement>().unwrap();
                
                self.new_game_name = input.value();
            },
            Msg::GameSaved => {
                ctx.link().send_message(Msg::RequestGames);

                self.create_game_open = false;
            },
            Msg::NavigateToGame(id) => {
                let navigator: Navigator = ctx.link().navigator().unwrap();
                navigator.push(&AppRoute::Game { id });
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
                        <Toolbar class="mt-14 !min-h-48px justify-between items-center">
                            <p class={classes!("font-medium", "flex")}>
                                <Icon name="list" class="mr-2"/>
                                {"Games"}
                            </p>
                            <Button class="!w-auto !flex min-w-min !px-0" onclick={ctx.link().callback(|_| Msg::AddNewGame)}>
                                <Icon name="add"/>
                            </Button>
                        </Toolbar>
                        <hr class={classes!("w-11/12", "border-zinc-500", "mx-auto")}/>
                        {match &self.state {
                            State::FetchingGames => html! {
                                <div class={classes!("flex", "justify-center", "items-center", "h-full")}>
                                    <Spinner class="w-20 h-20" />
                                </div>
                            },
                            _ => {
                                html! {
                                    <List>
                                        { for self.games.iter().map(|g| self.render_game_item(ctx, g)) }
                                    </List>
                                }
                            },
                        }}
                    </Drawer>
                </nav>
                <main style="width: calc(100% - 240px);" class={classes!("grow", "mt-14")}>
                    {ctx.props().children.clone()}
                </main>
                <Dialog open={&self.create_game_open} class="w-[450px] min-h-[300px] flex flex-wrap">
                    <h2 class={classes!("text-xl", "font-medium", "mb-4")}>
                        {"Game toevoegen"}
                    </h2>
                    {match &self.state {
                        State::SavingGame => {
                            html! {
                                <div class={classes!("flex", "justify-center", "items-center", "h-full", "w-full")}>
                                    <Spinner class="w-20 h-20" />
                                </div>
                            }
                        },
                        _ => {
                            html! {
                                <>
                                    <TextField 
                                        id={"game-name"}
                                        class="mb-10"
                                        name={"game-name"}
                                        required={true}
                                        label={"Naam"}
                                        full_width={true}
                                        onchange={ctx.link().callback(|e| Msg::NameChange(e))}
                                    />
                                    <div class={classes!("flex", "items-center", "justify-end", "w-full")}>
                                        <Button
                                            variant={ButtonVariant::Outlined}
                                            onclick={ctx.link().callback(|_| Msg::CancelNewGame)}
                                            class="text-blue-400 border-zinc-500 hover:bg-zinc-600"
                                        >
                                            {"Annuleren"}
                                        </Button>
                                        <Button
                                            variant={ButtonVariant::Contained}
                                            onclick={ctx.link().callback(|_| Msg::SaveGame)}
                                            class="ml-4 bg-blue-400 hover:bg-blue-300"
                                        >
                                            {"Opslaan"}
                                        </Button>
                                    </div>
                                </>
                            }
                        }
                    }}
                </Dialog>
            </div>
        }
    }
}

impl MainLayout {
    fn render_game_item(&self, ctx: &Context<Self>, game: &Game) -> Html {
        let game_id = game.id.to_string();
        let path = window().unwrap().location().pathname().unwrap();

        html! {
            <ListItem>
                <ListItemButton
                    selected={path.contains(&game_id)}
                    onclick={ctx.link().callback(move |_| Msg::NavigateToGame(game_id.clone()))}
                >
                    <ListItemIcon>
                        <Icon name="joystick" />
                    </ListItemIcon>
                    {&game.name}
                </ListItemButton>
            </ListItem>
        }
    }
}