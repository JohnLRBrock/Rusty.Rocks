use leptos::prelude::*;
use crate::components::rock::Rock;
use crate::components::inventory::Inventory;
use crate::components::map::Map;
use crate::components::rock_eater::RockEater;
use crate::components::clout::Clout;
use crate::app::App;
pub mod app;
pub mod components;


fn main() {
    mount_to_body(|| view! {
        <App>
        </App>
    })
}
