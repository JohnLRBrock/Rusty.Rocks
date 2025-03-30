use leptos::prelude::*;

#[component]
pub fn Rock() -> impl IntoView {
    let (click_count, set_click_count) = signal(0);
    
    let on_click = move |_| {
        set_click_count.update(|count| *count += 1);
    };

    view! {
        <div>
            <button
                class="rock"
                on:click=on_click
            >
                Collect Rock
            </button>
            <div class="click-count">
                "Clicks: " {click_count}
            </div>
        </div>
    }
} 