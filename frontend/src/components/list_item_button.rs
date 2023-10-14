use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct ListItemButton;

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub class: String,
}

impl Component for ListItemButton {
    type Message = ();
    type Properties = ListItemButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ListItemButton {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut base_classes = vec!["bg-transparent", "outline-0", "border-0", "m-0", "rounded-none", "cursor-pointer", "select-none", "appearance-none", "text-inherit", "flex", "grow", "justify-start", "items-center", "relative", "no-underline", "min-w-0", "box-border", "text-left", "transition-colors", "px-4", "py-2", "hover:appearance-none", "hover:bg-zinc-700"];
        if ctx.props().selected {
            base_classes[0] = "bg-slate-700";
        }

        html! {
            <div class={classes!(base_classes, &ctx.props().class)}>
                {ctx.props().children.clone()}            
            </div>
        }
    }
}