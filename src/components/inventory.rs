use leptos::prelude::*;
use reactive_stores::Store;
use crate::app::GameState;

#[component]
pub fn Inventory(children: Children) -> impl IntoView {
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
            <div class="inventory-status">
                {move || {
                    let state = store.get();
                    let remaining = state.inventory_size - state.rock_count;
                    if remaining <= 0 {
                        "Inventory Full!".to_string()
                    } else {
                        format!("Space for {} more rocks", remaining)
                    }
                }}
            </div>
            {children()}
        </div>
    }
}