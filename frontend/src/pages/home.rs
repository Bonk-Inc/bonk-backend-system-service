use yew::{Component, html, Html, Context, classes};

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
                <div class={classes!("p-4")}>
                    <h1 class={classes!("text-2xl", "font-medium")}>
                        {format!("Welkom, {}!", "Test123")}
                    </h1>
                </div>
            </MainLayout>
        }
    }
}