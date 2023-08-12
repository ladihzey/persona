use crate::browser;
use crate::components::{Avatar, Button, IcoPrinter, IconVariant};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bg-pattern min-h-screen w-screen flex flex-col justify-between">
            <main class="max-w-screen-lg w-full py-20 mx-auto flex flex-col items-center gap-10">
                <IntroSection />
                <PersonalitySection />
                <CareerSection />
                <EducationSection />
                <Button on:click=move |_| browser::print()>
                    <IcoPrinter variant=IconVariant::Solid/>
                    "Print Profile"
                </Button>
            </main>
            <Footer />
        </div>
    }
}

#[component]
fn IntroSection(cx: Scope) -> impl IntoView {
    view! { cx,
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
    }
}

#[component]
fn PersonalitySection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="w-full self-start flex flex-col gap-5">
            <h2 class="text-stone-100 text-3xl">"Who am I?"</h2>
            <article class="flex flex-col gap-3">
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
                        That is why I highly treasure teamwork and I am committed to being a valuable teammate as well.
                    "#
                </p>
            </article>
        </section>
    }
}

#[component]
fn CareerSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="w-full self-start flex flex-col gap-5">
            <h2 class="text-stone-100 text-3xl">"My Career"</h2>
            <article class="flex flex-col gap-3">
                <h3 class="text-stone-100 text-2xl">"Vention"</h3>
                <p class="text-stone-100">
                    r#"
                        Vention is an outsourcing company which played a pivotal role in shaping my career trajectory.
                        Thanks to it, I had the opportunity to mature as a web-engineer through work in multiple projects,
                        each contributing significantly to my professional growth and enriching my skill set.
                    "#
                </p>
            </article>
        </section>
    }
}

#[component]
fn EducationSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="w-full self-start flex flex-col gap-5">
            <h2 class="text-stone-100 text-3xl">"Education"</h2>
            <article>
                <h3 class="text-stone-100 text-2xl">"Vention Internship"</h3>
            </article>
            <article>
                <h3 class="text-stone-100 text-2xl">"Belarusian State University of Informatics and Radioelectronics"</h3>
            </article>
        </section>
    }
}
#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
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
    }
}
