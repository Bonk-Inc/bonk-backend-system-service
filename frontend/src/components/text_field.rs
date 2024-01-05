use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::{
    form_control::FormControl, 
    form_label::FormLabel, 
    input::Input
};

pub struct TextField;

#[derive(Clone, PartialEq, Properties)]
pub struct TextFieldProps {
    pub id: String,
    pub name: String,
    pub label: String,
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
    #[prop_or_default]
    pub value: String
}

impl Component for TextField {
    type Message = ();
    type Properties = TextFieldProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TextField {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone();
        let full_width = ctx.props().full_width;
        let required = ctx.props().required;
        let error = ctx.props().error;

        html! {
            <FormControl full_width={full_width} class={ctx.props().class.clone()}>
                <FormLabel class="mb-1" html_for={format!("input-{}", id)} required={required} error={error}>
                    {ctx.props().label.clone()}
                </FormLabel>
                <Input
                    id={id}
                    name={ctx.props().name.clone()}
                    onchange={ctx.props().onchange.clone()}
                    required={required}
                    error={error}
                    value={ctx.props().value.clone()}
                    html_type={ctx.props().html_type.clone()}
                    placeholder={ctx.props().placeholder.clone()}
                    full_width={full_width}
                />
            </FormControl>
        }
    }
}