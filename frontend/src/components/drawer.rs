use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::paper::Paper;

pub struct Drawer;

#[derive(Clone, PartialEq, Properties)]
pub struct DrawerProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for Drawer {
    type Message = ();
    type Properties = DrawerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Drawer {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <aside class={classes!("grow-0", "z-20", "shrink-0", "basis-auto", "w-[240px]", &ctx.props().class)}>
                <Paper square={true} class="overflow-y-auto border-b-0 border-l-0 h-full flex flex-col grow shrink-0 basis-auto !p-0 fixed t-0 outline-0 w-[240px]">
                    {ctx.props().children.clone()}
                </Paper>
            </aside>
        }
    }
}