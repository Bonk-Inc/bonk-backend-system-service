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

pub struct TableFooter;

#[derive(Clone, PartialEq, Properties)]
pub struct TableFooterProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for TableFooter {
    type Message = ();
    type Properties = TableFooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TableFooter { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.clone();
        let context = TableContext {
            variant: TableContextVariant::Footer,
        };

        html! {
            <TableContext context={context}>
                <tfoot class={classes!("table-footer-group", class)}>
                    {ctx.props().children.clone()}
                </tfoot>
            </TableContext>
        }
    }
}