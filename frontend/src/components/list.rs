use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct List;

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        List {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul class={classes!("list-none", "m-0", "px-0", "py-2", "relative", &ctx.props().class)}>
                {ctx.props().children.clone()}
            </ul>
        }
    }
}