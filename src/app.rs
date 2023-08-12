use crate::browser;
use crate::components::{Button, IcoPrinter, IcoVariant, Avatar};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bg-pattern min-h-screen w-screen flex flex-col justify-between">
            <div class="max-w-screen-lg pt-20 mx-auto flex items-center justify-center">
                <main class="flex flex-col items-center justify-center gap-10">
                    <section class="flex items-center justify-center gap-10">
                        <div class="w-48 h-48">
                            <Avatar
                                src="assets/profile-photo.webp"
                                alt="Profile photo"
                            />
                        </div>
                        <h1 class="text-stone-100 text-4xl">
                            "Hi, I'm "
                            <a
                                class="underline hover:text-stone-300 active:text-stone-400"
                                href="https://translate.google.com/?sl=en&tl=ru&text=Yegor%20Zhidal&op=translate"
                            >
                                "Yegor Zhidal"
                            </a>
                        </h1>
                    </section>
                    <Button on:click=move |_| browser::print()>
                        <IcoPrinter variant=IcoVariant::Solid/>
                        "Print Profile"
                    </Button>
                </main>
            </div>
            <footer class="w-full mx-auto bg-neutral-900 h-16 border-t-2 border-teal-700">
                <div class="max-w-screen-lg h-full mx-auto flex items-center">
                    <ul class="flex gap-10">
                        <li>
                            <a class="text-stone-100" href="https://github.com/ladihzey">GitHub</a>
                        </li>
                        <li>
                            <a class="text-stone-100" href="https://www.linkedin.com/in/yegor-zhidal-10530a1a7">LinkedIn</a>
                        </li>
                        <li>
                            <a class="text-stone-100" href="mailto: ladihzey@proton.me">Email</a>
                        </li>
                    </ul>
                </div>
            </footer>
        </div>
    }
}
