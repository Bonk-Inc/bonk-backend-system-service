use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct FormLabel;

#[derive(Clone, PartialEq, Properties)]
pub struct FormLabelProps {
    pub children: Children,
    pub html_for: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub error: bool,
    #[prop_or_default]
    pub class: String,
}

impl Component for FormLabel {
    type Message = ();
    type Properties = FormLabelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        FormLabel {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text_color = if ctx.props().error { "text-red-400" } else { "text-current" };
        let classes = ctx.props().class.clone();

        html! {
            <label
                class={classes!("leading-6", "p-0", "w-fit", "relative", "font-medium", text_color, classes)}
                for={ctx.props().html_for.clone()}
            >
                {ctx.props().children.clone()}
                {if ctx.props().required {
                    html!(<span class="text-inherit">{"*"}</span>)
                    
                } else { html!() }}
            </label>
        }
    }
}