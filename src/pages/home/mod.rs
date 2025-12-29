use components::{
    dark_mode::DarkMode, download::Download, person_info::PersonInfo, project_info::ProjectInfo,
};
use leptos::prelude::*;
use leptos_meta::*;

pub mod components;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="RUA" />
        // main background
        <main class="flex flex-col h-full sm:flex-row sm:justify-center sm:py-8 dark:bg-rua-gray-900 dark:text-rua-gray-100">
            <div class="flex flex-col w-full shadow-xl sm:flex-row sm:max-w-6xl" id="content">
                // left 1/3 width on pc
                <div class="p-8 w-full sm:w-1/3 bg-card-100 dark:bg-rua-gray-700">
                    <PersonInfo />
                </div>
                // right 2/3 width on pc
                <div class="px-8 w-full sm:w-2/3 dark:bg-rua-gray-900">
                    <ProjectInfo />
                </div>
            </div>
            <div class="inline-block relative z-50 text-left sm:flex sm:fixed sm:top-10 sm:left-10 sm:item-center">
                <DarkMode class=Some("sm:mr-2") />
                <Download />
            </div>
        </main>
    }
}
