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
            <header 
                style="width: calc(100% - 240px);"
                class={classes!("bg-zinc-800", "flex", "flex-col", "box-border", "ml-[240px]", "shrink-0", "fixed", "z-30", "t-0", "l-auto", "r-0", "border-b", "border-zinc-500", "border-solid")}>
                {ctx.props().children.clone()}
            </header>
        }
    }
}