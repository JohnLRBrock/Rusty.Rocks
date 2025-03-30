use leptos::html::*;
use leptos::IntoView;
use leptos::prelude::*;
use leptos::ev;

pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    let on_click = move |_| {
        set_count.update(|count| *count += 1);
    };

    div()
        .child((
            button()
                .on(ev::click, on_click)
                .child("Collect Rock"),
        )).child(
            div()
                .child(("Clicks: ", move || count.get())),
        )
}