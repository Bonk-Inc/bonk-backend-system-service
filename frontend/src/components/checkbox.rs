use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Checkbox {
    checked: bool
}

#[derive(Clone, PartialEq, Properties)]
pub struct CheckboxProps {
    pub id: String,
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub class: String,
}

pub enum Msg {
    CheckboxClicked(Event)
}

impl Component for Checkbox {
    type Message = Msg;
    type Properties = CheckboxProps;

    fn create(ctx: &Context<Self>) -> Self {
        let checked = ctx.props().checked;

        Checkbox { checked }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let onchange = &ctx.props().onchange;

        match msg {
            Msg::CheckboxClicked(e) => { 
                self.checked = !self.checked;
                onchange.emit(e);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut base_class = vec!["inline-flex", "items-center", "justify-center", "relative", "box-border", "bg-transparent", "outline-0", "border-0", "m-0", "cursor-pointer", "select-none", "align-middle", "appearance-none", "no-underline", "p-2", "rounded-half"];
        let class = ctx.props().class.clone();

        if self.checked {
            base_class.append(&mut vec!["text-blue-300"])
        } else {
            base_class.append(&mut vec!["text-white/70"])
        }

        html! {
            <span class={classes!(base_class, class)}>
                <input
                    id={ctx.props().id.clone()}
                    class={classes!("cursor-inherit", "absolute", "opacity-0", "w-full", "h-full", "top-0", "left-0", "m-0", "p-0", "z-10")}
                    type="checkbox"
                    onchange={ctx.link().callback(|e| Msg::CheckboxClicked(e))}
                />
                <svg class={classes!("select-none", "w-[1em]", "h-[1em]", "inline-block", "fill-current", "shrink-0", "text-2xl", "transition-colors")} focusable="false" aria-hidden="true" viewBox="0 0 24 24">
                    {if self.checked {
                        html!(<path d="M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"></path>)
                    } else {
                        html!(<path d="M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z"></path>)
                    }}
                </svg>
            </span>
        }
    }
}