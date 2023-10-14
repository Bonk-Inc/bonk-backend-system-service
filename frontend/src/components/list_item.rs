use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct ListItem;

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for ListItem {
    type Message = ();
    type Properties = ListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ListItem {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li class={classes!("flex", "justify-start", "items-center", "relative", "no-underline", "w-full", "box-border", "text-left")}>
                {ctx.props().children.clone()}
            </li>
        }
    }
}