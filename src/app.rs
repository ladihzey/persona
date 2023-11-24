use crate::components::{Avatar, BadgeGroup, BadgeVariant, IconPrinter, IconVariant};
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="md:bg-pattern-topography bg-neutral-900 min-h-screen min-w-fit w-screen">
            <main class="max-w-screen-lg w-full p-5 md:p-10 print:p-0 mx-auto flex flex-col items-center gap-8 md:gap-14 print:gap-5">
                <IntroSection />
                <PersonalitySection />
                <SkillsSection />
                <CareerSection />
                <EducationSection />
                <DownloadProfileSection />
            </main>
            <Footer />
        </div>
    }
}

#[component]
fn IntroSection() -> impl IntoView {
    view! {
        <section class="flex flex-col print:flex-row md:flex-row items-center justify-center gap-5 md:gap-14 py-8 md:py-14 print:p-0">
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
fn PersonalitySection() -> impl IntoView {
    view! {
        <section class="w-full">
            <h2 class="pb-3 md:pb-5">"Who am I?"</h2>
            <article class="flex flex-col gap-2">
                <p>
                    "I’m a software engineer who is deeply fascinated by the boundless possibilities
                    of the web-platform. My journey reflects adaptability, embracing diverse challenges
                    and creating seamless web-experiences."
                </p>
                <p>
                    "As a passionate learner, I continuously broaden my expertise and explore new domains
                    e.g. this very web-page you are currently reading is built with Rust."
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
fn SkillsSection() -> impl IntoView {
    view! {
       <section class="w-full">
            <h2 class="pb-3 md:pb-5">"Tech Stack"</h2>
            <BadgeGroup badges=&[
                (BadgeVariant::Green, "Node"),
                (BadgeVariant::Purple, "TypeScript"),
                (BadgeVariant::Yellow, "AWS"),
                (BadgeVariant::Yellow, "PostgreSQL"),
                (BadgeVariant::Yellow, "MySQL"),
                (BadgeVariant::Red, "Redis"),
                (BadgeVariant::Blue, "Docker"),
                (BadgeVariant::Pink, "NestJS"),
                (BadgeVariant::Pink, "Express"),
                (BadgeVariant::Pink, "TypeORM"),
                (BadgeVariant::Pink, "Sequelize"),
                (BadgeVariant::Blue, "React"),
                (BadgeVariant::Blue, "Redux"),
            ] />
       </section>
    }
}

#[component]
fn CareerSection() -> impl IntoView {
    view! {
        <section class="w-full">
            <h2 class="pb-3 md:pb-5">"My Journey"</h2>
            <div class="flex flex-col gap-8 md:gap-12">
                <CompanyMonday />
                <CompanyVention />
            </div>
        </section>
    }
}

#[component]
fn CompanyMonday() -> impl IntoView {
    view! {
        <article class="flex gap-5">
            <a class="w-12 h-12 md:w-32 md:h-32 print:w-12 print:h-12 flex-shrink-0 sticky print:static top-5 filter hover:brightness-90"
                href="https://monday.com/"
            >
                <img src="assets/monday.webp" alt="IT company logotype, three round shapes forming M-letter" />
            </a>
            <div class="flex flex-col gap-8 md:gap-10">
                <MondayDescription />
            </div>
        </article>
    }
}

#[component]
fn MondayDescription() -> impl IntoView {
    view! {
        <section class="flex flex-col gap-2">
            <div>
                <h3>
                    <a href="https://monday.com/">"Monday.com"</a>
                </h3>
                <div class="flex items-center gap-2">
                    <p class="italic text-teal-500">"December 2023 — Present"</p>
                    <span class="relative flex h-3 w-3">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-teal-400 opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-3 w-3 bg-teal-500"></span>
                    </span>
                </div>
                <p class="py-3">
                    "Monday.com is a product company that provides customers with no-code platform (Work OS),
                    which they could use to shape all the necessary workflows that their business requires."
                </p>
            </div>
        </section>
    }
}

#[component]
fn CompanyVention() -> impl IntoView {
    view! {
        <article class="flex gap-5">
            <a class="w-12 h-12 md:w-32 md:h-32 print:w-12 print:h-12 flex-shrink-0 sticky print:static top-5 filter hover:brightness-90"
                href="https://ventionteams.com/"
            >
                <img src="assets/vention.webp" alt="IT company logotype, abstract lines forming V-shape" />
            </a>
            <div class="flex flex-col gap-8 md:gap-10">
                <VentionDescription />
                <VentionArtWebScrappingPlatform />
                <VentionInvestmentAndFinancialLiteracyApp />
                <VentionBlockChainArtSellingPlatform />
                <VentionFoodDeliveryPlatform />
                <VentionEDUPlatform />
            </div>
        </article>
    }
}

#[component]
fn VentionDescription() -> impl IntoView {
    view! {
        <section class="flex flex-col gap-2">
            <div>
                <h3>
                    <a href="https://ventionteams.com/">"Vention"</a>
                </h3>
                <p class="italic text-teal-500">"August 2019 — November 2023"</p>
            </div>
            <p class="py-3">
                "Vention is an outsourcing company which played a pivotal role in shaping my career trajectory.
                Thanks to it, I had the opportunity to mature as a software engineer through work
                in multiple projects, each contributing significantly to my professional growth and
                enriching my skill set."
            </p>
            <p>
                "Within the company, there is an internal organization called EDU — a community
                of people, who willingly share their knowledge
                and experiences with each other"
            </p>
            <p>
                "I had the responsibility of"
                <em class="font-bold">" organizing the frontend community"</em>
                ", membership of which reached"
                <em class="font-bold">" 108 individuals "</em>
                "at my time. This role included coordinating various events such as meetups and news digests
                within the frontend community. These experiences granted me invaluable insights into
                human motivation, leadership, and the management of large groups.
                "
            </p>
            <p>
                "Additionally, I was an active member of the EDU community myself, which opened
                doors to mentorship, participation in interviews, and contributions to the shared knowledge base."
            </p>
        </section>
    }
}

#[component]
fn VentionInvestmentAndFinancialLiteracyApp() -> impl IntoView {
    view! {
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
            <p class="py-3">
                "The app goal was to educate people about investing by offering informative materials
                and a microinvestment platform. Through this initiative, users can not only gain
                insights into the budget management, but also build trust with the investment system."
            </p>
            <p>
                "Initially, I was introduced to this project to"
                <em class="font-semibold">" conduct an assessment"</em>
                ". While project assessments were new to me, identifying the issues wasn’t hard.
                What became really interesting experience is my further work there as a"
                <em class="font-semibold">" software engineer "</em>
                "aiming to mitigate the identified issues and establish
                development processes that would stabilize the project."
            </p>
            <p>
                "I’ve learnt that it’s a complex task to do a processes migration and perform refactoring
                strategies, while simultaneously keeping the development process ongoing. It ain’t easy to change
                the team and the project quickly, even when understanding how the things should be done
                and what exactly went wrong. It requires time and patience, moving further step-by-step."
            </p>
        </section>
    }
}

#[component]
fn VentionArtWebScrappingPlatform() -> impl IntoView {
    view! {
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
            <p class="py-3">
                "A platform which provided users with extensive analytics on the art world,
                so that they could decide which artworks could not only become valuable assets to their collection,
                but also serve as a wise investment of theirs money."
            </p>
            <p>
                "This projects stands out as one of the important moments in my career. I was working there as a"
                <em class="font-bold">" software engineer"</em>
                ", and had possibility to step into fascinating realm of"
                <em class="font-bold">" distributed systems "</em>
                "for the first time. It marked the beginning of my journey in grasping the concepts of
                microservices and event-driven design."
            </p>
            <p>
                "Over the span of almost two years, I had the opportunity of not only"
                <em class="font-bold">" designing and establishing the architecture "</em>
                "of the web-scrapping pipelines, but also"
                <em class="font-bold">" leading a small team "</em>" of two people."
            </p>
        </section>
    }
}

#[component]
fn VentionBlockChainArtSellingPlatform() -> impl IntoView {
    view! {
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
            <p class="py-3">
                "This project was dedicated to the creation of an art platform that seamlessly
                merged the worlds of digital NFC and physical art. It included the trading and purchase
                of artworks through the use of cryptocurrencies, offering a modern way to engage with
                the art market."
            </p>
            <p>
                "I joined this project as a"
                <em class="font-bold">" full-stack engineer "</em>
                "for a short term to support the team with an upcoming release. I had to adapt quickly
                in order to assist the team in completing their work. I can’t say that I’ve learned
                blockchain and cryptocurrencies, but it was a great lesson for working under pressure
                and tight deadlines."
            </p>
        </section>
    }
}

#[component]
fn VentionFoodDeliveryPlatform() -> impl IntoView {
    view! {
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
            <p class="py-3">
                "This project brought a new culinary experience by providing fresh ingredients instead
                of pre-prepared meals. It introduced a fine-grained diet planner that emphasized health
                and nutrition. It not only simplified cooking, but also empowered individuals to make healthier
                dietary choices."
            </p>
            <p>
                "I worked on this project as a"
                <em class="font-bold">" frontend-end engineer"</em>
                ". I was responsible for the development of a control panel for managing orders,
                logistics, users, and stored ingredients. This project introduced me to the concept of"
                <em class="font-bold">" microfrontends"</em>"."
            </p>
        </section>
    }
}

#[component]
fn VentionEDUPlatform() -> impl IntoView {
    view! {
        <section class="flex flex-col gap-2">
            <div>
                <h4>"EDU Platform"</h4>
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
            <p class="py-3">
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
fn EducationSection() -> impl IntoView {
    view! {
        <section class="w-full">
            <h2 class="pb-3 md:pb-5">"Education"</h2>
            <div class="flex flex-col gap-8 md:gap-12">
                <UniversityBSUIR />
            </div>
        </section>
    }
}

#[component]
fn UniversityBSUIR() -> impl IntoView {
    view! {
        <article class="flex gap-5">
            <div class="w-12 h-12 md:w-32 md:h-32 print:w-12 print:h-12 flex-shrink-0 sticky print:static top-5">
                <img src="assets/bsuir.webp" alt="university logotype, radio-waves" />
            </div>
            <div class="flex flex-col gap-2">
                <div>
                    <h3>"BSUIR"</h3>
                    <p class="italic text-teal-500">"2017 — 2022"</p>
                </div>
                <p>
                    "Belarusian State University of Informatics and Radioelectronics"
                    <br />
                    "Faculty of Computer Systems and Networks"
                </p>
                <p>
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
fn DownloadProfileSection() -> impl IntoView {
    view! {
        <section class="py-8 md:py-10 print:hidden">
            <button class="btn-primary" on:click=move |_| { web_sys::window().map(|w| w.print()); }>
                <IconPrinter variant=IconVariant::Solid/>
                "Print Profile"
            </button>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full mx-auto bg-neutral-900 border-t-2 border-teal-700">
            <div class="max-w-screen-lg mx-auto p-5 py-8 md:px-10 md:py-8 flex items-center">
                <ul class="w-full flex flex-col md:flex-row print:flex-col justify-between gap-5 md:gap-10 print:gap-3">
                    <li>
                        <a class="flex items-center gap-2 print:hidden" href="https://github.com/ladihzey">
                            <div class="w-5 h-5">
                                <img src="assets/github.png" alt="github space-cat logo" />
                            </div>
                            "GitHub"
                        </a>
                        <p class="hidden print:inline-block">"https://github.com/ladihzey"</p>
                    </li>
                    <li class="flex-grow">
                        <a class="w-fit flex items-center gap-2 print:hidden" href="https://www.linkedin.com/in/yegor-zhidal-10530a1a7">
                            <div class="w-5 h-5">
                                <img src="assets/linkedin.png" alt="blue linkedin logo" />
                            </div>
                            "LinkedIn"
                        </a>
                        <p class="hidden print:inline-block">"https://www.linkedin.com/in/yegor-zhidal-10530a1a7"</p>
                    </li>
                    <li>
                        <a class="flex items-center gap-2 print:hidden" href="tel: +48452143483">
                            <span>"🇵🇱"</span>
                            "452-143-483"
                        </a>
                        <p class="hidden print:inline-block">"452-143-483"</p>
                    </li>
                    <li>
                        <a class="flex items-center gap-2 print:hidden" href="mailto: ladihzey@proton.me">
                            "ladihzey@proton.me"
                        </a>
                        <p class="hidden print:inline-block">"ladihzey@proton.me"</p>
                    </li>
                </ul>
            </div>
        </footer>
    }
}
