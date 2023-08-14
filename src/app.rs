use crate::browser;
use crate::components::{Avatar, BadgeGroup, BadgeVariant, Button, IcoPrinter, IconVariant};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="md:bg-pattern-topography bg-neutral-900 min-h-screen min-w-fit w-screen">
            <main class="max-w-screen-lg w-full p-5 md:p-10 mx-auto flex flex-col items-center gap-5 md:gap-10">
                <IntroSection />
                <PersonalitySection />
                <CareerSection />
                <DownloadProfileSection />
            </main>
            <Footer />
        </div>
    }
}

#[component]
fn IntroSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex flex-col print:flex-row md:flex-row items-center justify-center gap-5 md:gap-10 py-8 md:py-10">
            <div class="w-36 h-36 md:w-48 md:h-48">
                <Avatar
                    src="assets/profile-photo.webp"
                    alt="profile photo"
                />
            </div>
            <h1>
                "Hi, I’m "
                <a class="underline" href="https://translate.google.com/?sl=en&tl=ru&text=Yegor%20Zhidal&op=translate" >
                    "Yegor Zhidal"
                </a>
            </h1>
        </section>
    }
}

#[component]
fn PersonalitySection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="w-full flex flex-col gap-3 md:gap-5">
            <h2>"Who am I?"</h2>
            <article class="flex flex-col gap-2 md:gap-3">
                <p>
                    "I’m a software engineer who is deeply fascinated by the boundless possibilities
                    of the web-platform. My journey reflects adaptability, embracing diverse challenges
                    and creating seamless web-experiences."
                </p>
                <p>
                    "As a passionate learner, I continuously broaden my expertise and explore new domains
                    e.g. this very web-page you are currently reading is built with Rust.
                    A few years ago, I couldn’t even have imagined that it would be possible!"
                </p>
                <p>
                    "I do believe that the most remarkable things arise from the efforts of great teams.
                    That is why I highly treasure teamwork and I am committed to being a valuable teammate as well."
                </p>
            </article>
        </section>
    }
}

#[component]
fn CareerSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="w-full flex flex-col gap-5 md:gap-8">
            <h2>"My Journey"</h2>
            <CompanyVention />
            <UniversityBSUIR />
        </section>
    }
}

#[component]
fn CompanyVention(cx: Scope) -> impl IntoView {
    view! { cx,
        <article class="flex gap-5">
            <div class="w-12 h-12 md:w-32 md:h-32 print:w-12 print:h-12 flex-shrink-0 sticky print:static top-5">
                <img src="assets/vention.webp" alt="IT company logotype, abstract lines forming V-shape" />
            </div>
            <div class="flex flex-col gap-8">
                <VentionDescription />
                <VentionInvestmentAndFinancialLiteracyApp />
                <VentionArtWebscrappingPlatform />
                <VentionBlockChainArtSellingPlatform />
                <VentionFoodDeliveryPlatform />
                <VentionITEducationPlatform />
            </div>
        </article>
    }
}

#[component]
fn VentionDescription(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex flex-col gap-3">
            <div>
                <h3>"Vention"</h3>
                <div class="flex items-center gap-2">
                    <p class="italic text-teal-500">"August 2019 — NOW"</p>
                    <span class="relative flex h-3 w-3">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-teal-400 opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-3 w-3 bg-teal-500"></span>
                    </span>
                </div>
            </div>
            <p>
                "Vention is an outsourcing company which played a pivotal role in shaping my career trajectory.
                Thanks to it, I had the opportunity to mature as a software engineer through work
                in multiple projects, each contributing significantly to my professional growth and
                enriching my skill set."
            </p>
        </section>
    }
}

#[component]
fn VentionInvestmentAndFinancialLiteracyApp(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex flex-col gap-2">
            <div>
                <h4>"Investment and Financial Literacy Education App"</h4>
                <p class="italic text-teal-500">"6 months"</p>
            </div>
            <BadgeGroup badges=&[
                (BadgeVariant::Green, "Node"),
                (BadgeVariant::Purple, "TypeScript"),
                (BadgeVariant::Pink, "Express"),
                (BadgeVariant::Pink, "Sequelize"),
                (BadgeVariant::Yellow, "PostgreSQL"),
                (BadgeVariant::Yellow, "AWS"),
                (BadgeVariant::Red, "Redis"),
            ] />
            <p class="pt-3">
                "The app goal was to educate people about investing by offering informative materials
                and a microinvestment platform. Through this initiative, users can not only gain
                insights into the workings of money and the economy, but also take important initial
                steps in building trust within the investment system."
            </p>
            <p>
                "Initially, I was introduced to this project to"
                <em class="font-bold">" conduct an assessment"</em>
                ". While project assessments were new to me, identifying the issues wasn’t hard.
                What became really interesting experience to me is my further work there as a"
                <em class="font-bold">" back-end engineer "</em>
                "aiming to mitigate the identified issues and establish
                development processes that would prevent such issues in the future"
            </p>
            <p>
                "I’ve learnt that it’s a complex task to change the team and the project quickly,
                even when understanding how the things should be done and what went wrong.
                Developing migration and refactoring strategies, while simultaneously keeping
                the development ongoing, ain’t easy. It requires time and patience,
                moving further step-by-step."
            </p>
        </section>
    }
}

#[component]
fn VentionArtWebscrappingPlatform(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex flex-col gap-2">
            <div>
                <h4>"Art Auctions Web-Scrapping Platform"</h4>
                <p class="italic text-teal-500">"1 year 11 months"</p>
            </div>
            <BadgeGroup badges=&[
                (BadgeVariant::Green, "Node"),
                (BadgeVariant::Purple, "TypeScript"),
                (BadgeVariant::Blue, "Puppeteer"),
                (BadgeVariant::Pink, "TypeORM"),
                (BadgeVariant::Yellow, "PostgreSQL"),
                (BadgeVariant::Yellow, "AWS"),
                (BadgeVariant::Red, "Ruby"),
                (BadgeVariant::Blue, "Python"),
            ] />
            <p class="pt-3">
                "A platform aimed to provide users with fine-grained analytics on the art world,
                so that they could decide which artworks or artists could not only become valuable assets
                to their collection, but also serve as a wise investment. Apparently, artworks prove to be
                an effective investment tool."
            </p>
            <p>
                "That projects stands out as one of the important moments in my career. I was working there as a"
                <em class="font-bold">" software engineer"</em>
                ", and had possibility to step into fascinating realm of"
                <em class="font-bold">" distributed systems "</em>
                "for the first time. It marked the beginning of my journey in grasping the concepts of
                microservices and event-driven design."
            </p>
            <p>
                "Over the span of almost two years, I had the opportunity of not only leading a small team
                , but also designing and establishing the architecture of
                the web-scrapping pipelines."
            </p>
        </section>
    }
}

#[component]
fn VentionBlockChainArtSellingPlatform(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex flex-col gap-2">
            <div>
                <h4>"Blockchain Based Art Selling Platform"</h4>
                <p class="italic text-teal-500">"2 months"</p>
            </div>
            <BadgeGroup badges=&[
                (BadgeVariant::Green, "Node"),
                (BadgeVariant::Purple, "TypeScript"),
                (BadgeVariant::Pink, "Express"),
                (BadgeVariant::Pink, "Sequelize"),
                (BadgeVariant::Yellow, "MySQL"),
                (BadgeVariant::Yellow, "AWS"),
                (BadgeVariant::Blue, "React"),
                (BadgeVariant::Blue, "Redux"),
            ] />
            <div class="flex flex-wrap gap-2">
            </div>
            <p class="pt-3">
                "This project was dedicated to the creation of an art platform that seamlessly
                merged the worlds of digital NFC and physical art. It included the trading and purchase
                of artworks through the use of cryptocurrencies, enhancing accessibility and offering
                a modern way to engage with the art market."
            </p>
            <p>
                "I joined this project as a"
                <em class="font-bold">" full-stack engineer "</em>
                "for a short term to support the team
                with an upcoming release. I had to adapt quickly in order to assist the team in
                completing their work. I can’t say that I’ve learned blockchain and cryptocurrencies,
                but it was a great lesson for working under pressure and tight deadlines."
            </p>
        </section>
    }
}

#[component]
fn VentionFoodDeliveryPlatform(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex flex-col gap-2">
            <div>
                <h4>"Fresh Food and Food-Kit Delivery"</h4>
                <p class="italic text-teal-500">"5 months"</p>
            </div>
            <BadgeGroup badges=&[
                (BadgeVariant::Purple, "TypeScript"),
                (BadgeVariant::Blue, "React"),
                (BadgeVariant::Blue, "Redux"),
            ] />
            <p class="pt-3">
                "This project revolutionized the culinary experience, or at least it was one of the
                first of its kind. By providing fresh ingredients instead of pre-prepared meals,
                it introduced a fine-grained diet planner that emphasized health and nutrition.
                It not only simplified cooking, but also empowered individuals to make healthier
                dietary choices."
            </p>
            <p>
                "I worked on this project as a"
                <em class="font-bold">" frontend-end engineer"</em>
                ". I was responsible for the development of a control panel for managing orders,
                logistics, users, and stored ingredients."
            </p>
            <p>
                "While the team was large and I didn’t feel like I was making a big impact,
                I had a good opportunity to work with"
                <em class="font-bold">" microfrontends architecture"</em>
                "."
            </p>
        </section>
    }
}

#[component]
fn VentionITEducationPlatform(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="flex flex-col gap-2">
            <div>
                <h4>"IT Educational Platform"</h4>
                <p class="italic text-teal-500">"8 months"</p>
            </div>
            <BadgeGroup badges=&[
                (BadgeVariant::Green, "Node"),
                (BadgeVariant::Purple, "TypeScript"),
                (BadgeVariant::Pink, "NestJS"),
                (BadgeVariant::Pink, "TypeORM"),
                (BadgeVariant::Yellow, "PostgreSQL"),
                (BadgeVariant::Blue, "React"),
                (BadgeVariant::Blue, "Redux"),
            ] />
            <p class="pt-3">
                "An internal company’s platform designed to organize education of IT specialists.
                Mentors were able to utilize it in order to create customized educational plans tailored
                to the abilities of their mentees. The platform provided tools for creation of plans,
                schedules based on students’ availability, and deadline tracking."
            </p>
            <p>
                "It was a great initial project, where I gradually gained invaluable experience.
                Starting from the small tasks, I eventually ended up responsible for the end-to-end
                development of entire features."
            </p>
        </section>
    }
}

#[component]
fn UniversityBSUIR(cx: Scope) -> impl IntoView {
    view! { cx,
        <article class="flex gap-5">
            <div class="w-12 h-12 md:w-32 md:h-32 print:w-12 print:h-12 flex-shrink-0 sticky print:static top-5">
                <img src="assets/bsuir.webp" alt="university logotype, radio-waves" />
            </div>
            <div class="flex flex-col">
                <h3>"BSUIR"</h3>
                <p class="italic text-teal-500">"2017 — 2022"</p>
                <p class="pt-3">
                    "Belarusian State University of Informatics and Radioelectronics"
                    <br />
                    "Faculty of Computer Systems and Networks"
                </p>
                <p class="pt-3">
                    "While I didn’t complete my diploma work and attain a bachelor’s degree,
                    I did complete all the required courses. This educational journey
                    has provided me with valuable foundational knowledge that I’ve applied
                    and built upon throughout my experiences."
                </p>
            </div>
        </article>
    }
}

#[component]
fn DownloadProfileSection(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="py-8 md:py-10 print:hidden">
            <Button on:click=move |_| browser::window::print()>
                <IcoPrinter variant=IconVariant::Solid/>
                "Print Profile"
            </Button>
        </section>
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="w-full mx-auto bg-neutral-900 border-t-2 border-teal-700">
            <div class="max-w-screen-lg mx-auto p-5 py-8 md:px-10 md:py-8 flex items-center">
                <ul class="flex flex-row print:flex-col gap-5 md:gap-10 print:gap-3">
                    <li>
                        <a class="flex items-center gap-2 print:hidden" href="https://github.com/ladihzey">
                            <div class="w-5 h-5">
                                <img src="assets/github.png" alt="github space-cat logo" />
                            </div>
                            "GitHub"
                        </a>
                        <p class="hidden print:inline-block">"GitHub: https://github.com/ladihzey"</p>
                    </li>
                    <li>
                        <a class="flex items-center gap-2 print:hidden" href="https://www.linkedin.com/in/yegor-zhidal-10530a1a7">
                            <div class="w-5 h-5">
                                <img src="assets/linkedin.png" alt="blue linkedin logo" />
                            </div>
                            "LinkedIn"
                        </a>
                        <p class="hidden print:inline-block">"LinkedIn: https://www.linkedin.com/in/yegor-zhidal-10530a1a7"</p>
                    </li>
                    <li>
                        <a class="flex items-center gap-2 print:hidden" href="mailto: ladihzey@proton.me">
                            <div class="w-5 h-5">
                                <img src="assets/protonmail.png" alt="protonmail logo" />
                            </div>
                            "Email"
                        </a>
                        <p class="hidden print:inline-block">"Email: ladihzey@proton.me"</p>
                    </li>
                </ul>
            </div>
        </footer>
    }
}
