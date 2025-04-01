use leptos::prelude::*;
use reactive_stores::Store;
use crate::components::inventory::Inventory;
use crate::components::clout::Clout;

#[derive(Clone)]
pub struct GameState {
    pub clout: i32,
    pub inventory_size: i32,
    pub rock_count: i32,
}

fn data() -> GameState {
    GameState {
        clout: 0,
        inventory_size: 10,
        rock_count: 0,
    }
}

#[component]
pub fn App() -> impl IntoView {
    let store = Store::new(data());
    provide_context(store);

    view! {
        <Clout/>
        <Inventory/>
    }
}