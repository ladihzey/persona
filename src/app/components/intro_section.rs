use leptos::*;
use stylers::{style};

#[component]
pub fn IntroSection(cx: Scope) -> impl IntoView {
    let styles = style! { "IntroSection",
        .intro-section {
            display: block flex;
            align-items: center;
            justify-content: center;
            width: 100lvw;
            height: 100lvh;
            background-color: dimgray;
        }

        h1 {
            text-transform: uppercase;
            font-family: mono;
            color: ivory;
        }
    };

    view! { cx, class = styles,
        <main class="intro-section">
            <h1>"Hi, I'm Yegor Zhidal"</h1>
        </main>
    }
}
