use babs::models::Game;
use yew::{Component, html, classes, Html, Context};

use crate::components::{
    drawer::Drawer,
    icon::Icon,
    list::List, 
    list_item::ListItem, 
    list_item_button::ListItemButton, 
    list_item_icon::ListItemIcon,
    app_bar::AppBar
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
                    <p>{"TesT"}</p>
                </AppBar>
                <Drawer>
                    <nav>
                        <List>
                            <ListItem>
                                <ListItemButton>
                                    <ListItemIcon>
                                        <Icon name="sports_esports" />
                                    </ListItemIcon>
                                    <p>{"Test game"}</p>
                                </ListItemButton>
                            </ListItem>
                        </List>
                    </nav>
                </Drawer>
            </div>
        }
    }
}