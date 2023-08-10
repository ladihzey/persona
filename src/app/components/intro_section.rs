use wasm_bindgen::prelude::*;
use leptos::*;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn print();
}

#[component]
pub fn IntroSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="min-h-screen w-screen bg-gray-800 flex flex-col items-center justify-center gap-10">
            <section class="flex items-center justify-center gap-10">
                <div class="relative w-48 h-48 overflow-hidden rounded-full border-8 border-gray-700">
                    <img
                        class="absolute inset-0 w-full h-full object-cover"
                        src="assets/profile-photo.webp"
                        alt="Profile photo"
                    />
                </div>
                <h1 class="text-stone-300 text-5xl">
                    "Hi, I'm "
                    <a
                        class="underline hover:text-stone-200 active:text-stone-50"
                        href="https://translate.google.com/?sl=en&tl=ru&text=Yegor%20Zhidal&op=translate">
                        "Yegor Zhidal"
                    </a>
                    " 👋"
                </h1>
            </section>
            <button
                on:click=move |_| print()
                class="text-stone-300 uppercase bg-gray-700 p-4 rounded-md"
            >"Print CV"</button>
        </main>
    }
}
