use leptos::prelude::*;
use reactive_stores::Store;
use crate::app::GameState;  

#[component]
pub fn RockEater() -> impl IntoView {
    let store = use_context::<Store<GameState>>()
        .expect("Store should be provided by App component");

    let on_click = move |_| {   
        store.update(|state| {
            if !state.rocks.is_empty() {
                if let Some(rock) = state.rocks.pop() {
                    state.clout += rock.value as i32;
                }
            }
        });
    };

    view! {
        <button on:click=on_click>Eat Rock</button>
    }   
}   