use babs::{models::Level, respone::ResponseBody};
use yew::{classes, html, Component, Context, Html, Properties};

use crate::{
    components::{
        alert::{Alert, Severity},
        spinner::Spinner,
        table::Table,
        table_body::TableBody,
        table_cell::TableCell,
        table_container::TableContainer,
        table_footer::TableFooter,
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
            Msg::Failed(error) => self.status = Status::Failed(error),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();

        html! {
            <GameLayout id={game_id}>
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
                                        </TableRow>
                                    </TableHead>
                                    <TableBody>
                                        { for self.levels.iter().map(|level| self.render_level_row(level)) }
                                    </TableBody>
                                    <TableFooter>
                                        <TableRow>
                                            {""}
                                        </TableRow>
                                    </TableFooter>
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
    fn render_level_row(&self, level: &Level) -> Html {
        let name = level.name.clone();

        html! {
            <TableRow>
                <TableCell>
                    {name}
                </TableCell>
            </TableRow>
        }
    }
}
