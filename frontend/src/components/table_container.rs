use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct TableContainer;

#[derive(Clone, PartialEq, Properties)]
pub struct TableContainerProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for TableContainer {
    type Message = ();
    type Properties = TableContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TableContainer { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.clone();

        html! {
            <div class={classes!("w-full", "overflow-x-auto", class)}>
                {ctx.props().children.clone()}
            </div>
        }
    }
}