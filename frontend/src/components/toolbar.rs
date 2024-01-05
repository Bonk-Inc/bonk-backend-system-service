use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Toolbar;

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for Toolbar {
    type Message = ();
    type Properties = ToolbarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Toolbar {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("min-h-[56px]", "relative", "flex", "items-center", "px-6", &ctx.props().class)}>
                {ctx.props().children.clone()}
            </div>
        }
    }
}