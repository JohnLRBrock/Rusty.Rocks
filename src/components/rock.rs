use leptos::prelude::*;
use reactive_stores::Store;
use crate::app::GameState;
use crate::models::rock_factory::RockFactory;

#[component]
pub fn Rock() -> impl IntoView {
    let store = use_context::<Store<GameState>>()
        .expect("Store should be provided by App component");
    
    let mut factory = RockFactory::new();
    
    let on_click = move |_| {
        store.update(|state| {
            if state.rocks.len() < state.inventory_size {
                state.rocks.push(factory.generate_rock());
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