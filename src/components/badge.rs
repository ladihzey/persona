use leptos::*;
use BadgeVariant::*;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum BadgeVariant {
    Gray,
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

#[component]
pub fn Badge(children: Children, variant: BadgeVariant) -> impl IntoView {
    match variant {
        Gray => view! {
            <span class="inline-flex items-center rounded-md bg-gray-600/40 px-1 md:px-2 py-1 text-xs font-medium text-gray-400 ring-1 ring-inset ring-gray-500/20">
                {children()}
            </span>
        },
        Red => view! {
            <span class="inline-flex items-center rounded-md bg-red-900/40 px-1 md:px-2 py-1 text-xs font-medium text-red-600 ring-1 ring-inset ring-red-500/20">
                {children()}
            </span>
        },
        Yellow => view! {
            <span class="inline-flex items-center rounded-md bg-yellow-800/40 px-1 md:px-2 py-1 text-xs font-medium text-yellow-600 ring-1 ring-inset ring-yellow-500/20">
                {children()}
            </span>
        },
        Green => view! {
            <span class="inline-flex items-center rounded-md bg-green-800/40 px-1 md:px-2 py-1 text-xs font-medium text-green-600 ring-1 ring-inset ring-green-500/20">
                {children()}
            </span>
        },
        Blue => view! {
            <span class="inline-flex items-center rounded-md bg-blue-800/40 px-1 md:px-2 py-1 text-xs font-medium text-blue-500 ring-1 ring-inset ring-blue-500/20">
                {children()}
            </span>
        },
        Purple => view! {
            <span class="inline-flex items-center rounded-md bg-purple-800/40 px-1 md:px-2 py-1 text-xs font-medium text-purple-500 ring-1 ring-inset ring-purple-500/20">
                {children()}
            </span>
        },
        Pink => view! {
            <span class="inline-flex items-center rounded-md bg-pink-800/40 px-1 md:px-2 py-1 text-xs font-medium text-pink-500 ring-1 ring-inset ring-pink-500/20">
                {children()}
            </span>
        },
    }
}

#[component]
pub fn BadgeGroup(badges: &'static [(BadgeVariant, &'static str)]) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <For
                each=move || badges.iter()
                key=|(_, label)| label
                children=move |(variant, label)| {
                    view! {
                        <Badge variant={*variant}>{label}</Badge>
                    }
                }
            />
        </div>
    }
}
