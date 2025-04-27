use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use leptos_use::{UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

use crate::{consts::COLOR_MODE, pages::home::Home};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Dark mode
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .emit_auto(true)
            .attribute("data-theme")
            .custom_modes(COLOR_MODE.iter().map(|m| m.to_string()).collect::<_>()),
    );
    provide_context((mode, set_mode));

    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </Routes>
        </Router>
    }
}
