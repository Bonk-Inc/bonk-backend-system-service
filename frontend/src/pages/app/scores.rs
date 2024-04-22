use std::{
    cmp,
    collections::{HashMap, HashSet},
    mem,
};

use babs::{
    models::{Level, Score},
    respone::ResponseBody,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{prelude::*, Component, Context, Html};
use yew_router::scope_ext::RouterScopeExt;

use crate::{
    app::AppRoute,
    components::{
        alert::{Alert, Severity},
        button::{Button, ButtonVariant},
        checkbox::Checkbox,
        icon::Icon,
        select::Select,
        spinner::Spinner,
        table::Table,
        table_body::TableBody,
        table_cell::TableCell,
        table_container::TableContainer,
        table_footer::TableFooter,
        table_head::TableHead,
        table_pagination::TablePagination,
        table_row::TableRow,
    },
    env,
    layouts::game_layout::GameLayout,
    service::fetch::Fetch,
};

pub struct Scores {
    scores: Vec<Score>,
    levels: Vec<Level>,
    status: Status,
    selected_scores: HashSet<String>,
    row_per_page: usize,
    page: usize,
    filter_level: String
}

#[derive(Clone, PartialEq, Properties)]
pub struct ScoresProps {
    pub game_id: String,
}

pub enum Status {
    Fetching,
    Finished,
    Failed(String),
}

pub enum Msg {
    MakeReq,
    Response(Vec<Score>, Vec<Level>),
    SelectScore(String),
    SelectAllScores,
    UpdateScore(Score),
    UpdateResponse(Score),
    DeleteScores,
    DeleteScoresResponse,
    LevelFilterChange(String),
    NavigateToForm(Option<String>),
    RowsPerPageChange(usize),
    PageChange(usize),
    Failed(String),
}

impl Component for Scores {
    type Message = Msg;
    type Properties = ScoresProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);

        Scores {
            scores: Vec::new(),
            levels: Vec::new(),
            selected_scores: HashSet::new(),
            status: Status::Fetching,
            row_per_page: 10,
            page: 0,
            filter_level: String::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                let game_id = ctx.props().game_id.clone();
                let level_id = self.filter_level.clone();
                self.status = Status::Fetching;

                ctx.link().send_future(async move {
                    let scores_url = if level_id.len() > 0 {
                        format!(
                            "{}/api/score/level/{}?hidden=true",
                            env::APP_API_URL,
                            level_id
                        )
                    } else {
                        format!(
                            "{}/api/score/game/{}?hidden=true",
                            env::APP_API_URL,
                            game_id
                        )
                    };

                    let levels_url = format!("{}/api/level/game/{}", env::APP_API_URL, game_id);
                    let levels = Fetch::get(&levels_url, Some(true)).await;
                    if levels.is_err() {
                        return Msg::Failed("Failed fetching levels".to_string());
                    }

                    let scores = Fetch::get(&scores_url, Some(true)).await;
                    if scores.is_err() {
                        return Msg::Failed("Failed fetching scores".to_string());
                    }

                    let score_data: ResponseBody<Vec<Score>> =
                        serde_wasm_bindgen::from_value(scores.unwrap()).unwrap();
                    let level_data: ResponseBody<Vec<Level>> =
                        serde_wasm_bindgen::from_value(levels.unwrap()).unwrap();
                    Msg::Response(score_data.data, level_data.data)
                })
            }
            Msg::Response(scores, levels) => {
                self.scores = scores;
                self.levels = levels;
                self.status = Status::Finished;
            }
            Msg::SelectScore(score) => {
                if self.selected_scores.contains(&score) {
                    self.selected_scores.remove(&score);
                    return true;
                }

                self.selected_scores.insert(score.clone());
            }
            Msg::SelectAllScores => {
                let ids = self
                    .scores
                    .iter()
                    .map(|s| s.id.to_string())
                    .collect::<Vec<String>>();

                if self.selected_scores.len() == self.scores.len() {
                    self.selected_scores.clear();
                    return true;
                }

                self.selected_scores.extend(ids);
            }
            Msg::UpdateScore(new_score) => ctx.link().send_future(async move {
                let url = format!("{}/api/score/{}", env::APP_API_URL, new_score.id);
                let body = serde_json::to_string(&new_score).unwrap();

                let response = Fetch::put(&url, &body, Some(true)).await;
                if response.is_err() {
                    return Msg::Failed("An error occured during updating score".to_string());
                }

                let score: ResponseBody<Score> =
                    serde_wasm_bindgen::from_value(response.unwrap()).unwrap();
                Msg::UpdateResponse(score.data)
            }),
            Msg::UpdateResponse(new_score) => {
                let old_position = self
                    .scores
                    .iter()
                    .position(|s| s.id == new_score.id)
                    .unwrap();

                let _ = mem::replace(&mut self.scores[old_position], new_score.clone());
            }
            Msg::DeleteScores => {
                let ids = self
                    .selected_scores
                    .iter()
                    .map(|id| id.clone())
                    .collect::<Vec<String>>()
                    .join(",");

                ctx.link().send_future(async move {
                    let url = format!("{}/api/score/({})", env::APP_API_URL, ids);
                    let scores = Fetch::delete(&url, Some(true)).await;

                    if scores.is_err() {
                        return Msg::Failed("An error occured when deleting score".to_string());
                    }

                    Msg::DeleteScoresResponse
                })
            }
            Msg::DeleteScoresResponse => {
                self.scores = self
                    .scores
                    .iter()
                    .filter(|s| {
                        self.selected_scores
                            .iter()
                            .any(|id| *id != s.id.to_string())
                    })
                    .map(|s: &Score| s.clone())
                    .collect::<Vec<Score>>();

                self.selected_scores.clear();
            }
            Msg::NavigateToForm(score_id) => {
                let navigator = ctx.link().navigator().unwrap();
                let game_id = ctx.props().game_id.clone();

                if score_id.is_none() {
                    let _ = navigator
                        .push_with_query(&AppRoute::ScoreAdd, &HashMap::from([("game", game_id)]));
                } else {
                    let _ = navigator.push_with_query(
                        &AppRoute::ScoreEdit {
                            score_id: score_id.expect("Score Id"),
                        },
                        &HashMap::from([("game", game_id)]),
                    );
                }

                return false;
            }
            Msg::RowsPerPageChange(rows) => self.row_per_page = rows,
            Msg::PageChange(page) => self.page = page,
            Msg::Failed(error) => self.status = Status::Failed(error),
            Msg::LevelFilterChange(level_id) => {
                self.filter_level = level_id;
                ctx.link().send_message(Msg::MakeReq)
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_id = ctx.props().game_id.clone();
        let selected_score_len = self.selected_scores.len();
        let score_len = self.scores.len();
        let page = self.page;
        let row_per_page = self.row_per_page;

        let scores = &self.scores
            [page * row_per_page..cmp::min(page * row_per_page + row_per_page, self.scores.len())];

        html! {
            <GameLayout id={game_id}>
                <div class="py-4 flex justify-between items-center">
                    {if self.selected_scores.len() > 0 {
                        html! {
                            <Button dense={true} onclick={ctx.link().callback(|_| Msg::DeleteScores)} class="text-red-400">
                                <Icon name="delete"/>
                            </Button>
                        }
                    } else {
                        html! {
                            <form>
                                <Select
                                    id="filter-level"
                                    label="Level: "
                                    name="filter-level"
                                    class="border-none !flex-row items-baseline !pl-2"
                                    onchange={ctx.link().callback(|e: Event| {
                                        let target = e.target().unwrap();
                                        let input = target.dyn_ref::<HtmlSelectElement>().unwrap();
                                        Msg::LevelFilterChange(input.value())
                                    })}
                                >
                                    <option selected={true} value={""}>{"Alle"}</option>
                                    { for self.levels.iter().map(|level| html! {
                                        <option value={level.id.to_string()}>
                                            {level.name.clone()}
                                        </option>
                                    }) }
                                </Select>
                            </form>
                        }
                    }}
                    <Button
                        class="bg-blue-400 inline-flex items-center"
                        onclick={ctx.link().callback(|_| Msg::NavigateToForm(None))}
                        variant={ButtonVariant::Contained}
                    >
                        <Icon name="add" class="mr-2"/> {"Score toevoegen"}
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
                                            <TableCell checkbox={true}>
                                                <Checkbox
                                                    id={"select-all"}
                                                    indeterminate={selected_score_len > 0 && selected_score_len != score_len}
                                                    checked={selected_score_len > 0 && selected_score_len == score_len}
                                                    onchange={ctx.link().callback(|_| Msg::SelectAllScores)}
                                                />
                                            </TableCell>
                                            <TableCell>
                                                {"Username"}
                                            </TableCell>
                                            <TableCell>
                                                {"Score"}
                                            </TableCell>
                                            <TableCell>
                                                {"Gezet op"}
                                            </TableCell>
                                            <TableCell>
                                                {"Verborgen"}
                                            </TableCell>
                                            <TableCell checkbox={true}>
                                                <Button
                                                    class="flex justify-end w-full"
                                                    onclick={ctx.link().callback(|_| Msg::MakeReq)}
                                                >
                                                    <Icon name="refresh"/>
                                                </Button>
                                            </TableCell>
                                        </TableRow>
                                    </TableHead>
                                    <TableBody>
                                        { for scores.iter().map(|score| self.render_score_row(ctx, score)) }
                                    </TableBody>
                                    <TableFooter>
                                        <TableRow>
                                            <TablePagination
                                                col_span={6}
                                                rows_per_page={row_per_page}
                                                count={self.scores.len()}
                                                page={page}
                                                on_page_change={ctx.link().callback(|page| Msg::PageChange(page))}
                                                on_rows_per_page_change={ctx.link().callback(|e: Event| {
                                                    let target = e.target().unwrap();
                                                    let input = target.dyn_ref::<HtmlSelectElement>().unwrap();

                                                    Msg::RowsPerPageChange(input.value().parse::<usize>().unwrap())
                                                })}
                                            />
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

impl Scores {
    fn render_score_row(&self, ctx: &Context<Self>, score: &Score) -> Html {
        let score_id = score.id.to_string();
        let update_id = score.id.to_string();
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
                <TableCell>
                    <Button
                        dense={true}
                        class="flex justify-end w-full"
                        onclick={ctx.link().callback(move |_| Msg::NavigateToForm(Some(update_id.clone())))}
                    >
                        <Icon name="edit"/>
                    </Button>
                </TableCell>
            </TableRow>
        }
    }
}
