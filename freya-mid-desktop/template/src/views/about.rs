use freya::material_design::Ripple;
use freya::prelude::*;

use crate::app::AppState;
use crate::core::update::UpdateInfo;
use crate::theme::{theme_tokens, with_alpha, ThemeTokens};

#[derive(PartialEq)]
pub struct AboutView;

impl Component for AboutView {
    fn render(&self) -> impl IntoElement {
        let app_state = use_consume::<State<AppState>>();
        let update_info = use_consume::<State<Option<UpdateInfo>>>();

        let mut hover_update_btn = use_state(|| false);
        let hover_github = use_state(|| false);
        let hover_docs = use_state(|| false);
        let hover_license = use_state(|| false);

        let tokens = theme_tokens(app_state.read().theme_mode);
        let accent = app_state.read().accent_color;
        let is_hover_btn = *hover_update_btn.read();

        ScrollView::new()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .main_align(Alignment::Center)
                    .cross_align(Alignment::Center)
                    .child(
                        rect()
                            .direction(Direction::Vertical)
                            .spacing(16.0)
                            .cross_align(Alignment::Center)
                            .padding(Gaps::new_all(40.0))
                            .child(
                                rect()
                                    .width(Size::px(56.0))
                                    .height(Size::px(56.0))
                                    .corner_radius(14.0)
                                    .background(with_alpha(accent, 220))
                                    .main_align(Alignment::Center)
                                    .cross_align(Alignment::Center)
                                    .child(
                                        svg(include_bytes!("../../assets/logo.svg"))
                                            .width(Size::px(34.0))
                                            .height(Size::px(34.0))
                                            .color((255, 255, 255)),
                                    ),
                            )
                            .child(
                                label()
                                    .font_size(22.0)
                                    .font_weight(FontWeight::BOLD)
                                    .color(tokens.text_primary)
                                    .text(env!("CARGO_PKG_NAME").replace('-', " ").to_uppercase()),
                            )
                            .child(
                                label()
                                    .font_size(12.0)
                                    .color(tokens.text_muted)
                                    .text(format!("v{}  ·  Beta", env!("CARGO_PKG_VERSION"))),
                            )
                            .child(
                                label()
                                    .font_size(13.0)
                                    .color(tokens.text_secondary)
                                    .text(
                                        "A minimalist industrial desktop utility built with Freya.",
                                    ),
                            )
                            .child(
                                rect()
                                    .direction(Direction::Vertical)
                                    .spacing(8.0)
                                    .cross_align(Alignment::Center)
                                    .child(
                                        Ripple::new()
                                            .color((255u8, 255u8, 255u8, 80u8))
                                            .child(
                                                rect()
                                                    .padding(Gaps::new_symmetric(8.0, 20.0))
                                                    .background(if is_hover_btn {
                                                        with_alpha(accent, 255)
                                                    } else {
                                                        with_alpha(accent, 200)
                                                    })
                                                    .corner_radius(6.0)
                                                    .on_press(move |_| {
                                                        if let Some(info) =
                                                            update_info.read().as_ref()
                                                        {
                                                            if let Some(url) = &info.release_url {
                                                                let _ = open::that(url);
                                                            }
                                                        }
                                                    })
                                                    .on_pointer_enter(move |_| {
                                                        hover_update_btn.set(true);
                                                        Cursor::set(CursorIcon::Pointer);
                                                    })
                                                    .on_pointer_leave(move |_| {
                                                        hover_update_btn.set(false);
                                                        Cursor::set(CursorIcon::Default);
                                                    })
                                                    .child(
                                                        label()
                                                            .font_size(12.0)
                                                            .font_weight(FontWeight::BOLD)
                                                            .color((255, 255, 255))
                                                            .text("Check for Updates"),
                                                    ),
                                            ),
                                    )
                                    .maybe_child(if let Some(info) = update_info.read().as_ref() {
                                        Some(
                                            label()
                                                .font_size(12.0)
                                                .color(accent)
                                                .text(format!(
                                                    "v{} is available",
                                                    info.latest_version
                                                        .clone()
                                                        .unwrap_or_default()
                                                )),
                                        )
                                    } else {
                                        None
                                    }),
                            )
                            .child(
                                rect()
                                    .direction(Direction::Horizontal)
                                    .spacing(8.0)
                                    .cross_align(Alignment::Center)
                                    .child(
                                        Ripple::new()
                                            .color(with_alpha(accent, 60))
                                            .child(link_chip(
                                                "GitHub",
                                                tokens,
                                                accent,
                                                hover_github,
                                                "https://github.com/mocikadev/freya-mid-app",
                                            )),
                                    )
                                    .child(
                                        Ripple::new()
                                            .color(with_alpha(accent, 60))
                                            .child(link_chip(
                                                "Docs",
                                                tokens,
                                                accent,
                                                hover_docs,
                                                "https://github.com/mocikadev/freya-mid-app#readme",
                                            )),
                                    )
                                    .child(
                                        Ripple::new()
                                            .color(with_alpha(accent, 60))
                                            .child(link_chip(
                                                "MIT · Apache-2.0",
                                                tokens,
                                                accent,
                                                hover_license,
                                                "https://github.com/mocikadev/freya-mid-app/blob/main/LICENSE-MIT",
                                            )),
                                    ),
                            )
                            .child(
                                label()
                                    .font_size(11.0)
                                    .color(tokens.text_disabled)
                                    .text("Licensed under MIT OR Apache-2.0"),
                            ),
                    ),
            )
    }
}

fn link_chip(
    label_text: &'static str,
    tokens: ThemeTokens,
    accent: (u8, u8, u8),
    mut hover: State<bool>,
    url: &'static str,
) -> impl IntoElement {
    let is_hover = *hover.read();

    rect()
        .padding(Gaps::new_symmetric(4.0, 8.0))
        .background(if is_hover {
            with_alpha(tokens.bg_elevated, 255)
        } else {
            (255, 255, 255, 10)
        })
        .border(Border::new().width(1.0).fill(if is_hover {
            with_alpha(accent, 120)
        } else {
            tokens.border
        }))
        .corner_radius(6.0)
        .on_press(move |_| {
            let _ = open::that(url);
        })
        .on_pointer_enter(move |_| {
            hover.set(true);
            Cursor::set(CursorIcon::Pointer);
        })
        .on_pointer_leave(move |_| {
            hover.set(false);
            Cursor::set(CursorIcon::Default);
        })
        .child(
            label()
                .font_size(11.0)
                .font_weight(FontWeight::BOLD)
                .color(accent)
                .text(label_text),
        )
}
