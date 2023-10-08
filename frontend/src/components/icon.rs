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
            <i class={classes!("material-symbols-outlined", "px-4", "text-current")}>
                {&ctx.props().name}
            </i>
        }
    }
}