use babs::models::Game;
use yew::{Component, html, classes, Html, Context};

use crate::components::{
    drawer::Drawer, 
    list::List, 
    list_item::ListItem, 
    list_item_button::ListItemButton, 
    list_item_icon::ListItemIcon, icon::Icon
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
                <Drawer>
                    <List>
                        <ListItem>
                            <ListItemButton>
                                <ListItemIcon>
                                    <Icon name="folder" />
                                </ListItemIcon>
                                <p>{"Test game"}</p>
                            </ListItemButton>
                        </ListItem>
                    </List>
                </Drawer>
            </div>
        }
    }
}