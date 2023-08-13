use crate::browser;
use crate::components::{Avatar, Button, IcoPrinter, IconVariant};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bg-pattern-topography bg-neutral-900 min-h-screen w-screen">
            <main class="max-w-screen-lg w-full p-10 mx-auto flex flex-col items-center gap-10">
                <IntroSection />
                <PersonalitySection />
                <JourneySection />
                <DownloadPortfolioSection />
            </main>
            <Footer />
        </div>
    }
}

#[component]
fn IntroSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex items-center justify-center gap-10 py-10">
            <div class="w-48 h-48">
                <Avatar
                    src="assets/profile-photo.webp"
                    alt="profile photo"
                />
            </div>
            <h1>
                "Hi, I'm "
                <a
                    class="underline"
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
            <h2>"Who am I?"</h2>
            <article class="flex flex-col gap-3">
                <p>
                    r#"
                        I'm a web-engineer who is deeply fascinated by the boundless possibilities
                        of the web-platform. My journey reflects adaptability, embracing diverse challenges
                        and creating seamless web-experiences.
                    "#
                </p>
                <p>
                    r#"
                        As a passionate learner, I continuously broaden my expertise and explore new domains
                        e.g. this very web-page you are currently reading is built with Rust.
                        A few years ago, I couldn't even have imagined that it would be possible!
                    "#
                </p>
                <p>
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
fn JourneySection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="w-full self-start flex flex-col gap-5">
            <h2>"My Career"</h2>
            <article class="flex gap-5">
                <img
                    class="w-36 h-36"
                    src="assets/vention.webp"
                    alt="IT company logotype, abstract lines forming V-shape"
                />
                <div class="flex flex-col gap-3">
                    <h3>"Vention"</h3>
                    <div class="flex items-center gap-2">
                        <p class="italic text-lg text-teal-500">"November 2019 — NOW"</p>
                        <span class="relative flex h-3 w-3">
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-teal-400 opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-3 w-3 bg-teal-500"></span>
                        </span>
                    </div>
                    <p>
                        r#"
                            Vention is an outsourcing company which played a pivotal role in shaping my career trajectory.
                            Thanks to it, I had the opportunity to mature as a web-engineer through work in multiple projects,
                            each contributing significantly to my professional growth and enriching my skill set.
                        "#
                    </p>
                </div>
            </article>
        </section>
    }
}

#[component]
fn DownloadPortfolioSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="py-10 no-print">
            <Button on:click=move |_| browser::window::print()>
                <IcoPrinter variant=IconVariant::Solid/>
                "Print Portfolio"
            </Button>
        </section>
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="w-full mx-auto bg-neutral-900 border-t-2 border-teal-700">
            <div class="max-w-screen-lg mx-auto px-10 py-8 flex items-center">
                <ul class="flex gap-10">
                    <li>
                        <a href="https://github.com/ladihzey">GitHub</a>
                    </li>
                    <li>
                        <a href="https://www.linkedin.com/in/yegor-zhidal-10530a1a7">LinkedIn</a>
                    </li>
                    <li>
                        <a href="mailto: ladihzey@proton.me">Email</a>
                    </li>
                </ul>
            </div>
        </footer>
    }
}
