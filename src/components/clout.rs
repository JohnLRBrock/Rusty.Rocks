use leptos::prelude::*;
use reactive_stores::Store;
use crate::app::GameState;

#[component]
pub fn Clout() -> impl IntoView {
    let store = use_context::<Store<GameState>>()
        .expect("Store should be provided by App component");

    view! {
        <div>
            {move || {
                let state = store.get();
                format!("Clout: {}", state.clout)
            }}
        </div>
    }
} 