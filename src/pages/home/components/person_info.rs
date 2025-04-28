use leptos::prelude::*;

/// Personal information in left
#[component]
pub fn PersonInfo() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-card-300">
            <h1 class="text-3xl mb-2 font-semibold">
                李林军
            </h1>
            <div class="mb-1">
                全栈开发工程师
            </div>
            <div class="mb-1">
                男 / 2000.02
            </div>
            <div class="flex rounded-full font-semibold text-[#333] bg-card-200 px-2 w-max">
                3 年工作经验
            </div>
        </div>
    }
}
