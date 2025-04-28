use leptos::prelude::*;

use crate::components::title::Title;

/// Project information in right
#[component]
pub fn ProjectInfo() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center text-card-300">
            <div class="flex justify-center items-center w-full">
                <div class="h-[2px] bg-hr-100 flex-[1.1]"></div>
                <div class="flex-1 mx-4">
                    <Title>工作经历</Title>
                </div>
                <div class="h-[2px] bg-hr-100 flex-[1.1]"></div>
            </div>
            <div class="w-full mb-2">
                <div class="font-semibold text-lg mb-2">视尔信息科技有限公司</div>
                <div class="text-sm font-semibold text-word-100 mb-2">2022.02 - 至今</div>
                <div class="text-sm text-word-100">
                    "工作内容：参与公司的主要平台开发与维护，主导平台的前端开发。参与平台的后端新功能开发以及维护。负责公司内部测试服务器以及线上服务器的运维部署工作。负责公司内部 GitLab 的维护以及 CI 搭建。独立开发公司内部工具。"
                </div>
            </div>
            <div class="w-full mb-2">
                <div class="font-semibold text-lg mb-2">浙深供应链</div>
                <div class="text-sm font-semibold text-word-100 mb-2">2021.04 - 2021.09</div>
                <div class="text-sm text-word-100">
                    "工作内容：参与公司主要产品的模块开发，主导 B 端供应链系统的前端开发工作。同时负责公司内部的基础桌面运维。"
                </div>
            </div>
        </div>
    }
}
