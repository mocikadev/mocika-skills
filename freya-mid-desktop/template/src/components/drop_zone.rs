use freya::icons;
use freya::prelude::*;

use crate::app::AppState;
use crate::theme::{theme_tokens, with_alpha, RADIUS_CARD};

#[derive(Clone, Copy, PartialEq, Eq)]
enum DropZoneState {
    Idle,
    Hover,
    Active,
}

#[derive(PartialEq)]
pub struct DropZone;

impl Component for DropZone {
    fn render(&self) -> impl IntoElement {
        let mut app_state = use_consume::<State<AppState>>();
        let tokens = theme_tokens(app_state.read().theme_mode);
        let accent = app_state.read().accent_color;
        let has_files = !app_state.read().dropped_files.is_empty();
        let is_hovering = app_state.read().is_file_hovering;

        let state = if is_hovering {
            DropZoneState::Hover
        } else if has_files {
            DropZoneState::Active
        } else {
            DropZoneState::Idle
        };

        let height = if state == DropZoneState::Active {
            120.0
        } else {
            220.0
        };

        let border_color = match state {
            DropZoneState::Idle => with_alpha(tokens.text_muted, 60),
            DropZoneState::Active => tokens.border,
            DropZoneState::Hover => with_alpha(accent, 255),
        };

        let bg = match state {
            DropZoneState::Hover => with_alpha(accent, 25),
            _ => with_alpha(tokens.bg_card, 255),
        };

        let caption = match state {
            DropZoneState::Idle => "Drag files here",
            DropZoneState::Hover => "Drop to process",
            DropZoneState::Active => "Drop more files",
        };

        let caption_color = match state {
            DropZoneState::Hover => accent,
            _ => tokens.text_muted,
        };

        let border_width = if state == DropZoneState::Hover { 1.5 } else { 1.0 };

        rect()
            .width(Size::fill())
            .height(Size::px(height))
            .background(bg)
            .border(Border::new().width(border_width).fill(border_color))
            .corner_radius(RADIUS_CARD)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .direction(Direction::Vertical)
            .spacing(8.0)
            .on_global_file_hover(move |_| {
                app_state.write().is_file_hovering = true;
            })
            .on_global_file_hover_cancelled(move |_| {
                app_state.write().is_file_hovering = false;
            })
            .on_file_drop(move |e: Event<FileEventData>| {
                if let Some(path) = e.file_path.clone() {
                    app_state.write().dropped_files.push(path.to_string_lossy().to_string());
                }
                app_state.write().is_file_hovering = false;
            })
            .on_press(move |_| {
                spawn(async move {
                    if let Some(files) = rfd::AsyncFileDialog::new().pick_files().await {
                        for f in files {
                            app_state
                                .write()
                                .dropped_files
                                .push(f.path().to_string_lossy().to_string());
                        }
                    }
                });
            })
            .on_pointer_enter(move |_| {
                Cursor::set(CursorIcon::Pointer);
            })
            .on_pointer_leave(move |_| {
                Cursor::set(CursorIcon::Default);
            })
            .child(
                svg(icons::lucide::upload())
                    .width(Size::px(24.0))
                    .height(Size::px(24.0))
                    .color(caption_color),
            )
            .child(
                label()
                    .font_size(13.0)
                    .font_weight(if state == DropZoneState::Hover {
                        FontWeight::BOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .color(caption_color)
                    .text(caption),
            )
            .maybe_child((state == DropZoneState::Idle).then(|| {
                label()
                    .font_size(11.0)
                    .color(with_alpha(tokens.text_muted, 150))
                    .text("or click to browse")
            }))
    }
}
