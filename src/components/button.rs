use leptos::*;

#[component]
pub fn Button(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <button class="text-gray-800 bg-emerald-400 hover:bg-emerald-600 flex gap-2 font-bold py-2 px-4 rounded">
            {children(cx)}
        </button>
    }
}
