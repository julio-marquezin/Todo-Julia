use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

use crate::components::*;
use crate::pages::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="*" view=NotFound/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
