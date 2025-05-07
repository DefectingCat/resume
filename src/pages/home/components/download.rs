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
    let download_img_action: Action<(), (), SyncStorage> =
        Action::new_unsync(|_: &()| async move {
            download_img().await;
        });
    let download_loading = download_img_action.pending();

    view! {
        <button
            type="button"
            class="inline-flex w-full justify-center gap-x-1.5 rounded-md dark:bg-rua-gray-800 dark:text-word-300 dark:ring-gray-600 dark:hover:bg-rua-gray-700 transition-all bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-xs ring-1 ring-gray-300 ring-inset hover:bg-gray-50 cursor-pointer"
            id="menu-button"
            aria-expanded="true"
            aria-haspopup="true"
            on:click=move |_| {
                download_img_action.dispatch(());
            }
            disabled=move || download_loading
        >
            Download
        </button>
    }
}
