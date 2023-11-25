use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct TableRow;

#[derive(Clone, PartialEq, Properties)]
pub struct TableRowProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for TableRow {
    type Message = ();
    type Properties = TableRowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TableRow { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.clone();

        html! {
            <tr class={classes!("text-inherit", "table-row", "align-middle", "outline-0", "hover:bg-zinc-700", class)}>
                {ctx.props().children.clone()}
            </tr>
        }
    }
}