use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct FormControl;

#[derive(Clone, PartialEq, Properties)]
pub struct FormControlProps {
    pub children: Children,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub class: String,
}

impl Component for FormControl {
    type Message = ();
    type Properties = FormControlProps;

    fn create(_ctx: &Context<Self>) -> Self {
        FormControl {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let width = if ctx.props().full_width { "w-full" } else { "w-auto" };
        let classes = ctx.props().class.clone();

        html! {
            <div class={classes!("inline-flex", "flex-col", "relative", "min-w-0", "p-0", "m-0", "border-0", "align-top", width, classes)}>
                {ctx.props().children.clone()}
            </div>
        }
    }
}