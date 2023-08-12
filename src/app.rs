use crate::browser;
use crate::components::{Button, IcoPrinter, IcoVariant, Avatar};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bg-pattern min-h-screen w-screen flex flex-col justify-between">
            <main class="max-w-screen-lg w-full pt-20 mx-auto flex flex-col items-center gap-10">
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
                <section class="w-full self-start flex flex-col gap-5">
                    <h2 class="text-stone-100 text-3xl">"Who am I?"</h2>
                    <p class="text-stone-100 text-lg">
                        r#"
                            I'm a web-engineer who is deeply fascinated by the boundless possibilities
                            of the web-platform. My journey reflects adaptability, embracing diverse challenges
                            and creating seamless web-experiences through innovative solutions.

                        "#
                    </p>
                    <p class="text-stone-100 text-lg">
                        r#"
                            As a passionate learner, I continuously broaden my expertise and explore new domains
                            e.g. this very web page you are currently reading is written in Rust.
                            A few years ago, I couldn't even have imagined that it would be possible!
                        "#
                    </p>
                    <p class="text-stone-100 text-lg">
                        r#"
                            I do believe that the most remarkable things arise from the efforts of great teams.
                            As a result, I highly treasure teamwork, and I am committed to being a valuable teammate as well.
                        "#
                    </p>
                </section>
                <Button on:click=move |_| browser::print()>
                    <IcoPrinter variant=IcoVariant::Solid/>
                    "Print Profile"
                </Button>
            </main>
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
