use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct AppBar;

#[derive(Clone, PartialEq, Properties)]
pub struct AppBarProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for AppBar {
    type Message = ();
    type Properties = AppBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        AppBar {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class={classes!("bg-zinc-800", "flex", "flex-col", "box-border", "w-full", "shrink-0", "fixed", "z-30", "top-0", "left-auto", "right-0", &ctx.props().class)}>
                {ctx.props().children.clone()}
            </header>
        }
    }
}