use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Tabs;

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

impl Component for Tabs {
    type Message = ();
    type Properties = TabsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Tabs {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = ctx.props().class.clone();

        html! {
            <nav class={classes!("overflow-hidden", "min-h-48px", "flex", classes)}>
                <div class={classes!("relative", "inline-block", "flex-auto", "whitespace-nowrap", "w-full", "overflow-x-hidden")}>
                    <div class={classes!("flex")}>
                        {ctx.props().children.clone()}
                    </div>
                </div>
            </nav>
        }
    }
}