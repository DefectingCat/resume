use leptos::prelude::*;
use leptos_meta::*;

pub mod components;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="RUA" />
        <main class="dark:bg-rua-gray-900 dark:text-rua-gray-100 h-full py-8 flex justify-center">
            <div class="shadow-lg">
                <h1>"Hello, world!"</h1>
            </div>
        </main>
    }
}
