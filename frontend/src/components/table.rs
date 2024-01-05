use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Table;

#[derive(Clone, PartialEq, Properties)]
pub struct TableProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Table { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.clone();

        html! {
            <table class={classes!("table", "w-full", "border-collapse", "border-spacing-0", class)}>
                {ctx.props().children.clone()}
            </table>
        }
    }
}