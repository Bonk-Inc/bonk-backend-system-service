use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::{button::Button, icon::Icon};

pub struct Tab;

#[derive(Clone, PartialEq, Properties)]
pub struct TabProps {
    pub label: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub icon: String,
    pub onclick: Callback<MouseEvent>
}

impl Component for Tab {
    type Message = ();
    type Properties = TabProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Tab { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.props().onclick.clone();
        let icon = ctx.props().icon.clone();

        let selected = if ctx.props().selected { "shadow-inner-b-solid shadow-blue-500 text-blue-500" } else { "" };
        let flex_direction = if icon.len() > 0 { "flex-row" } else { "flex-col" };

        html! { 
            <Button
                class={format!("min-h-48px !inline-flex items-center justify-center border-0 box-border bg-transparent outline-0 m-0 select-none relative text-center max-w-[360px] shrink-0 py-3 overflow-hidden !rounded-none {} {} hover:bg-zinc-700", flex_direction, selected)}
                onclick={Callback::from(move |e| { onclick.emit(e); })}
            >
                {if icon.len() > 0 {
                    html!(<Icon name={icon} class="mr-1" />)
                } else { html!() }}
                {ctx.props().label.clone()}
            </Button>
        }
    }
}