use leptos::*;

#[component]
pub fn Avatar(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
) -> impl IntoView {
    view! {
        <div class="w-full h-full relative overflow-hidden rounded-full">
            <img
                class="absolute inset-0 w-full h-full object-cover"
                src=src
                alt=alt
            />
        </div>
    }
}
