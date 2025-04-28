use leptos::prelude::*;

/// Personal information in left
#[component]
pub fn PersonInfo() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-card-300">
            <h1 class="text-3xl mb-2 font-semibold">李林军</h1>
            <div class="mb-1">全栈开发工程师</div>
            <div class="mb-2 text-sm">男 / 2000.02</div>
            <div class="flex rounded-full font-semibold text-[#333] bg-card-200 px-2 w-max">
                3 年工作经验
            </div>
            // contact
            <div class="mt-3">
                <div class="flex items-center mb-2">
                    <img src="public/svg/email.svg" class="w-6 h-6 mr-2" />
                    <a href="mailto:i@rua.plus">i@rua.plus</a>
                </div>
                <div class="flex items-center mb-2">
                    <img src="public/svg/phone.svg" class="w-6 h-6 mr-2" />
                    <a href="tel:19840170636">19840170636</a>
                </div>
                <div class="flex items-center mb-2">
                    <img src="public/svg/github.svg" class="w-6 h-6 mr-2" />
                    <a href="https://github.com/DefectingCat" target="_blank">
                        Sonetto
                    </a>
                </div>
            </div>
            <div class="mt-3 flex flex-col items-center">
                <div class="font-semibold mb-1">个人博客</div>
                <a href="https://rua.plus" class="text-sm" target="_blank">
                    {"https://rua.plus"}
                </a>
            </div>
        </div>
    }
}
