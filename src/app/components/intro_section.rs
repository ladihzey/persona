use leptos::*;

#[component]
pub fn IntroSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="min-h-screen w-screen bg-gray-800 flex items-center justify-center gap-8">
            <div class="relative w-48 h-48 overflow-hidden rounded-full border-8 border-gray-700">
                <img src="assets/profile-photo.webp" alt="Profile photo" class="absolute inset-0 w-full h-full object-cover" />
            </div>
            <h1 class="text-stone-200 text-5xl">
                "Hi, I'm Yegor Zhidal"
            </h1>
        </main>
    }
}
