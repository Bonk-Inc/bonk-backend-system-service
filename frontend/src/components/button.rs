use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Button;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or(ButtonVariant::Text)]
    pub variant: ButtonVariant,
    #[prop_or(ButtonSize::Normal)]
    pub size: ButtonSize,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub dense: bool,
    pub onclick: Callback<MouseEvent>
}

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Text,
    Outlined,
    Contained
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Normal,
    Large
}


impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Button {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.props().onclick.clone();
        let class = ctx.props().class.clone();
        let mut base_classes = vec!["text-center", "font-medium", "rounded", "block", "min-w-[64px]", "transition-colors"];

        match ctx.props().size {
            ButtonSize::Small => base_classes.append(&mut vec!["py-1", "px-3"]),
            ButtonSize::Normal => base_classes.append(&mut vec!["py-2", "px-4"]),
            ButtonSize::Large => base_classes.append(&mut vec!["py-2", "px-6"])
        }

        match ctx.props().variant {
            ButtonVariant::Outlined => base_classes.append(&mut vec!["border", "border-solid"]),
            ButtonVariant::Contained => base_classes.append(&mut vec!["text-zinc-800"]),
            _ => ()
        }

        if ctx.props().dense {
            base_classes.append(&mut vec!["!py-0"])
        }

        html! {
            <button 
                onclick={Callback::from(move |e| { onclick.emit(e); })}
                class={classes!(base_classes, class)}
                type={"button"}
            >
                {ctx.props().children.clone()}
            </button>
        }
    }
}