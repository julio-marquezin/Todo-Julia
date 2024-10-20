use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-screen">
            <div class="text-6xl">404</div>
            <div class="text-2xl">Page not found</div>
        </div>
    }
}
