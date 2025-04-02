use leptos::prelude::*;
use crate::app::App;
pub mod app;
pub mod components;
pub mod models;
pub mod consts;


fn main() {
    mount_to_body(|| view! {
        <App>
        </App>
    })
}
