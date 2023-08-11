use leptos::*;

#[component]
pub fn Button(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <button class="text-neutral-900 bg-teal-500 hover:bg-teal-700 active:bg-teal-800 flex gap-2 font-bold py-2 px-4 rounded">
            {children(cx)}
        </button>
    }
}
