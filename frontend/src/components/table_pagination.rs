use std::cmp;

use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::{
    button::Button, icon::Icon, table_cell::TableCell, toolbar::Toolbar
};

pub struct TablePagination;

#[derive(Clone, PartialEq, Properties)]
pub struct TablePaginationProps {
    pub col_span: usize,
    pub rows_per_page: usize,
    #[prop_or(vec![10, 25, 50])]
    pub rows_per_page_option: Vec<usize>,
    pub count: usize,
    pub page: usize,
    pub on_rows_per_page_change: Callback<Event>,
    pub on_page_change: Callback<usize>,
    #[prop_or_default]
    pub class: String,
}

impl Component for TablePagination {
    type Message = ();
    type Properties = TablePaginationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TablePagination { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.clone();
        let colspan = ctx.props().col_span;
        let count = ctx.props().count;
        let page = ctx.props().page;
        let rows_per_page = ctx.props().rows_per_page;
        let rows_per_page_option = ctx.props().rows_per_page_option.clone();
        let on_rows_per_page_change = ctx.props().on_rows_per_page_change.clone();

        let on_page_change_prev = ctx.props().on_page_change.clone();
        let on_page_change_next = ctx.props().on_page_change.clone();

        let from = if count == 0 { 0 } else { page * rows_per_page + 1 };
        let to = if count == 0 { 
            (page + 1) * rows_per_page 
        } else {
            cmp::min(count, (page + 1) * rows_per_page)
        };

        html! {
            <TableCell class={format!("overflow-auto !p-0 {}", class)} col_span={colspan}>
                <Toolbar class={"min-h-[52px]"}>
                    <div class={classes!("flex-1", "basis-full")} />
                    {if rows_per_page_option.len() > 1 {
                        html! {
                            <>
                                <p class={classes!("shrink-0")}>
                                    {"Rows per page:"}
                                </p>
                                <div class={classes!("relative", "box-border", "inline-flex", "items-center", "mr-8", "ml-4", )}>
                                    <select 
                                        class={classes!("relative", "bg-transparent", "leading-6", "box-border", "cursor-text", "inline-flex", "items-center", "focus:outline-none")}
                                        onchange={Callback::from(move |e| on_rows_per_page_change.emit(e))}
                                    >
                                        { for rows_per_page_option.iter().map(|r| html!(<option value={r.to_string()} selected={r == &rows_per_page}>{r}</option>)) }
                                    </select>
                                </div>
                            </>
                        }
                    } else { html!() }}
                    <p class={classes!("shrink-0")}>
                        {format!("{}-{} of {}", from, to, count)}
                    </p>
                    <div class={classes!("flex")}>
                        <Button 
                            class="!pr-0"
                            dense={true}
                            disabled={page == 0}
                            onclick={Callback::from(move |_| { on_page_change_prev.emit(page.saturating_sub(1)) })}
                        >
                            <Icon name={"chevron_left"} />
                        </Button>
                        <Button
                            class="!pl-0"
                            dense={true}
                            disabled={page == count.div_ceil(rows_per_page) - 1}
                            onclick={Callback::from(move |_| { on_page_change_next.emit(page + 1) })}
                        >
                            <Icon name={"chevron_right"} />
                        </Button>
                    </div>
                </Toolbar>
            </TableCell>
        }
    }
}