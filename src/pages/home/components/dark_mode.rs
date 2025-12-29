use leptos::prelude::*;
use leptos_use::ColorMode;

#[component]
pub fn DarkMode(class: Option<&'static str>) -> impl IntoView {
    let (mode, set_mode) = use_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>()
        .expect("cannot access color mode");
    let (show, set_show) = signal(false);

    view! {
        <div class=move || {
            if let Some(class) = class { format!("{class} relative") } else { "relative".into() }
        }>
            <button
                type="button"
                class="inline-flex gap-x-1.5 justify-center py-2 px-3 w-full text-sm font-semibold text-gray-900 bg-white rounded-md ring-1 ring-inset ring-gray-300 transition-all cursor-pointer dark:ring-gray-600 hover:bg-gray-50 shadow-xs dark:bg-rua-gray-800 dark:text-word-300 dark:hover:bg-rua-gray-700"
                on:click=move |_| { set_show.set(!show.get()) }
                id="menu-button"
                aria-expanded="true"
                aria-haspopup="true"
            >
                Theme
                <svg
                    class="-mr-1 text-gray-400 size-5"
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
                class="absolute right-0 z-10 mt-2 w-full bg-white rounded-md ring-1 shadow-lg transition duration-100 ease-out origin-top-right dark:ring-gray-600 ring-black/5 dark:bg-rua-gray-800 dark:text-word-300 focus:outline-hidden"
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
                        on:click=move |_| set_mode.set(ColorMode::Auto)
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
                        on:click=move |_| set_mode.set(ColorMode::Dark)
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
                        on:click=move |_| set_mode.set(ColorMode::Light)
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
