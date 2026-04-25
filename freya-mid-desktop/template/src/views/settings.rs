use freya::icons;
use freya::material_design::Ripple;
use freya::prelude::*;

use crate::app::AppState;
use crate::components::color_picker_panel::ColorPickerPanel;
use crate::core::update::UpdateInfo;
use crate::theme::{theme_tokens, with_alpha, ThemeMode, ThemeTokens, DANGER_RED, RADIUS_CARD};

#[derive(PartialEq)]
pub struct SettingsView;

impl Component for SettingsView {
    fn render(&self) -> impl IntoElement {
        let update_info = use_consume::<State<Option<UpdateInfo>>>();
        let mut app_state = use_consume::<State<AppState>>();

        let hover_dark = use_state(|| false);
        let hover_light = use_state(|| false);
        let hover_auto = use_state(|| false);
        let mut hover_update = use_state(|| false);
        let mut show_color_popup = use_state(|| false);

        let tokens = theme_tokens(app_state.read().theme_mode);
        let accent = app_state.read().accent_color;
        let theme_mode = app_state.read().theme_mode;

        let on_download = move |_| {
            if let Some(info) = update_info.read().as_ref() {
                if let Some(url) = &info.release_url {
                    let _ = open::that(url);
                }
            }
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                ScrollView::new()                    .width(Size::fill())
                    .height(Size::fill())
                    .child(
                        rect()
                            .width(Size::fill())
                            .cross_align(Alignment::Center)
                            .child(
                                rect()
                                    .width(Size::fill())
                                    .max_width(Size::px(600.0))
                                    .padding(Gaps::new_all(24.0))
                                    .direction(Direction::Vertical)
                                    .spacing(12.0)
                                    .child(
                                        label()
                                            .font_size(16.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(tokens.text_primary)
                                            .text("Settings"),
                                    )
                                    .child(
                                        label()
                                            .margin(Gaps::new(4.0, 0.0, 0.0, 0.0))
                                            .font_size(11.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(tokens.text_muted)
                                            .text("APPEARANCE"),
                                    )
                                    .child(
                                        rect()
                                            .width(Size::fill())
                                            .padding(Gaps::new_symmetric(12.0, 14.0))
                                            .background(tokens.bg_card)
                                            .border(Border::new().width(1.0).fill(tokens.border))
                                            .corner_radius(RADIUS_CARD)
                                            .direction(Direction::Horizontal)
                                            .main_align(Alignment::SpaceBetween)
                                            .cross_align(Alignment::Center)
                                            .child(
                                                label()
                                                    .font_size(13.0)
                                                    .color(tokens.text_primary)
                                                    .text("Theme"),
                                            )
                                            .child(
                                                rect()
                                                    .direction(Direction::Horizontal)
                                                    .spacing(6.0)
                                                    .cross_align(Alignment::Center)
                                                    .child(
                                                        Ripple::new()
                                                            .color(with_alpha(accent, 60))
                                                            .child(theme_chip(
                                                                svg(icons::lucide::moon()),
                                                                "Dark",
                                                                theme_mode == ThemeMode::Dark,
                                                                tokens,
                                                                accent,
                                                                hover_dark,
                                                                move |_| {
                                                                    app_state.write().theme_mode =
                                                                        ThemeMode::Dark;
                                                                },
                                                            )),
                                                    )
                                                    .child(
                                                        Ripple::new()
                                                            .color(with_alpha(accent, 60))
                                                            .child(theme_chip(
                                                                svg(icons::lucide::sun()),
                                                                "Light",
                                                                theme_mode == ThemeMode::Light,
                                                                tokens,
                                                                accent,
                                                                hover_light,
                                                                move |_| {
                                                                    app_state.write().theme_mode =
                                                                        ThemeMode::Light;
                                                                },
                                                            )),
                                                    )
                                                    .child(
                                                        Ripple::new()
                                                            .color(with_alpha(accent, 60))
                                                            .child(theme_chip(
                                                                svg(icons::lucide::monitor()),
                                                                "Auto",
                                                                theme_mode == ThemeMode::Auto,
                                                                tokens,
                                                                accent,
                                                                hover_auto,
                                                                move |_| {
                                                                    app_state.write().theme_mode =
                                                                        ThemeMode::Auto;
                                                                },
                                                            )),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        rect()
                                            .width(Size::fill())
                                            .padding(Gaps::new_symmetric(12.0, 14.0))
                                            .background(tokens.bg_card)
                                            .border(Border::new().width(1.0).fill(tokens.border))
                                            .corner_radius(RADIUS_CARD)
                                            .direction(Direction::Horizontal)
                                            .main_align(Alignment::SpaceBetween)
                                            .cross_align(Alignment::Center)
                                            .child(
                                                label()
                                                    .font_size(13.0)
                                                    .color(tokens.text_primary)
                                                    .text("Accent Color"),
                                            )
                                            .child(
                                                rect()
                                                    .width(Size::px(40.0))
                                                    .height(Size::px(24.0))
                                                    .corner_radius(4.0)
                                                    .background(accent)
                                                    .border(Border::new().width(1.0).fill(tokens.border))
                                                    .on_press(move |_| {
                                                        show_color_popup.toggle();
                                                    })
                                                    .on_pointer_enter(move |_| {
                                                        Cursor::set(CursorIcon::Pointer);
                                                    })
                                                    .on_pointer_leave(move |_| {
                                                        Cursor::set(CursorIcon::Default);
                                                    }),
                                            ),
                                    )
                                    .child(
                                        label()
                                            .margin(Gaps::new(8.0, 0.0, 0.0, 0.0))
                                            .font_size(11.0)
                                            .font_weight(FontWeight::BOLD)
                                            .color(tokens.text_muted)
                                            .text("SYSTEM"),
                                    )
                                    .child(
                                        rect()
                                            .padding(Gaps::new_all(12.0))
                                            .background(tokens.bg_card)
                                            .border(Border::new().width(1.0).fill(tokens.border))
                                            .corner_radius(RADIUS_CARD)
                                            .direction(Direction::Vertical)
                                            .spacing(8.0)
                                            .child(
                                                label()
                                                    .font_size(11.0)
                                                    .color(tokens.text_muted)
                                                    .text(format!(
                                                        "VERSION: {}",
                                                        env!("CARGO_PKG_VERSION")
                                                    )),
                                            )
                                            .child(
                                                label()
                                                    .font_size(11.0)
                                                    .color(tokens.text_muted)
                                                    .text(format!(
                                                        "TOTAL DROPPED FILES: {}",
                                                        app_state.read().dropped_files.len()
                                                    )),
                                            )
                                            .maybe_child(
                                                if let Some(info) = update_info.read().as_ref() {
                                                    let is_hover_update = *hover_update.read();
                                                    Some(
                                                        rect()
                                                            .direction(Direction::Vertical)
                                                            .spacing(8.0)
                                                            .child(
                                                                label()
                                                                    .font_size(13.0)
                                                                    .color(DANGER_RED)
                                                                    .text(format!(
                                                                        "New Version Available: v{}",
                                                                        info.latest_version
                                                                            .clone()
                                                                            .unwrap_or_default()
                                                                    )),
                                                            )
                                                            .child(
                                                                Ripple::new()
                                                                    .color((255u8, 255u8, 255u8, 80u8))
                                                                    .child(
                                                                        rect()
                                                                            .width(Size::px(160.0))
                                                                            .padding(Gaps::new_symmetric(8.0, 12.0))
                                                                            .background(if is_hover_update {
                                                                                with_alpha(accent, 255)
                                                                            } else {
                                                                                with_alpha(accent, 40)
                                                                            })
                                                                            .border(Border::new().width(1.0).fill(with_alpha(accent, 255)))
                                                                            .corner_radius(6.0)
                                                                            .on_press(on_download)
                                                                            .on_pointer_enter(move |_| {
                                                                                hover_update.set(true);
                                                                                Cursor::set(CursorIcon::Pointer);
                                                                            })
                                                                            .on_pointer_leave(move |_| {
                                                                                hover_update.set(false);
                                                                                Cursor::set(CursorIcon::Default);
                                                                            })
                                                                            .child(
                                                                                label()
                                                                                    .font_size(11.0)
                                                                                    .font_weight(FontWeight::BOLD)
                                                                                    .color(tokens.text_primary)
                                                                                    .text("CHECK UPDATE"),
                                                                            ),
                                                                    ),
                                                            ),
                                                    )
                                                } else {
                                                    None
                                                },
                                            ),
                                    ),
                            ),
                    ),
            )
            .child(
                Popup::new()
                    .show(show_color_popup())
                    .width(Size::px(260.0))
                    .on_close_request(move |_| show_color_popup.set(false))
                    .child(
                        ColorPickerPanel::new(move |c: Color| {
                            app_state.write().accent_color = (c.r(), c.g(), c.b());
                        })
                        .value(Color::from_rgb(accent.0, accent.1, accent.2))
                        .width(Size::px(240.0)),
                    ),
            )
    }
}

fn theme_chip(
    icon: Svg,
    chip_label: &'static str,
    active: bool,
    tokens: ThemeTokens,
    accent: (u8, u8, u8),
    mut hover: State<bool>,
    on_press: impl FnMut(Event<PressEventData>) + 'static,
) -> impl IntoElement {
    let is_hover = *hover.read();

    rect()
        .padding(Gaps::new_symmetric(5.0, 8.0))
        .corner_radius(6.0)
        .direction(Direction::Horizontal)
        .spacing(4.0)
        .cross_align(Alignment::Center)
        .background(if active {
            with_alpha(accent, 35)
        } else if is_hover {
            (255, 255, 255, 20)
        } else {
            (255, 255, 255, 10)
        })
        .border(Border::new().width(1.0).fill(if active {
            with_alpha(accent, 200)
        } else {
            tokens.border
        }))
        .on_press(on_press)
        .on_pointer_enter(move |_| {
            hover.set(true);
            Cursor::set(CursorIcon::Pointer);
        })
        .on_pointer_leave(move |_| {
            hover.set(false);
            Cursor::set(CursorIcon::Default);
        })
        .child(
            icon.width(Size::px(12.0))
                .height(Size::px(12.0))
                .color(if active {
                    with_alpha(accent, 255)
                } else {
                    with_alpha(tokens.text_muted, 255)
                }),
        )
        .child(
            label()
                .font_size(10.0)
                .font_weight(if active { FontWeight::BOLD } else { FontWeight::NORMAL })
                .color(if active {
                    with_alpha(accent, 255)
                } else {
                    with_alpha(tokens.text_muted, 255)
                })
                .text(chip_label),
        )
}
