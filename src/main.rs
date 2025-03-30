use leptos::prelude::*;
mod components;
use components::Rock;
use components::Counter;

fn main() {
    mount_to_body(|| view! {
        <div class="app-container">
            <Rock/>
            {Counter()}
        </div>
    })
}
