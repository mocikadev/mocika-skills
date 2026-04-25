use freya::prelude::*;
use crate::app::AppState;
use crate::components::drop_zone::DropZone;
use crate::theme::{theme_tokens, with_alpha};

#[derive(PartialEq)]
pub struct HomeView;

impl Component for HomeView {
    fn render(&self) -> impl IntoElement {
        let state = use_consume::<State<AppState>>();
        let tokens = theme_tokens(state.read().theme_mode);

        let dropped_items = state.read().dropped_files.clone();

        ScrollView::new()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .padding(Gaps::new_all(24.0))
                    .child(
                        rect()
                            .width(Size::fill())
                            .child(DropZone {}),
                    )
                    .child(
                        rect()
                            .margin(Gaps::new(16.0, 0.0, 0.0, 0.0))
                            .width(Size::fill())
                            .direction(Direction::Vertical)
                            .spacing(8.0)
                            .children(dropped_items.into_iter().enumerate().map(|(i, f)| {
                                rect()
                                    .key(i)
                                    .padding(Gaps::new_all(12.0))
                                    .background(with_alpha(tokens.bg_card, 220))
                                    .corner_radius(6.0)
                                    .border(Border::new().width(1.0).fill(tokens.border))
                                    .child(
                                        label()
                                            .color(tokens.text_primary)
                                            .font_size(12.0)
                                            .text(format!("Dropped: {}", f)),
                                    )
                                    .into_element()
                            })),
                    ),
            )
    }
}
