use freya::prelude::*;
use freya::router::*;

use crate::components::activity_bar::ActivityBar;
use crate::core::update::{check_update, UpdateInfo};
use crate::theme::{theme_tokens, ThemeMode};
use crate::views::about::AboutView;
use crate::views::home::HomeView;
use crate::views::settings::SettingsView;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/", HomeView)]
        Home,
        #[route("/settings", SettingsView)]
        Settings,
        #[route("/about", AboutView)]
        About,
}

#[derive(Clone)]
pub struct AppState {
    pub dropped_files: Vec<String>,
    pub theme_mode: ThemeMode,
    pub accent_color: (u8, u8, u8),
    pub is_file_hovering: bool,
}

#[derive(PartialEq)]
struct AppLayout;

impl Component for AppLayout {
    fn render(&self) -> impl IntoElement {
        let mut update_info = use_state(|| None::<UpdateInfo>);
        let mut app_state = use_state(|| AppState {
            dropped_files: Vec::new(),
            theme_mode: ThemeMode::Dark,
            accent_color: (94, 106, 210),
            is_file_hovering: false,
        });

        use_provide_context(|| update_info);
        use_provide_context(|| app_state);

        use_hook(move || {
            let version = env!("CARGO_PKG_VERSION").to_string();
            spawn(async move {
                if let Some(info) = check_update(&version).await {
                    update_info.set(Some(info));
                }
            });
        });

        let mut freya_theme = use_init_theme(dark_theme);
        let current_mode = app_state.read().theme_mode;
        let mut last_synced_mode = use_state(|| current_mode);
        if *last_synced_mode.read() != current_mode {
            last_synced_mode.set(current_mode);
            freya_theme.set(match current_mode {
                ThemeMode::Light => light_theme(),
                ThemeMode::Dark | ThemeMode::Auto => dark_theme(),
            });
        }

        let tokens = theme_tokens(current_mode);

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .direction(Direction::Horizontal)
            .background(tokens.bg_stage)
            .on_file_drop(move |e: Event<FileEventData>| {
                if let Some(path) = e.file_path.clone() {
                    app_state
                        .write()
                        .dropped_files
                        .push(path.to_string_lossy().to_string());
                }
                app_state.write().is_file_hovering = false;
            })
            .child(ActivityBar {})
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .padding(Gaps::new_all(20.0))
                    .border(Border::new().width(1.0).fill(tokens.border))
                    .child(Outlet::<Route>::new()),
            )
    }
}

#[derive(PartialEq)]
pub struct AppRoot;

impl Component for AppRoot {
    fn render(&self) -> impl IntoElement {
        Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::Home))
    }
}

pub fn app() -> impl IntoElement {
    AppRoot {}
}
