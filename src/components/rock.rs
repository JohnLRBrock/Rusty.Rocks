use leptos::prelude::*;
use reactive_stores::Store;
use crate::app::GameState;

#[component]
pub fn Rock() -> impl IntoView {
    let store = use_context::<Store<GameState>>()
        .expect("Store should be provided by App component");
    
    let on_click = move |_| {
        store.update(|state| {
            if state.rock_count < state.inventory_size {
                state.rock_count += 1;
            }
        });
    };

    view! {
        <div>
            <button
                class="rock"
                on:click=on_click
            >
                Collect Rock
            </button>
        </div>
    }
} 