use std::{collections::HashSet, mem};

use babs::{models::Score, respone::ResponseBody};
use yew::{html, Component, Context, Html, Properties, classes};

use crate::{
    components::{
        table::Table, table_body::TableBody, table_cell::TableCell,
        table_container::TableContainer, table_head::TableHead,
        table_row::TableRow, checkbox::Checkbox, spinner::Spinner,
    },
    layouts::game_layout::GameLayout,
    service::fetch::Fetch,
};

pub struct Scores {
    scores: Vec<Score>,
    status: Status,
    selected_scores: HashSet<String>
}

#[derive(Clone, PartialEq, Properties)]
pub struct ScoresProps {
    pub game_id: String,
}

pub enum Status {
    Fetching,
    Finished,
}

pub enum Msg {
    MakeReq,
    Response(Vec<Score>),
    SelectScore(String),
    SelectAllScores,
    UpdateScore(Score),
    UpdateResponse(Score),
    Failed,
}

impl Component for Scores {
    type Message = Msg;
    type Properties = ScoresProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);

        Scores { 
            scores: Vec::new(),
            selected_scores: HashSet::new(),
            status: Status::Fetching
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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

                    let stats_data: ResponseBody<Vec<Score>> = serde_wasm_bindgen::from_value(scores.unwrap()).unwrap();
                    Msg::Response(stats_data.data)
                })
            }
            Msg::Response(scores) => {
                self.scores = scores.clone();
                self.status = Status::Finished;
            },
            Msg::SelectScore(score) => {
                if self.selected_scores.contains(&score) {
                    self.selected_scores.remove(&score);
                    return true;
                }

                self.selected_scores.insert(score.clone());
            },
            Msg::SelectAllScores => {
                let ids = self.scores.iter()
                    .map(|s| s.id.to_string())
                    .collect::<Vec<String>>();

                if self.selected_scores.len() == self.scores.len() {
                    self.selected_scores.clear();
                    return true;
                }

                self.selected_scores.extend(ids);
            },
            Msg::UpdateScore(new_score) => {
                ctx.link().send_future(async move {
                    let url = format!("http://localhost:8080/api/score/{}", new_score.id);
                    let body = serde_json::to_string(&new_score).unwrap();

                    let response = Fetch::put(&url, &body, Some(true)).await;
                    if response.is_err() {
                        return Msg::Failed;
                    }

                    let score: ResponseBody<Score> = serde_wasm_bindgen::from_value(response.unwrap()).unwrap();
                    Msg::UpdateResponse(score.data)
                })
            },
            Msg::UpdateResponse(new_score) => {
                let old_position = self.scores.iter()
                    .position(|s| s.id == new_score.id)
                    .unwrap();

                let _ = mem::replace(&mut self.scores[old_position], new_score.clone());
            },
            Msg::Failed => todo!(),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();
        let selected_score_len = self.selected_scores.len();
        let score_len = self.scores.len();

        html! {
            <GameLayout id={game_id}>
                <TableContainer>
                    {match self.status {
                        Status::Fetching => html! {
                            <div class={classes!("flex", "justify-center", "items-center", "h-full")}>
                                <Spinner class="w-20 h-20" />
                            </div>
                        },
                        Status::Finished => html! {
                            <Table>
                                <TableHead>
                                    <TableRow>
                                        <TableCell checkbox={true}>
                                            <Checkbox 
                                                id={"select-all"}
                                                indeterminate={selected_score_len > 0 && selected_score_len != score_len}
                                                checked={selected_score_len > 0 && selected_score_len == score_len}
                                                onchange={ctx.link().callback(|_| Msg::SelectAllScores)}
                                            />
                                        </TableCell>
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
                                    { for self.scores.iter().map(|score| self.render_score_row(ctx, score)) }
                                </TableBody>
                            </Table>
                        }
                    }}
                </TableContainer>
            </GameLayout>
        }
    }
}

impl Scores {
    fn render_score_row(&self, ctx: &Context<Self>, score: &Score) -> Html {
        let score_id = score.id.to_string();
        let old_score = score.to_owned();
        let set_at = if let Some(updated_at) = score.updated_at {
            updated_at
        } else {
            score.created_at
        };

        html! {
            <TableRow>
                <TableCell checkbox={true}>
                    <Checkbox 
                        id={format!("select-score-{}", &score_id)}
                        checked={self.selected_scores.contains(&score_id)}
                        onchange={ctx.link().callback(move |_| Msg::SelectScore(score_id.clone()))}
                    />
                </TableCell>
                <TableCell>{score.username.clone()}</TableCell>
                <TableCell>{score.highscore}</TableCell>
                <TableCell>
                    {set_at.format("%Y-%m-%d %H:%M:%S").to_string()}
                </TableCell>
                <TableCell checkbox={true}>
                    <Checkbox
                        checked={score.is_hidden}
                        id={format!("score-hidden-{}", score.id.to_string())}
                        onchange={ctx.link().callback(move |_| {
                            let mut edited_score = old_score.clone();
                            edited_score.is_hidden = !old_score.is_hidden;

                            Msg::UpdateScore(edited_score)
                        })}
                    />
                </TableCell>
            </TableRow>
        }
    }
}
