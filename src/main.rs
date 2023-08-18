use leptos::*;

mod app;
mod components;

use app::App;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
