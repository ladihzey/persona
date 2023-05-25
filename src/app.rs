use leptos::*;
use stylers::{style_sheet};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let increment = move |_| set_count.update(|n| *n += 1);

    let styles = style_sheet!("./src/app.css");

    view! { cx, class = styles,
        <button
            class="one"
            on:click=increment
        >
            "Click me: "
            {count}
        </button>
    }
}
