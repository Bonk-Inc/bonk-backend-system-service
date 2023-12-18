use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::{form_control::FormControl, form_label::FormLabel};

pub struct Select;

#[derive(Clone, PartialEq, Properties)]
pub struct SelectProps {
    pub children: Children,
    pub label: String,
    pub name: String,
    pub id: String,
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub error: bool
}

impl Component for Select {
    type Message = ();
    type Properties = SelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Select {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = ctx.props().class.clone();
        let full_width = ctx.props().full_width;
        let required = ctx.props().required;
        let error = ctx.props().error;
        let onchange = ctx.props().onchange.clone();
        let id = ctx.props().id.clone();
        let name = ctx.props().name.clone();
        
        let text_color = if error { "text-red-400 border-red-400 focus:shadow-red-400" } else { "text-current border-white focus:shadow-blue-300 focus:border-blue-300" };
        let width = if full_width { "w-full" } else { "w-auto" };

        html! {
            <FormControl full_width={full_width} class={classes.clone()}>
                <FormLabel class="mb-1" html_for={format!("input-{}", id)} required={required} error={error}>
                    {ctx.props().label.clone()}
                </FormLabel>
                <select 
                    class={classes!("relative", "bg-transparent", "leading-6", "box-border", "cursor-text", "inline-flex", "items-center", "border", "px-4", "py-2", "focus:outline-none", "shadow-inner-solid", "border-solid", "rounded", text_color, width, classes)}
                    name={name} 
                    required={required}
                    id={id}
                    onchange={Callback::from(move |e| onchange.emit(e))}
                >
                    {ctx.props().children.clone()}
                </select>
            </FormControl>
        }
    }
}