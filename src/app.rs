use leptos::*;
use crate::browser;
use crate::components::{Button, Printer, Variant};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bg-pattern min-h-screen w-screen">
            <div class="max-w-screen-lg min-h-screen mx-auto flex items-center justify-center">
                <main class="flex flex-col items-center justify-center gap-10">
                    <section class="flex items-center justify-center gap-10">
                        <div class="relative w-48 h-48 overflow-hidden rounded-full">
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
                        </h1>
                    </section>
                    <Button on:click=move |_| browser::print()>
                        <Printer variant=Variant::Solid/>
                        "Print Profile"
                    </Button>
                </main>
            </div>
        </div>
    }
}
