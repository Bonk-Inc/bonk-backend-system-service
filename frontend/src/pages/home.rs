use babs::models::Game;
use yew::{Component, html, classes, Html, Context};

pub struct Home {
    games: Vec<Game>
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Home { games: Vec::new() }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("min-h-screen", "bg-zinc-800", "text-white")}>

            </div>
        }
    }
}