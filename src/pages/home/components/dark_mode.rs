use leptos::prelude::*;
use leptos_use::ColorMode;

#[component]
pub fn DarkMode(class: Option<&'static str>) -> impl IntoView {
    let (mode, set_mode) = use_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>()
        .expect("cannot access color mode");
    let (show, set_show) = signal(false);

    view! {
        <div class=move || {
            if let Some(class) = class {
                format!("{class} relative" )
            } else {
                "relative".into()
            }
        }>
            <button
                type="button"
                class="inline-flex w-full justify-center gap-x-1.5 rounded-md dark:bg-rua-gray-800 dark:text-word-300 dark:ring-gray-600 dark:hover:bg-rua-gray-700 transition-all bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-xs ring-1 ring-gray-300 ring-inset hover:bg-gray-50 cursor-pointer"
                on:click=move |_| set_show(!show.get())
                id="menu-button"
                aria-expanded="true"
                aria-haspopup="true"
            >
                Theme
                <svg
                    class="-mr-1 size-5 text-gray-400"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                    data-slot="icon"
                >
                    <path
                        fill-rule="evenodd"
                        d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
                        clip-rule="evenodd"
                    />
                </svg>
            </button>

            <div
                class="absolute right-0 z-10 mt-2 w-full origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-hidden dark:bg-rua-gray-800 dark:text-word-300 dark:ring-gray-600 transition ease-out duration-100"
                class=(["transform", "opacity-0", "scale-95"], move || !show.get())
                class=(["transform", "opacity-100", "scale-100"], move || show.get())
                class=("pointer-events-none", move || !show.get())
                class=("pointer-events-auto", move || show.get())
                role="menu"
                aria-orientation="vertical"
                aria-labelledby="menu-button"
                tabindex="-1"
            >
                <div class="py-1" role="none">
                    <a
                        href="#"
                        class="block px-4 py-2 text-sm text-gray-700 dark:text-word-300 dark:hover:text-word-400 dark:hover:bg-rua-gray-700 no-underline! hover:bg-gray-100 hover:text-gray-900 hover:outline-hidden transition-all"
                        class=(
                            [
                                "bg-gray-100",
                                "text-gray-900",
                                "outline-hidden",
                                "dark:bg-rua-gray-700",
                            ],
                            move || mode.get() == ColorMode::Auto,
                        )
                        role="menuitem"
                        on:click=move |_| set_mode(ColorMode::Auto)
                        tabindex="-1"
                        id="menu-item-0"
                    >
                        Auto
                    </a>
                    <a
                        href="#"
                        class="block px-4 py-2 text-sm text-gray-700 dark:text-word-300 dark:hover:text-word-400 dark:hover:bg-rua-gray-700 no-underline! hover:bg-gray-100 hover:text-gray-900 hover:outline-hidden transition-all"
                        class=(
                            [
                                "bg-gray-100",
                                "text-gray-900",
                                "outline-hidden",
                                "dark:bg-rua-gray-700",
                            ],
                            move || mode.get() == ColorMode::Dark,
                        )
                        on:click=move |_| set_mode(ColorMode::Dark)
                        role="menuitem"
                        tabindex="-1"
                        id="menu-item-1"
                    >
                        Dark
                    </a>
                    <a
                        href="#"
                        class="block px-4 py-2 text-sm text-gray-700 dark:text-word-300 dark:hover:text-word-400 dark:hover:bg-rua-gray-700 no-underline! hover:bg-gray-100 hover:text-gray-900 hover:outline-hidden transition-all"
                        class=(
                            [
                                "bg-gray-100",
                                "text-gray-900",
                                "outline-hidden",
                                "dark:bg-rua-gray-700",
                            ],
                            move || mode.get() == ColorMode::Light,
                        )
                        on:click=move |_| set_mode(ColorMode::Light)
                        role="menuitem"
                        tabindex="-1"
                        id="menu-item-2"
                    >
                        Light
                    </a>
                </div>
            </div>
        </div>
    }
}
