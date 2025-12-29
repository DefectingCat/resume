use leptos::prelude::*;

/// Tag component
#[component]
pub fn Tag(children: Children) -> impl IntoView {
    view! {
        <div class="px-1 mr-2 text-xs rounded-md border last:mr-0 border-word-200 py-[2px]">
            {children()}
        </div>
    }
}
