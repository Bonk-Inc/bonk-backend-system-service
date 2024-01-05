use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::icon::Icon;

pub struct Alert;

#[derive(Clone, PartialEq)]
pub enum Severity {
    Success,
    Info,
    Warning,
    Error
}

#[derive(Clone, PartialEq, Properties)]
pub struct AlertProps {
    pub children: Children,
    pub severity: Severity,
}

impl Component for Alert {
    type Message = ();
    type Properties = AlertProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Alert { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut base_classes = vec!["px-4", "py-3", "mb-6", "font-medium", "w-full", "rounded", "inline-flex", "items-center"];
        let icon = match ctx.props().severity {
            Severity::Success => "task_alt",
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
        };

        match ctx.props().severity {
            Severity::Success => base_classes.append(&mut vec!["bg-green-600", "text-black"]),
            Severity::Info => base_classes.append(&mut vec!["bg-blue-500", "text-black"]),
            Severity::Warning => base_classes.append(&mut vec!["bg-amber-600", "text-black"]),
            Severity::Error => base_classes.append(&mut vec!["bg-red-600", "text-white"]),
        };

        html! {
            <div class={classes!(base_classes)} role="alert">
                <Icon name={icon} class="mr-4" />
                {ctx.props().children.clone()}
            </div>
        }
    }
}