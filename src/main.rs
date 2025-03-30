use leptos::prelude::*;
use crate::components::rock::Rock;
use crate::components::inventory::Inventory;
use crate::components::map::Map;
use crate::app::App;
pub mod app;
pub mod components;


fn main() {
    mount_to_body(|| view! {
        <App>
            <Inventory>
                <Rock/>
            </Inventory>
            <Map/>
        </App>
    })
}
