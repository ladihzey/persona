use leptos::*;
use BadgeVariant::*;

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
pub fn Badge(cx: Scope, children: Children, variant: BadgeVariant) -> impl IntoView {
    match variant {
        Gray => view! { cx,
            <span class="inline-flex items-center rounded-md bg-gray-600/40 px-2 py-1 text-xs font-medium text-gray-400 ring-1 ring-inset ring-gray-500/20">
                {children(cx)}
            </span>
        },
        Red => view! { cx,
            <span class="inline-flex items-center rounded-md bg-red-900/40 px-2 py-1 text-xs font-medium text-red-600 ring-1 ring-inset ring-red-500/20">
                {children(cx)}
            </span>
        },
        Yellow => view! { cx,
            <span class="inline-flex items-center rounded-md bg-yellow-800/40 px-2 py-1 text-xs font-medium text-yellow-600 ring-1 ring-inset ring-yellow-500/20">{children(cx)}
            </span>
        },
        Green => view! { cx,
            <span class="inline-flex items-center rounded-md bg-green-800/40 px-2 py-1 text-xs font-medium text-green-600 ring-1 ring-inset ring-green-500/20">
                {children(cx)}
            </span>
        },
        Blue => view! { cx,
            <span class="inline-flex items-center rounded-md bg-blue-800/40 px-2 py-1 text-xs font-medium text-blue-500 ring-1 ring-inset ring-blue-500/20">
                {children(cx)}
            </span>
        },
        Purple => view! { cx,
            <span class="inline-flex items-center rounded-md bg-purple-800/40 px-2 py-1 text-xs font-medium text-purple-500 ring-1 ring-inset ring-purple-500/20">
                {children(cx)}
            </span>
        },
        Pink => view! { cx,
            <span class="inline-flex items-center rounded-md bg-pink-800/40 px-2 py-1 text-xs font-medium text-pink-500 ring-1 ring-inset ring-pink-500/20">
                {children(cx)}
            </span>
        },
    }
}
