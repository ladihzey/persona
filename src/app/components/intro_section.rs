use leptos::*;

#[component]
pub fn IntroSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="min-h-screen w-screen bg-gray-800 flex items-center justify-center">
            <h1 class="text-stone-200 text-5xl">
                "Hi, I'm Yegor Zhidal"
            </h1>
        </main>
    }
}
