use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Input;

#[derive(Clone, PartialEq, Properties)]
pub struct InputProps {
    pub id: String,
    pub name: String,
    pub onchange: Callback<Event>,
    #[prop_or("text".to_string())]
    pub html_type: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub error: bool,
    #[prop_or_default]
    pub class: String,
}

impl Component for Input {
    type Message = ();
    type Properties = InputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Input {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = ctx.props().class.clone();
        let onchange = ctx.props().onchange.clone();
        let text_color = if ctx.props().error { "text-red-400 border-red-400 focus:shadow-red-400" } else { "text-current border-white focus:shadow-blue-400 focus:border-blue-400" };
        let width = if ctx.props().full_width { "w-full" } else { "w-auto" };

        html! {
            <input
                id={format!("input-{}", ctx.props().id)}
                name={ctx.props().name.clone()}
                class={classes!("relative", "bg-transparent", "leading-6", "box-border", "cursor-text", "inline-flex", "items-center", "border", "px-4", "py-2", "focus:outline-none", "shadow-inner-solid", "border-solid", "rounded", text_color, width, classes)}
                required={ctx.props().required}
                type={ctx.props().html_type.clone()}
                placeholder={ctx.props().placeholder.clone()}
                onchange={Callback::from(move |e| { onchange.emit(e); })}
            />
        }
    }
}