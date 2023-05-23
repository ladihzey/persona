use leptos::*;
use stylers::{style};

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let increment = move |_| set_count.update(|n| *n += 1);

    let styler_class = style! {"App",
        .one {
            color: red;
            padding: 20px;
        }
    };

    view! { cx, class = styler_class,
        <button
            class="one"
            on:click=increment
        >
            "Click me: "
            {count}
        </button>
    }
}

