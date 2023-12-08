use yew::{classes, html, Children, Component, Context, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

use crate::{components::{toolbar::Toolbar, tabs::Tabs, tab::Tab}, app::AppRoute};

pub struct GameLayout;

#[derive(Clone, PartialEq, Properties)]
pub struct GameLayoutProps {
    pub children: Children,
    pub id: String,
}

pub enum Msg {
    Test,
    NavigateToGame,
    NavigateToScores,
}

impl Component for GameLayout {
    type Message = Msg;
    type Properties = GameLayoutProps;

    fn create(_ctx: &Context<Self>) -> Self {
        GameLayout {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let navigator = ctx.link().navigator().unwrap();
        let id = ctx.props().id.clone();

        match msg {
            Msg::Test => {todo!()},
            Msg::NavigateToGame => {
                navigator.push(&AppRoute::Game { id });
            },
            Msg::NavigateToScores => {
                navigator.push(&AppRoute::Scores { game_id: id });
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone();
        let location = ctx.link().route::<AppRoute>().unwrap_or(AppRoute::Game { id });

        html! {
            <div class={classes!("h-full")}>
                <Toolbar class="!p-0 border-b border-zinc-500 border-solid !min-h-48px">
                    <Tabs>
                        <Tab 
                            icon="home" 
                            label="Home" 
                            onclick={ctx.link().callback(|_| Msg::NavigateToGame)} 
                            selected={matches!(location, AppRoute::Game { .. })} 
                        />
                        <Tab 
                            icon="scoreboard" 
                            label="Scores" 
                            onclick={ctx.link().callback(|_| Msg::NavigateToScores)}
                            selected={matches!(location, AppRoute::Scores { .. })} 
                        />
                        <Tab icon="map" label="Levels" onclick={ctx.link().callback(|_| Msg::Test)} />
                    </Tabs>
                </Toolbar>
                <div class={classes!("p-4")}>
                    {ctx.props().children.clone()}
                </div>
            </div>
        }
    }
}
