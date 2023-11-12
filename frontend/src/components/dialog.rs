use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::paper::{Paper, PaperElevation};

pub struct Dialog;

#[derive(Clone, PartialEq, Properties)]
pub struct DialogProps {
    pub children: Children,
    pub open: bool,
    #[prop_or_default]
    pub class: String
}

impl Component for Dialog {
    type Message = ();
    type Properties = DialogProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Dialog {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden = if ctx.props().open {
            "block"
        } else {
            "hidden"
        };

        html! {
            <div class={classes!("z-50", "inset-0", "fixed", hidden)} role="presentation">
                <div class={classes!("-z-10", "inset-0", "fixed", "flex", "items-center", "bg-black", "opacity-50", "justify-center")} />
                <div class={classes!("h-full", "outline-0", "flex", "items-center", "justify-center")} tabindex="-1" role="presentation">
                    <Paper elevation={PaperElevation::Elevated}>
                        {ctx.props().children.clone()}
                    </Paper>
                </div>
            </div>
        }
    }
}