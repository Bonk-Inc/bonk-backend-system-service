use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Paper;

#[derive(Clone, PartialEq, Properties)]
pub struct PaperProps {
    pub children: Children,
    #[prop_or(PaperElevation::Flat)]
    pub elevation: PaperElevation,
    #[prop_or_default]
    pub class: String,

}

#[derive(Clone, PartialEq)]
pub enum PaperElevation {
    Flat,
    Elevated
}

impl Component for Paper {
    type Message = ();
    type Properties = PaperProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Paper {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut base_classes = vec!["p-8", "text-slate-200"];
        match ctx.props().elevation {
            PaperElevation::Flat => base_classes.append(&mut vec!["border", "rounded", "border-solid"]),
            PaperElevation::Elevated => base_classes.append(&mut vec!["shadow-lg", "bg-zinc-700"]),
        }

        html! {
            <div class={classes!(base_classes, &ctx.props().class)}>
                {ctx.props().children.clone()}
            </div>
        }
    }
}