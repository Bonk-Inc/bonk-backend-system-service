use babs::{models::Score, respone::ResponseBody};
use yew::{html, Component, Context, Html, Properties};

use crate::{
    components::{
        table::Table, table_body::TableBody, table_cell::TableCell,
        table_container::TableContainer, table_head::TableHead,
        table_row::TableRow,
    },
    layouts::game_layout::GameLayout,
    service::fetch::Fetch,
};

pub struct Scores {
    scores: Vec<Score>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ScoresProps {
    pub game_id: String,
}

pub enum Msg {
    MakeReq,
    Response(Vec<Score>),
    Nothing,
    Failed,
}

impl Component for Scores {
    type Message = Msg;
    type Properties = ScoresProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);

        Scores { scores: Vec::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match &msg {
            Msg::MakeReq => {
                let game_id = ctx.props().game_id.clone();

                ctx.link().send_future(async move {
                    let url = format!(
                        "http://localhost:8080/api/score/game/{}?hidden=true",
                        game_id
                    );
                    let scores = Fetch::get(&url, Some(true)).await;
                    if scores.is_err() {
                        return Msg::Failed;
                    }

                    let stats_data: ResponseBody<Vec<Score>> =
                        serde_wasm_bindgen::from_value(scores.unwrap()).unwrap();
                    Msg::Response(stats_data.data)
                })
            }
            Msg::Response(scores) => {
                self.scores = scores.clone();
            }
            Msg::Nothing => todo!(),
            Msg::Failed => todo!(),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();

        html! {
            <GameLayout id={game_id}>
                <TableContainer>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableCell>
                                    {"User"}
                                </TableCell>
                                <TableCell>
                                    {"Score"}
                                </TableCell>
                                <TableCell>
                                    {"Set at"}
                                </TableCell>
                                <TableCell>
                                    {"Hidden"}
                                </TableCell>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            { for self.scores.iter().map(|score| self.render_score_row(score)) }
                        </TableBody>
                    </Table>
                </TableContainer>
            </GameLayout>
        }
    }
}

impl Scores {
    fn render_score_row(&self, score: &Score) -> Html {
        let set_at = if let Some(updated_at) = score.updated_at {
            updated_at
        } else {
            score.created_at
        };

        html! {
            <TableRow>
                <TableCell>{score.username.clone()}</TableCell>
                <TableCell>{score.highscore}</TableCell>
                <TableCell>
                    {set_at.format("%Y-%m-%d %H:%M:%S").to_string()}
                </TableCell>
                <TableCell>{score.is_hidden}</TableCell>
            </TableRow>
        }
    }
}
