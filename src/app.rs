use leptos::*;
use crate::browser;
use crate::components::{Button, Printer, Variant};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
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
                <h1 class="text-stone-100 text-4xl">
                    "Hi, I'm "
                    <a
                        class="underline hover:text-stone-300 active:text-stone-400"
                        href="https://translate.google.com/?sl=en&tl=ru&text=Yegor%20Zhidal&op=translate">
                        "Yegor Zhidal"
                    </a>
                    " 👋"
                </h1>
            </section>
            <Button on:click=move |_| browser::print()>
                <Printer variant=Variant::Solid/>
                "Print CV"
            </Button>
        </main>
    }
}
