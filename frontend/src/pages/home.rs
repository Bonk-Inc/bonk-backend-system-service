use yew::{Component, html, Html, Context};

use crate::layouts::main_layout::MainLayout;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
    
        Home { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MainLayout>
                
            </MainLayout>
        }
    }
}