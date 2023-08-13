use crate::browser;
use crate::components::{Avatar, Button, IcoPrinter, IconVariant};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="bg-pattern min-h-screen w-screen">
            <main class="max-w-screen-lg w-full p-10 mx-auto flex flex-col items-center gap-10">
                <IntroSection />
                <PersonalitySection />
                <CareerSection />
                <EducationSection />
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
                    alt="Profile photo"
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
                        e.g. this very web page you are currently reading is written in Rust.
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
fn CareerSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="w-full self-start flex flex-col gap-5">
            <h2>"My Career"</h2>
            <article class="flex flex-col gap-3">
                <h3>"Vention"</h3>
                <p>
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
            <h2>"Education"</h2>
            <article>
                <h3>"Vention Internship"</h3>
            </article>
            <article>
                <h3>"BSUIR"</h3>
            </article>
        </section>
    }
}

#[component]
fn DownloadPortfolioSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="py-10">
            <Button on:click=move |_| browser::print()>
                <IcoPrinter variant=IconVariant::Solid/>
                "Print Portfolio"
            </Button>
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
