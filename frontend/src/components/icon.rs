use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Icon;

#[derive(PartialEq, Properties)]
pub struct IconProps {
    pub name: String,
}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Icon {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <i class={classes!("material-symbols-outlined", "select-none", "inline-block", "text-current", "shrink-0", "w-[1em]", "h-[1em]", "transition-colors", "duration-200")}>
                {&ctx.props().name}
            </i>
        }
    }
}