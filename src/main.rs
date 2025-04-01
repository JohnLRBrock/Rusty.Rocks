use leptos::prelude::*;
use crate::app::App;
pub mod app;
pub mod components;


fn main() {
    mount_to_body(|| view! {
        <App>
        </App>
    })
}
