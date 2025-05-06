use leptos::prelude::*;

use crate::components::tag::Tag;

/// Project experience card
#[component]
pub fn ProjectCard(
    project_title: &'static str,
    project_tag: &'static str,
    project_stack: Vec<&'static str>,
    project_description: &'static str,
    project_duty: &'static str,
    project_duties: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="w-full mb-4 text-sm">
            <div class="mb-2 flex justify-between items-center font-semibold">
                <div class="text-2xl">{project_title}</div>
                <div class="rounded-full text-xs sm:text-sm px-4 py-1 bg-[#cfffd9] dark:bg-[#888eb3]">
                    {project_tag}
                </div>
            </div>
            <div class="mb-2">
                <span class="font-semibold leading-[2] sm:leading-0">"主要技术栈："</span>
                <div class="inline-flex items-center text-word-100 flex-wrap gap-y-2 dark:text-word-500">
                    {project_stack
                        .into_iter()
                        .map(|stack| view! { <Tag>{stack}</Tag> })
                        .collect::<Vec<_>>()}
                </div>
            </div>
            <div class="mb-2">
                <span class="font-semibold">"项目描述："</span>
                <span class="text-word-100 dark:text-word-500">{project_description}</span>
            </div>
            <div class="mb-1">
                <span class="font-semibold">"项目职责："</span>
                <span class="text-word-100 dark:text-word-500">{project_duty}</span>
            </div>
            <div class="px-4 text-word-100 dark:text-word-500">
                <ul>
                    {project_duties
                        .into_iter()
                        .map(|duty| view! { <li>{duty}</li> })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}
