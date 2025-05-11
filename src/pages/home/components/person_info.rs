use leptos::prelude::*;

use crate::components::title::Title;

/// Personal information in left
#[component]
pub fn PersonInfo() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-word-100 dark:text-word-300">
            <h1 class="mb-2 text-3xl font-semibold">李林军</h1>
            <div class="mb-1">全栈开发工程师</div>
            <div class="mb-2 text-sm">男 / 2000.02</div>
            <div class="flex px-2 w-max font-semibold rounded-full text-word-200 bg-card-200 dark:text-word-400">
                3 年工作经验
            </div>
            // contact
            <div class="mt-3">
                <div class="flex items-center mb-2">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 16 16"
                    >
                        <path
                            fill="#33477c"
                            d="M4 3a2 2 0 0 0-2 2v.201l6 3.231l6-3.23V5a2 2 0 0 0-2-2zm10 3.337L8.237 9.44a.5.5 0 0 1-.474 0L2 6.337V11a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2z"
                        />
                    </svg>
                    <a href="mailto:i@rua.plus" class="ml-2">
                        i@rua.plus
                    </a>
                </div>
                <div class="flex items-center mb-2">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 32 32"
                    >
                        <path
                            fill="#33477c"
                            d="M10.25 2A3.25 3.25 0 0 0 7 5.25v21.5A3.25 3.25 0 0 0 10.25 30h11.5A3.25 3.25 0 0 0 25 26.75V5.25A3.25 3.25 0 0 0 21.75 2zM14 24h4a1 1 0 1 1 0 2h-4a1 1 0 1 1 0-2"
                        />
                    </svg>
                    <a href="tel:19840170636" class="ml-2">
                        19840170636
                    </a>
                </div>
                <div class="flex items-center mb-2">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill="#33477c"
                            fill-rule="evenodd"
                            d="M12.006 2a9.85 9.85 0 0 0-6.484 2.44a10.32 10.32 0 0 0-3.393 6.17a10.48 10.48 0 0 0 1.317 6.955a10.05 10.05 0 0 0 5.4 4.418c.504.095.683-.223.683-.494c0-.245-.01-1.052-.014-1.908c-2.78.62-3.366-1.21-3.366-1.21a2.7 2.7 0 0 0-1.11-1.5c-.907-.637.07-.621.07-.621c.317.044.62.163.885.346c.266.183.487.426.647.71c.135.253.318.476.538.655a2.08 2.08 0 0 0 2.37.196c.045-.52.27-1.006.635-1.37c-2.219-.259-4.554-1.138-4.554-5.07a4.02 4.02 0 0 1 1.031-2.75a3.77 3.77 0 0 1 .096-2.713s.839-.275 2.749 1.05a9.26 9.26 0 0 1 5.004 0c1.906-1.325 2.74-1.05 2.74-1.05c.37.858.406 1.828.101 2.713a4.02 4.02 0 0 1 1.029 2.75c0 3.939-2.339 4.805-4.564 5.058a2.47 2.47 0 0 1 .679 1.897c0 1.372-.012 2.477-.012 2.814c0 .272.18.592.687.492a10.05 10.05 0 0 0 5.388-4.421a10.47 10.47 0 0 0 1.313-6.948a10.32 10.32 0 0 0-3.39-6.165A9.85 9.85 0 0 0 12.007 2Z"
                            clip-rule="evenodd"
                        />
                    </svg>
                    <a href="https://github.com/DefectingCat" target="_blank" class="ml-2">
                        Sonetto
                    </a>
                </div>
            </div>
            <div class="flex flex-col items-center mt-3">
                <div class="mb-1 font-semibold">个人博客</div>
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
            <div class="flex flex-col items-center px-4 text-sm">
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
            <div class="flex flex-col items-center px-4 text-sm">
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
