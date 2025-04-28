use leptos::prelude::*;

/// Tag component
#[component]
pub fn Tag(children: Children) -> impl IntoView {
    view! {
        <div class="border border-word-200 rounded-md px-1 py-[2px] text-xs mr-2 last:mr-0">
            {children()}
        </div>
    }
}
