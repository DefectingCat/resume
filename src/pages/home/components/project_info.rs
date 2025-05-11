use leptos::prelude::*;

use crate::{components::title::Title, pages::home::components::project_card::ProjectCard};

/// Project information in right
#[component]
pub fn ProjectInfo() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-card-300">
            // button title
            <div class="flex justify-center items-center w-full">
                <div class="h-[2px] bg-hr-100 flex-[1.1] dark:bg-hr-200"></div>
                <div class="flex-1 mx-4">
                    <Title>工作经历</Title>
                </div>
                <div class="h-[2px] bg-hr-100 flex-[1.1] dark:bg-hr-200"></div>
            </div>
            <div class="mb-2 w-full">
                <div class="mb-2 text-lg font-semibold">视尔信息科技有限公司</div>
                <div class="mb-2 text-sm font-semibold text-word-100 dark:text-word-500">
                    2022.02 - 至今
                </div>
                <div class="text-sm text-word-100 dark:text-word-500">
                    "工作内容：参与公司的主要平台开发与维护，主导平台的前端开发。参与平台的后端新功能开发以及维护。负责公司内部测试服务器以及线上服务器的运维部署工作。负责公司内部 GitLab 的维护以及 CI 搭建。独立开发公司内部工具。"
                </div>
            </div>
            <div class="mb-2 w-full">
                <div class="mb-2 text-lg font-semibold">浙深供应链</div>
                <div class="mb-2 text-sm font-semibold text-word-100 dark:text-word-500">
                    2021.04 - 2021.09
                </div>
                <div class="text-sm text-word-100 dark:text-word-500">
                    "工作内容：参与公司主要产品的模块开发，主导 B 端供应链系统的前端开发工作。同时负责公司内部的基础桌面运维。"
                </div>
            </div>

            // button title
            <div class="flex justify-center items-center w-full">
                <div class="h-[2px] bg-hr-100 flex-[1.1] dark:bg-hr-200"></div>
                <div class="flex-1 mx-4">
                    <Title>项目经验</Title>
                </div>
                <div class="h-[2px] bg-hr-100 flex-[1.1] dark:bg-hr-200"></div>
            </div>
            <div class="flex justify-center mb-2 w-full font-semibold text-word-100 dark:text-word-500">
                视尔信息科技有限公司 - 项目
            </div>
            <ProjectCard
                project_title="云汇展平台"
                project_tag="三维实时渲染平台"
                project_stack=vec![
                    "React",
                    "Redux",
                    "Three.js",
                    "WASM",
                    "Ant Design",
                    "TypeScript",
                    "Axios",
                ]
                project_description="使用 Three.js 技术实现的 3D 展台平台，具有多种功能，支持 3D 模型的渲染、全景图的展示模式。同时提供编辑端与展示端，用户可以在编辑端对三维场景进行编辑，并通过平台链接一键上架分享三维展台；对接微信平台，实现微信一键登录，卡片分享等功能；对接市面大模型服务，实现智能 AI 客服，在展台内通过语音、文字等方式与大模型交互，同时支持语音操作展台等功能；通过 WebSocket 技术多人联机在线浏览展台，并配合即构服务实现在线聊天、音视频通话功能。"
                project_duty="主要开发人员"
                project_duties=vec![
                    "编辑器 UI 开发，实现与 Three.js 的交互",
                    "通过接口联动，实现后台账号素材管理，展台的控制",
                    "通过 React i18n 实现多语言切换，以及通过调用实时翻译接口实现动态内容翻译",
                    "基于 html2canvas 与 canvas API 渲染图片以及保存功能",
                    "对接微信，实现扫码关注打开展台、扫码登录、移动端微信一键登录、微信分享等功能",
                    "使用 WASM、Web worker 等技术优化三维模型加载速度",
                    "使用 MediaStream API 配合大模型服务器实现语音控制展台",
                    "使用 WebSocket 配合即构服务实现的多人在线，音视频聊天",
                    "系统日常迭代开发、维护及优化",
                ]
            />
            <ProjectCard
                project_title="Gorra"
                project_tag="实时在线后端"
                project_stack=vec!["Golang", "WebSocket", "gorilla/websocket", "net/http", "Docker"]
                project_description="通过 Golang 的 net/http 实现的 RESTful API 服务，以及 gorilla/websocket 实现的多人实时在线功能；通过对 WebSocket 消息的封装，实现在同一个展台中对不同账号实现权限控制，实现主持人以及参会人员角色；通过与即构服务的对接，实现在三维场景中的实时投屏，视频串流等功能。基于 WebSocket 实现的文字聊天服务，用于在云汇展平台的大模型聊天中；"
                project_duty="独立开发"
                project_duties=vec![
                    "net/http 实现的 RESTful API 服务",
                    "gorilla/websocket 实现的多人实时在线功能",
                    "对接平台已有数据库，实现用户账号权限控制，会议控制等功能",
                    "基于 WebSocket 实现的文字聊天服务",
                    "负责线上的部署，版本的迭代管理",
                    "系统日常迭代开发、维护及优化",
                ]
            />
            <ProjectCard
                project_title="Ant Encrypt"
                project_tag="多端加密工具"
                project_stack=vec!["Rust", "WASM", "Trunk", "CLI", "Linux", "Windows"]
                project_description="使用 Rust 实现的加密工具，通过 WASM 技术实现在云汇展等平台中的对接，实现在浏览器中实时的加解密数据；通过静态链接的方式实现 CLI 工具，对接后端在 Linux 服务器上的实时加解密功能；通过 Trunk 构建 WASM 到私有 NPM 源，实现前端工具链可以无缝接入。"
                project_duty="独立开发"
                project_duties=vec![
                    "Rust 实现对字节的加密和解密",
                    "WASM 实现对浏览器中的加解密",
                    "CLI 实现对 Linux 服务器上的加解密",
                    "Trunk 构建 WASM 到私有 NPM 源",
                ]
            />
            <ProjectCard
                project_title="Three Library"
                project_tag="三维渲染库"
                project_stack=vec!["Three.js", "WASM", "TypeScript", "Web Worker"]
                project_description="通过对 Three.js 的封装，实现对云汇展等平台的三维渲染提供驱动。"
                project_duty="开发人员"
                project_duties=vec![
                    "优化项目的构建，实现部署到私有 NPM 源",
                    "通过 WASM、Web worker 等技术优化三维模型加载速度",
                ]
            />
        </div>
    }
}
