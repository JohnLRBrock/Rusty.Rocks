use leptos::prelude::*;
use reactive_stores::Store;
use crate::components::inventory::Inventory;
use crate::components::clout::Clout;
use crate::models::rock_factory::Rock;
use crate::components::rock::Rock;
use crate::components::rock_eater::RockEater;

#[derive(Clone)]
pub struct GameState {
    pub clout: i32,
    pub inventory_size: usize,
    pub rocks: Vec<Rock>,
}

fn data() -> GameState {
    GameState {
        clout: 0,
        inventory_size: 10,
        rocks: Vec::new(),
    }
}

#[component]
pub fn App() -> impl IntoView {
    let store = Store::new(data());
    provide_context(store);

    view! {
        <Clout/>
        <Rock/>
        <RockEater/>
        <Inventory/>
    }
}