use freya::icons;
use freya::material_design::Ripple;
use freya::prelude::*;
use freya::router::*;

use crate::app::{AppState, Route};
use crate::core::update::UpdateInfo;
use crate::theme::{theme_tokens, with_alpha, ThemeTokens, DANGER_RED};

#[derive(PartialEq)]
pub struct ActivityBar;

impl Component for ActivityBar {
    fn render(&self) -> impl IntoElement {
        let current_route = use_route::<Route>();
        let update_info = use_consume::<State<Option<UpdateInfo>>>();
        let app_state = use_consume::<State<AppState>>();

        let has_update = update_info.read().is_some();
        let tokens = theme_tokens(app_state.read().theme_mode);
        let accent = app_state.read().accent_color;

        let hover_home = use_state(|| false);
        let hover_settings = use_state(|| false);
        let hover_about = use_state(|| false);

        rect()
            .width(Size::px(56.0))
            .height(Size::fill())
            .background(tokens.bg_nav)
            .border(Border::new().width(1.0).fill(tokens.border))
            .direction(Direction::Vertical)
            .main_align(Alignment::SpaceBetween)
            .child(
                rect()
                    .width(Size::fill())
                    .direction(Direction::Vertical)
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::px(56.0))
                            .main_align(Alignment::Center)
                            .cross_align(Alignment::Center)
                            .child(
                                rect()
                                    .width(Size::px(36.0))
                                    .height(Size::px(36.0))
                                    .corner_radius(8.0)
                                    .background(with_alpha(accent, 220))
                                    .main_align(Alignment::Center)
                                    .cross_align(Alignment::Center)
                                    .child(
                                        svg(include_bytes!("../../assets/logo.svg"))
                                            .width(Size::px(24.0))
                                            .height(Size::px(24.0))
                                            .color((255u8, 255u8, 255u8)),
                                    ),
                            ),
                    )
                    .child(
                        Ripple::new()
                            .width(Size::fill())
                            .color(with_alpha(tokens.text_muted, 40))
                            .child(nav_item(
                                "Home",
                                svg(icons::lucide::house()),
                                current_route == Route::Home,
                                accent,
                                tokens,
                                hover_home,
                                move |_| {
                                    let _ = RouterContext::get().replace(Route::Home);
                                },
                                false,
                            )),
                    ),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .direction(Direction::Vertical)
                    .padding(Gaps::new(0.0, 0.0, 12.0, 0.0))
                    .child(
                        Ripple::new()
                            .width(Size::fill())
                            .color(with_alpha(tokens.text_muted, 40))
                            .child(nav_item(
                                "Settings",
                                svg(icons::lucide::settings()),
                                current_route == Route::Settings,
                                accent,
                                tokens,
                                hover_settings,
                                move |_| {
                                    let _ = RouterContext::get().replace(Route::Settings);
                                },
                                has_update,
                            )),
                    )
                    .child(
                        Ripple::new()
                            .width(Size::fill())
                            .color(with_alpha(tokens.text_muted, 40))
                            .child(nav_item(
                                "About",
                                svg(icons::lucide::info()),
                                current_route == Route::About,
                                accent,
                                tokens,
                                hover_about,
                                move |_| {
                                    let _ = RouterContext::get().replace(Route::About);
                                },
                                false,
                            )),
                    ),
            )
    }
}

fn nav_item(
    label_text: &'static str,
    icon: Svg,
    active: bool,
    accent: (u8, u8, u8),
    tokens: ThemeTokens,
    mut hover: State<bool>,
    on_click: impl FnMut(Event<PressEventData>) + 'static,
    show_dot: bool,
) -> impl IntoElement {
    let is_hover = *hover.read();

    rect()
        .width(Size::fill())
        .height(Size::px(40.0))
        .direction(Direction::Horizontal)
        .cross_align(Alignment::Center)
        .background(if active {
            with_alpha(accent, 20)
        } else if is_hover {
            with_alpha(tokens.bg_elevated, 255)
        } else {
            (0, 0, 0, 0)
        })
        .on_press(on_click)
        .on_pointer_enter(move |_| {
            hover.set(true);
            Cursor::set(CursorIcon::Pointer);
        })
        .on_pointer_leave(move |_| {
            hover.set(false);
            Cursor::set(CursorIcon::Default);
        })
        .child(
            rect()
                .width(Size::px(3.0))
                .height(Size::fill())
                .corner_radius(99.0)
                .background(if active {
                    with_alpha(accent, 255)
                } else {
                    (0, 0, 0, 0)
                }),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::fill())
                .direction(Direction::Vertical)
                .cross_align(Alignment::Center)
                .main_align(Alignment::Center)
                .spacing(3.0)
                .child(
                    rect()
                        .width(Size::px(20.0))
                        .height(Size::px(20.0))
                        .main_align(Alignment::Center)
                        .cross_align(Alignment::Center)
                        .child(
                            icon.width(Size::px(16.0))
                                .height(Size::px(16.0))
                                .color(if active {
                                    with_alpha(accent, 255)
                                } else {
                                    with_alpha(tokens.text_muted, 255)
                                }),
                        )
                        .maybe_child(if show_dot {
                            Some(
                                rect()
                                    .position(Position::new_absolute())
                                    .offset_x(10.0)
                                    .offset_y(-10.0)
                                    .width(Size::px(6.0))
                                    .height(Size::px(6.0))
                                    .background(DANGER_RED)
                                    .corner_radius(3.0),
                            )
                        } else {
                            None
                        }),
                )
                .child(
                    label()
                        .font_size(10.0)
                        .font_weight(if active {
                            FontWeight::BOLD
                        } else {
                            FontWeight::NORMAL
                        })
                        .color(if active {
                            with_alpha(accent, 255)
                        } else {
                            with_alpha(tokens.text_muted, 255)
                        })
                        .text(label_text),
                ),
        )
}
