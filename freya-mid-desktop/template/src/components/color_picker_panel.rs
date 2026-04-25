use freya::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
enum DragTarget {
    #[default]
    None,
    Sv,
    Hue,
}

#[derive(Clone, PartialEq)]
pub struct ColorPickerPanel {
    value: Color,
    on_change: EventHandler<Color>,
    width: Size,
}

impl ColorPickerPanel {
    pub fn new(on_change: impl Into<EventHandler<Color>>) -> Self {
        Self {
            value: Color::WHITE,
            on_change: on_change.into(),
            width: Size::px(240.0),
        }
    }

    pub fn value(mut self, value: Color) -> Self {
        self.value = value;
        self
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }
}

impl Component for ColorPickerPanel {
    fn render(&self) -> impl IntoElement {
        let mut color = use_state(|| self.value);
        let mut dragging = use_state(DragTarget::default);
        let mut sv_area = use_state(Area::default);
        let mut hue_area = use_state(Area::default);

        const MIN_S: f32 = 0.07;
        const MIN_V: f32 = 0.07;

        let mut update_sv = {
            let on_change = self.on_change.clone();
            move |coords: CursorPoint| {
                let a = sv_area.read().to_f64();
                let rel_x =
                    (((coords.x - a.min_x()) / a.width()).clamp(0., 1.)) as f32;
                let rel_y = (((coords.y - a.min_y()) / a.height())
                    .clamp(MIN_V as f64, 1. - MIN_V as f64)) as f32;
                let sat = rel_x.max(MIN_S);
                let v = (1.0 - rel_y).clamp(MIN_V, 1.0 - MIN_V);
                let hsv = color.read().to_hsv();
                color.set(Color::from_hsv(hsv.h, sat, v));
                on_change.call(color());
            }
        };

        let mut update_hue = {
            let on_change = self.on_change.clone();
            move |coords: CursorPoint| {
                let a = hue_area.read().to_f64();
                let rel_x =
                    ((coords.x - a.min_x()) / a.width()).clamp(0.01, 1.) as f32;
                let hsv = color.read().to_hsv();
                color.set(Color::from_hsv(rel_x * 360.0, hsv.s, hsv.v));
                on_change.call(color());
            }
        };

        let on_sv_down = {
            let mut update_sv = update_sv.clone();
            move |e: Event<PointerEventData>| {
                dragging.set(DragTarget::Sv);
                update_sv(e.global_location());
                e.stop_propagation();
                e.prevent_default();
            }
        };

        let on_hue_down = {
            let mut update_hue = update_hue.clone();
            move |e: Event<PointerEventData>| {
                dragging.set(DragTarget::Hue);
                update_hue(e.global_location());
                e.stop_propagation();
                e.prevent_default();
            }
        };

        let on_move = move |e: Event<PointerEventData>| match *dragging.read() {
            DragTarget::Sv => update_sv(e.global_location()),
            DragTarget::Hue => update_hue(e.global_location()),
            DragTarget::None => {}
        };

        let on_global_press = move |_| {
            dragging.set_if_modified(DragTarget::None);
        };

        let hue_deg = color.read().to_hsv().h;
        let hex = format!(
            "#{:02X}{:02X}{:02X}",
            color.read().r(),
            color.read().g(),
            color.read().b()
        );

        rect()
            .on_global_pointer_move(on_move)
            .on_global_pointer_press(on_global_press)
            .width(self.width.clone())
            .spacing(8.0)
            .child(
                rect()
                    .on_sized(move |e: Event<SizedEventData>| sv_area.set(e.area))
                    .on_pointer_down(on_sv_down)
                    .child(
                        rect()
                            .height(Size::px(140.0))
                            .width(Size::fill())
                            .corner_radius(4.0)
                            .overflow(Overflow::Clip)
                            .child(
                                rect()
                                    .expanded()
                                    .background_linear_gradient(
                                        LinearGradient::new()
                                            .angle(-90.0)
                                            .stop(((255, 255, 255), 0.0))
                                            .stop((Color::from_hsv(hue_deg, 1.0, 1.0), 100.0)),
                                    )
                                    .child(
                                        rect()
                                            .position(Position::new_absolute())
                                            .expanded()
                                            .background_linear_gradient(
                                                LinearGradient::new()
                                                    .stop(((255, 255, 255, 0.0_f32), 0.0))
                                                    .stop(((0, 0, 0), 100.0)),
                                            ),
                                    ),
                            ),
                    ),
            )
            .child(
                rect()
                    .height(Size::px(18.0))
                    .on_sized(move |e: Event<SizedEventData>| hue_area.set(e.area))
                    .on_pointer_down(on_hue_down)
                    .child(
                        rect()
                            .height(Size::px(18.0))
                            .width(Size::fill())
                            .corner_radius(4.0)
                            .background_linear_gradient(
                                LinearGradient::new()
                                    .angle(-90.0)
                                    .stop(((255, 0, 0), 0.0))
                                    .stop(((255, 255, 0), 16.0))
                                    .stop(((0, 255, 0), 33.0))
                                    .stop(((0, 255, 255), 50.0))
                                    .stop(((0, 0, 255), 66.0))
                                    .stop(((255, 0, 255), 83.0))
                                    .stop(((255, 0, 0), 100.0)),
                            ),
                    ),
            )
            .child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .main_align(Alignment::Center)
                    .child(label().font_size(12.0).color((161u8, 161u8, 170u8)).text(hex)),
            )
    }
}
