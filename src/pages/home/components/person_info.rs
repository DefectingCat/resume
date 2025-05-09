use leptos::prelude::*;

use crate::components::title::Title;

/// Personal information in left
#[component]
pub fn PersonInfo() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-word-100 dark:text-word-300">
            <h1 class="text-3xl mb-2 font-semibold">李林军</h1>
            <div class="mb-1">全栈开发工程师</div>
            <div class="mb-2 text-sm">男 / 2000.02</div>
            <div class="flex rounded-full font-semibold text-word-200 dark:text-word-400 bg-card-200 px-2 w-max">
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
                    "https://rua.plus"
                </a>
            </div>

            <Title>教育背景</Title>
            <div class="flex flex-col items-center text-sm text-word-200 dark:text-word-400">
                <div class="mb-1">安徽新华学院</div>
                <div class="mb-1">2017.03 - 2019.07</div>
                <div>计算机网络-专科</div>
            </div>

            <Title>技能水平</Title>
            <div class="flex flex-col items-center text-sm px-4">
                <ul>
                    <li class="mb-2">
                        "扎实的 JavaScript/TypeScript 基础，了解 ES6+ 新特性"
                    </li>
                    <li class="mb-2">
                        "扎实的 Rust 基础，对 WebAssembly、CLI 以及后端有过实际开发经验"
                    </li>
                    <li class="mb-2">
                        "掌握 React 框架，且理解其工作原理；熟悉 Vue 框架，有过实际开发经验"
                    </li>
                    <li class="mb-2">
                        "熟悉使用 Golang 进行后端开发，对 WebSocket 过实际开发经验"
                    </li>
                    <li class="mb-2">
                        "熟悉 Linux 及其多种发行版，熟悉 Docker，有服务器以及 CI 的维护经验"
                    </li>
                    <li class="mb-2">
                        "熟悉 HTTP 协议，有过 HTTP 服务器的开发经验（Rust）"
                    </li>
                    <li class="mb-2">
                        "对桌面端开发有一定了解，使用 Tauri 开发过桌面端应用"
                    </li>
                    <li class="mb-2">"对计算机基本工作原理有一定了解"</li>
                </ul>
            </div>

            <Title>个人评价</Title>
            <div class="flex flex-col items-center text-sm px-4">
                <ul>
                    <li class="mb-2">
                        "拥有良好的代码编写习惯，对代码质量有自我约束。"
                    </li>
                    <li class="mb-2">
                        "较强的自我驱动学习能力, 乐于钻研学习新技术, 时刻关注行业动态拓展视野"
                    </li>
                    <li class="mb-2">"热爱开源，对开源项目有深入的理解"</li>
                </ul>
            </div>
        </div>
    }
}
