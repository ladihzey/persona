use leptos::*;

#[component]
pub fn Avatar(
    cx: Scope,
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
) -> impl IntoView {
    view! { cx,
        <div class="w-full h-full relative overflow-hidden rounded-full">
            <img
                class="absolute inset-0 w-full h-full object-cover"
                src=src
                alt=alt
            />
        </div>
    }
}
