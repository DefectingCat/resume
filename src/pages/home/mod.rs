use components::{person_info::PersonInfo, project_info::ProjectInfo};
use leptos::prelude::*;
use leptos_meta::*;

pub mod components;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="RUA" />
        // main background
        <main class="dark:bg-rua-gray-900 dark:text-rua-gray-100 h-full flex flex-col sm:justify-center sm:flex-row sm:py-8">
            <div class="shadow-lg w-full sm:max-w-5xl flex flex-col sm:flex-row">
                // left 1/3 width on pc
                <div class="w-full sm:w-1/3 bg-card-100 p-8">
                    <PersonInfo />
                </div>
                // right 2/3 width on pc
                <div>
                    <ProjectInfo />
                </div>
            </div>
        </main>
    }
}
