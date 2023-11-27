use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct ListItemIcon;

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemIconProps {
    pub children: Children,
}

impl Component for ListItemIcon {
    type Message = ();
    type Properties = ListItemIconProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ListItemIcon {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("text-white", "shrink-0", "inline-flex", "mr-2")}>
                {ctx.props().children.clone()}
            </div>
        }
    }
}