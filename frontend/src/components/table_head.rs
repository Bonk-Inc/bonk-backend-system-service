use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::table_context::{TableContext, TableContextVariant};

pub struct TableHead;

#[derive(Clone, PartialEq, Properties)]
pub struct TableHeadProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for TableHead {
    type Message = ();
    type Properties = TableHeadProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TableHead { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.clone();
        let context = TableContext {
            variant: TableContextVariant::Head,
        };

        html! {
            <TableContext context={context}>
                <thead class={classes!("table-header-group", class)}>
                    {ctx.props().children.clone()}
                </thead>
            </TableContext>
        }
    }
}