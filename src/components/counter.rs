use leptos::html::*;
use leptos::IntoView;
use leptos::prelude::*;
use leptos::ev;

/// A simple counter view.
pub fn Counter(
    initial_value: i32,
    step: i32,
) -> impl IntoView {
    let (count, set_count) = signal(initial_value);

    div()
        .child((
            button()
                .on(ev::click, move |_| set_count.set(0))
                .child("Clear"),
            button()
                .on(ev::click, move |_| set_count.update(|count| *count -= step))
                .child("-1"),
            span()
                .child(("Value: ", move || count.get(), "!")),
            button()
                .on(ev::click, move |_| set_count.update(|count| *count += step))
                .child("+1"),
        ))
}