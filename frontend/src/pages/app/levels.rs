use babs::{models::Level, respone::ResponseBody};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console::log_1, window, Event, HtmlInputElement};
use yew::{classes, html, Component, Context, Html, Properties};
use yew_router::navigator;

use crate::{
    components::{
        alert::{Alert, Severity},
        button::{Button, ButtonVariant},
        dialog::Dialog,
        icon::Icon,
        spinner::Spinner,
        table::Table,
        table_body::TableBody,
        table_cell::TableCell,
        table_container::TableContainer,
        table_head::TableHead,
        table_row::TableRow,
        text_field::TextField,
    },
    env,
    layouts::game_layout::GameLayout,
    models::level::LevelDTO,
    service::fetch::Fetch,
};

pub struct Levels {
    status: Status,
    levels: Vec<Level>,
    new_level_name: String,
    create_level_open: bool,
}

pub enum Status {
    Fetching,
    Finished,
    SavingLevel,
    Failed(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct LevelsProps {
    pub game_id: String,
}

pub enum Msg {
    MakeReq,
    Response(Vec<Level>),
    DeleteLevel(String),
    LevelDeleted(String),
    OpenAddPopup,
    CloseAddPopup,
    SaveLevel,
    CopyLevelId(String),
    LevelSaved,
    LevelNameChange(Event),
    Failed(String),
}

impl Component for Levels {
    type Message = Msg;
    type Properties = LevelsProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);

        Levels {
            status: Status::Fetching,
            levels: vec![],
            new_level_name: String::new(),
            create_level_open: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                let game_id = ctx.props().game_id.clone();
                self.status = Status::Fetching;

                ctx.link().send_future(async move {
                    let url = format!("{}/api/level/game/{}", env::APP_API_URL, game_id);

                    let levels = Fetch::get(&url, Some(true)).await;
                    if levels.is_err() {
                        return Msg::Failed("Failed fetching levels".to_string());
                    }

                    let levels_data: ResponseBody<Vec<Level>> =
                        serde_wasm_bindgen::from_value(levels.unwrap()).unwrap();
                    Msg::Response(levels_data.data)
                });
            }
            Msg::Response(leves) => {
                self.levels = leves;
                self.status = Status::Finished;
            }
            Msg::DeleteLevel(level_id) => {
                ctx.link().send_future(async move {
                    let url = format!("{}/api/level/{}", env::APP_API_URL, level_id);

                    let deleted_level = Fetch::delete(&url, Some(true)).await;
                    if deleted_level.is_err() {
                        return Msg::Failed("Failed deleting level".to_string());
                    }

                    Msg::LevelDeleted(level_id)
                });
            }
            Msg::LevelDeleted(level_id) => {
                self.levels = self
                    .levels
                    .iter()
                    .filter(|l| l.id.to_string() != level_id)
                    .map(|l| l.clone())
                    .collect::<Vec<Level>>();
            }
            Msg::LevelNameChange(event) => {
                let target = event.target().unwrap();
                let input = target.dyn_ref::<HtmlInputElement>().unwrap();

                self.new_level_name = input.value();
            }
            Msg::SaveLevel => {
                self.status = Status::SavingLevel;

                let game_id = ctx.props().game_id.clone();
                let level = LevelDTO {
                    name: self.new_level_name.clone(),
                    game_id,
                };
                let body = serde_json::to_string(&level).unwrap();

                ctx.link().send_future(async move {
                    match Fetch::post(
                        &format!("{}/api/level", env::APP_API_URL),
                        &body,
                        Some(true),
                    )
                    .await
                    {
                        Ok(_) => Msg::LevelSaved,
                        Err(_) => Msg::Failed("Failed to create new level".to_string()),
                    }
                })
            }
            Msg::LevelSaved => {
                ctx.link().send_message(Msg::MakeReq);

                self.create_level_open = false;
            }
            Msg::OpenAddPopup => self.create_level_open = true,
            Msg::CloseAddPopup => self.create_level_open = false,
            Msg::Failed(error) => self.status = Status::Failed(error),
            Msg::CopyLevelId(level_id) => {
                let navigator = window().unwrap().navigator();
                log_1(&JsValue::from_str(&level_id));
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();

        html! {
            <GameLayout id={game_id}>
                <div class="py-4 flex justify-end items-center">
                    <Button
                        class="bg-blue-400 inline-flex items-center"
                        onclick={ctx.link().callback(|_| Msg::OpenAddPopup)}
                        variant={ButtonVariant::Contained}
                    >
                        <Icon name="add" class="mr-2"/> {"Level toevoegen"}
                    </Button>
                </div>
                <TableContainer>
                    {match self.status {
                        Status::Fetching => html! {
                            <div class={classes!("flex", "justify-center", "items-center", "h-full")}>
                                <Spinner class="w-20 h-20" />
                            </div>
                        },
                        _ => html! {
                            <>
                                if let Status::Failed(error) = &self.status {
                                    <div class={classes!("absolute", "w-80", "z-50", "top-20", "left-[40%]")}>
                                        <Alert severity={Severity::Error}>{error.clone()}</Alert>
                                    </div>
                                }
                                <Table>
                                    <TableHead>
                                        <TableRow>
                                            <TableCell>
                                                {"Naam"}
                                            </TableCell>
                                            <TableCell>{""}</TableCell>
                                        </TableRow>
                                    </TableHead>
                                    <TableBody>
                                        { for self.levels.iter().map(|level| self.render_level_row(ctx, level)) }
                                    </TableBody>
                                </Table>
                            </>
                        }
                    }}
                </TableContainer>
                <Dialog open={&self.create_level_open} class="w-[450px] min-h-[300px] flex flex-wrap">
                    <h2 class={classes!("text-xl", "font-medium", "mb-4")}>
                        {"Game toevoegen"}
                    </h2>
                    {match &self.status {
                        Status::SavingLevel => {
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
                                        onchange={ctx.link().callback(|e| Msg::LevelNameChange(e))}
                                    />
                                    <div class={classes!("flex", "items-center", "justify-end", "w-full")}>
                                        <Button
                                            variant={ButtonVariant::Outlined}
                                            onclick={ctx.link().callback(|_| Msg::CloseAddPopup)}
                                            class="text-blue-400 border-zinc-500 hover:bg-zinc-600"
                                        >
                                            {"Annuleren"}
                                        </Button>
                                        <Button
                                            variant={ButtonVariant::Contained}
                                            onclick={ctx.link().callback(|_| Msg::SaveLevel)}
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
            </GameLayout>
        }
    }
}

impl Levels {
    fn render_level_row(&self, ctx: &Context<Self>, level: &Level) -> Html {
        let name = level.name.clone();
        let delete_level_id = level.id.to_string();
        let copy_level_id = level.id.to_string();

        html! {
            <TableRow>
                <TableCell>
                    {name}
                </TableCell>
                <TableCell>
                    <div class={classes!("w-full", "flex", "justify-end", "items-center")}>
                        <Button
                            dense={true}
                            title="Kopieer Level Id"
                            onclick={ctx.link().callback(move |_| Msg::CopyLevelId(copy_level_id.clone()))}
                        >
                            <Icon name="content_copy"/>
                        </Button>
                        <Button
                            dense={true}
                            title="Verwijder Level"
                            onclick={ctx.link().callback(move |_| Msg::DeleteLevel(delete_level_id.clone()))}
                        >
                            <Icon name="delete"/>
                        </Button>
                    </div>
                </TableCell>
            </TableRow>
        }
    }
}
