use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

use crate::components::{
    icon::Icon,
    paper::Paper
};

pub struct StatsCard;

#[derive(Clone, PartialEq, Properties)]
pub struct StatsCardProps {
    pub name: String,
    pub icon: String,
    pub value: i64,
    #[prop_or_default]
    pub class: String
}

impl Component for StatsCard {
    type Message = ();
    type Properties = StatsCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        StatsCard {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
             <Paper class={format!("flex items-center justify-between m-3 w-[260px] {}", ctx.props().class)}>
                <div class={classes!("text-blue-300")}>
                    <Icon name={ctx.props().icon.clone()} class="text-4xl" />
                </div>
                <div>
                    <p class={classes!("mb-3", "text-right", "text-base")}>
                        {&ctx.props().name}
                    </p>
                    <p class={classes!("mt-3", "text-right", "font-medium", "text-xl", "text-blue-300")}>
                        {ctx.props().value}
                    </p>
                </div>
            </Paper>
        }
    }
}