use leptos::prelude::*;

/// Title component
#[component]
pub fn Title(children: Children) -> impl IntoView {
    view! {
        <div class="flex justify-center items-center py-2 px-16 my-4 w-max text-xl font-semibold rounded-full bg-card-200 text-card-300 dark:text-card-400">
            {children()}
        </div>
    }
}
