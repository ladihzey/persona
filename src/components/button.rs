use leptos::*;

#[component]
pub fn Button(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <button class="text-stone-100 bg-teal-700 hover:bg-teal-800 active:bg-teal-900 flex gap-2 font-bold py-2 px-4 rounded">
            {children(cx)}
        </button>
    }
}
