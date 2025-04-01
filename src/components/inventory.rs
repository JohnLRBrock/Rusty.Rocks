use leptos::prelude::*;
use reactive_stores::Store;
use crate::app::GameState;
use crate::components::rock::Rock;
use crate::components::rock_eater::RockEater;

#[component]
pub fn Inventory() -> impl IntoView {
    let store = use_context::<Store<GameState>>()
        .expect("Store should be provided by App component");

    view! {
        <div class="inventory">
            <div class="inventory-count">
                {move || {
                    let state = store.get();
                    format!("Rocks: {} / {}", state.rock_count, state.inventory_size)
                }}
            </div>
            <Rock/>
            <RockEater/>
        </div>
    }
}