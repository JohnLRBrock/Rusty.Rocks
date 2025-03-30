use leptos::html::*;
use leptos::IntoView;
use leptos::prelude::*;
use leptos::ev;

/// A simple counter view.
pub fn Counter(
) -> impl IntoView {
    let (count, set_count) = signal(0);

    div()
        .child((
            button()
                .on(ev::click, move |_| set_count.update(|count| *count += 1))
                .child("Collect Rock"),
        )).child(
            div()
                .child(("Clicks: ", move || count.get())),
        )
}