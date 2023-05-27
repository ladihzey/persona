use leptos::*;
use stylers::{style_sheet};

#[component]
pub fn IntroSection(cx: Scope) -> impl IntoView {
    let styles = style_sheet!("./src/app/components/intro_section.css");

    view! { cx, class = styles,
        <main class="intro-section">
            "Hi, I'm Yegor Zhidal"
        </main>
    }
}
