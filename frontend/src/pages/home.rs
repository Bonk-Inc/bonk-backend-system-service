use babs::models::Game;
use yew::{Component, html, classes, Html, Context};

use crate::components::{
    drawer::Drawer,
    icon::Icon,
    list::List, 
    list_item::ListItem, 
    list_item_button::ListItemButton, 
    list_item_icon::ListItemIcon,
    app_bar::AppBar, toolbar::Toolbar
};

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
            <div class={classes!("min-h-screen", "bg-zinc-800", "text-white", "flex")}>
                <AppBar>
                    <Toolbar class="border-b border-zinc-500 border-solid">
                        <p class={classes!("font-medium", "text-xl", "flex", "items-center")}>
                            <Icon name="database" class="mr-4"/>
                            {"Bonk Inc Backend System"}
                        </p>
                    </Toolbar>
                </AppBar>
                <nav class={classes!("h-full")}>
                    <Drawer>
                        <Toolbar class="mt-14 [min-h-48px]">
                            <p class={classes!("font-medium", "flex")}>
                                <Icon name="list" class="mr-2"/>
                                {"Games"}
                            </p>
                        </Toolbar>
                        <hr class={classes!("w-11/12", "border-zinc-500", "mx-auto")}/>
                        <List>
                            <ListItem>
                                <ListItemButton>
                                    <ListItemIcon>
                                        <Icon name="joystick" />
                                    </ListItemIcon>
                                    <p>{"Test game"}</p>
                                </ListItemButton>
                            </ListItem>
                        </List>
                    </Drawer>
                </nav>
                <main style="width: calc(100% - 240px);" class={classes!("grow", "mt-14")}>

                </main>
            </div>
        }
    }
}