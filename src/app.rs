use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Clone)]
pub struct GameState {
    pub rock_count: i32,
    pub inventory_size: i32,
}

fn data() -> GameState {
    GameState {
        rock_count: 0,
        inventory_size: 10,
    }
}

#[component]
pub fn App(children: Children) -> impl IntoView {
    let store = Store::new(data());
    provide_context(store);

    view! {
        <div class="app">
            {children()}
        </div>
    }
}