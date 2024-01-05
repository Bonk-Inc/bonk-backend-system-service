use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::table_context::{
    TableContext, 
    TableContextVariant
};

pub struct TableBody;

#[derive(Clone, PartialEq, Properties)]
pub struct TableBodyProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for TableBody {
    type Message = ();
    type Properties = TableBodyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TableBody { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.clone();
        let context = TableContext {
            variant: TableContextVariant::Body,
        };

        html! {
            <TableContext context={context}>
                <tbody class={classes!("table-row-group", class)}>
                    {ctx.props().children.clone()}
                </tbody>
            </TableContext>
        }
    }
}