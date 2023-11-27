use yew::{
    prelude::*,
    Component,
    Context,
    Html
};

pub struct Spinner;

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnterProps {
    #[prop_or_default]
    pub class: String,
}

impl Component for Spinner {
    type Message = ();
    type Properties = SpinnterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Spinner {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("inline-flex", "items-center", &ctx.props().class)}>
                <svg class="animate-spin ml-1 mr-3 h-full w-full text-blue-300" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
            </div>  
        }
    }
}