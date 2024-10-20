use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-blue-500 text-white p-2 flex justify-center">
            <h1>"Welcome to Tauri!"</h1>
        </header>
    }
}
