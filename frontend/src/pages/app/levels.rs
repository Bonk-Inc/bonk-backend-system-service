use babs::{models::Level, respone::ResponseBody};
use yew::{classes, html, Component, Context, Html, Properties};

use crate::{
    components::{
        alert::{Alert, Severity},
        button::{Button, ButtonVariant},
        icon::Icon,
        spinner::Spinner,
        table::Table,
        table_body::TableBody,
        table_cell::TableCell,
        table_container::TableContainer,
        table_head::TableHead,
        table_row::TableRow,
    },
    env,
    layouts::game_layout::GameLayout,
    service::fetch::Fetch,
};

pub struct Levels {
    status: Status,
    levels: Vec<Level>,
}

pub enum Status {
    Fetching,
    Finished,
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
                self.levels = self.levels
                    .iter()
                    .filter(|l| l.id.to_string() != level_id)
                    .map(|l| l.clone())
                    .collect::<Vec<Level>>();
            }
            Msg::Failed(error) => self.status = Status::Failed(error),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();

        html! {
            <GameLayout id={game_id}>
                <div class="py-4 flex justify-between items-center">
                    <Button
                        class="bg-blue-400 inline-flex items-center"
                        onclick={ctx.link().callback(|_| Msg::NavigateToForm(None))}
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
            </GameLayout>
        }
    }
}

impl Levels {
    fn render_level_row(&self, ctx: &Context<Self>, level: &Level) -> Html {
        let name = level.name.clone();
        let level_id = level.id.to_string();

        html! {
            <TableRow>
                <TableCell>
                    {name}
                </TableCell>
                <TableCell>
                    <Button
                        dense={true}
                        class="flex justify-end w-full"
                        onclick={ctx.link().callback(move |_| Msg::DeleteLevel(level_id.clone()))}
                    >
                        <Icon name="delete"/>
                    </Button>
                </TableCell>
            </TableRow>
        }
    }
}
