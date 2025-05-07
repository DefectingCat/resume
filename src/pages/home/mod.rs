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
        <main class="dark:bg-rua-gray-900 dark:text-rua-gray-100 h-full flex flex-col sm:justify-center sm:flex-row sm:py-8">
            <div class="shadow-xl w-full sm:max-w-6xl flex flex-col sm:flex-row" id="content">
                // left 1/3 width on pc
                <div class="w-full sm:w-1/3 bg-card-100 dark:bg-rua-gray-700 p-8">
                    <PersonInfo />
                </div>
                // right 2/3 width on pc
                <div class="w-full sm:w-2/3 px-8">
                    <ProjectInfo />
                </div>
            </div>
            <div class="relative sm:fixed inline-block text-left sm:left-10 sm:top-10 z-50 sm:flex sm:item-center">
                <DarkMode />
                <Download />
            </div>
        </main>
    }
}
