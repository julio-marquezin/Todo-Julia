use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-3xl font-bold underline">
                "Welcome to Tauri + Leptos!"
            </h1>
        </div>
    }
}
