use leptos::*;

mod components;
use components::IntroSection;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <IntroSection />
    }
}
