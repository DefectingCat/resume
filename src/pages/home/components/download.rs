use leptos::{logging, prelude::*};

use crate::app::download;

#[component]
pub fn Download() -> impl IntoView {
    async fn download_img() {
        download()
            .await
            .map_err(|e| logging::error!("download error: {e:?}"))
            .ok();
    }
    let download_img_action: Action<(), ()> = Action::new_unsync(|_: &()| download_img());
    let download_loading = download_img_action.pending();

    view! {
        <button
            type="button"
            class="inline-flex gap-x-1.5 justify-center items-center py-2 px-3 w-full text-sm font-semibold text-gray-900 bg-white rounded-md ring-1 ring-inset ring-gray-300 transition-all cursor-pointer dark:ring-gray-600 hover:bg-gray-50 disabled:bg-gray-100 disabled:cursor-not-allowed shadow-xs dark:bg-rua-gray-800 dark:text-word-300 dark:hover:bg-rua-gray-700 dark:disabled:bg-gray-800"
            id="menu-button"
            aria-expanded="true"
            aria-haspopup="true"
            on:click=move |_| {
                download_img_action.dispatch(());
            }
            disabled=move || download_loading
        >
            Download
            <img
                alt="loading"
                src="/public/svg/loading.svg"
                class="w-0 h-4 transition-all"
                class=("w-4", move || download_loading.get())
            />
        </button>
    }
}
