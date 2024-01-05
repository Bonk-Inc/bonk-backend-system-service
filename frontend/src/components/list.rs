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
            <ul class={classes!("relative", "py-2", &ctx.props().class)}>
                {ctx.props().children.clone()}
            </ul>
        }
    }
}