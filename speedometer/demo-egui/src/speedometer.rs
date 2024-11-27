use crate::shape::rotate::Rotate;
use eframe::egui::{
    self,
    epaint::{CircleShape, Shape},
    Align2, Color32, FontFamily, FontId, Pos2, Response, Stroke, Ui, Vec2, Widget,
};
use std::f32::consts::PI;
use std::ops::Not;

const BACKGROUND_COLOR: Color32 = Color32::from_rgb(0x20u8, 0x22u8, 0x25u8);
const BACKGROUND_BORDER_COLOR: Color32 = Color32::from_rgb(0x44u8, 0x44u8, 0x46u8);
const TICK_COLOR: Color32 = Color32::from_rgb(0xFF, 0x45, 0x3A);
const SECTOR_DEGREES: f32 = 100.0;

pub trait SpeedometerUi {
    fn speedometer(&mut self, speed: u32, min_speed: u32, max_speed: u32) -> Response;
}

impl SpeedometerUi for Ui {
    fn speedometer(&mut self, speed: u32, min_speed: u32, max_speed: u32) -> Response {
        Speedometer {
            speed,
            min_speed,
            max_speed,
        }
        .ui(self)
    }
}

struct Speedometer {
    speed: u32,
    min_speed: u32,
    max_speed: u32,
}

impl Widget for Speedometer {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::focusable_noninteractive());

        if ui.is_rect_visible(rect).not() {
            return response;
        };

        let radius = rect.width().min(rect.height()) / 2.0;
        let center = rect.center();

        draw_background(ui, center, radius);
        self.draw_ticks(ui, center, radius, radius);
        self.draw_needle(ui, center, radius);

        response
    }
}

impl Speedometer {
    fn draw_needle(&self, ui: &mut Ui, center: Pos2, scale_by_radius: f32) {
        let mut line = Shape::line_segment(
            [
                Pos2::new(0.0, 0.12 * scale_by_radius),
                Pos2::new(0.0, -0.88 * scale_by_radius),
            ],
            Stroke::new(scale_by_radius / 60.0, TICK_COLOR),
        );

        let angle = needle_rotation(self.speed, self.max_speed);

        line.rotate(angle);
        line.translate(Vec2::new(center.x, center.y));

        ui.painter().add(line);
    }

    fn draw_ticks(&self, ui: &mut Ui, center: Pos2, scale_by_radius: f32, radius: f32) {
        for tick in (self.min_speed..=self.max_speed).step_by(10) {
            if tick % 20 == 0 {
                let mut line = Shape::line_segment(
                    [
                        Pos2::new(0.0, 16.0 - scale_by_radius),
                        Pos2::new(0.0, -0.85 * scale_by_radius),
                    ],
                    Stroke::new(scale_by_radius / 50.0, Color32::WHITE),
                );
                let angle = needle_rotation(tick, self.max_speed);
                line.rotate(angle);
                line.translate(Vec2::new(center.x, center.y));

                ui.painter().add(line);

                self.draw_tick_label(ui, tick, center, scale_by_radius, radius);
            } else {
                let mut line = Shape::line_segment(
                    [
                        Pos2::new(0.0, 16.0 - scale_by_radius),
                        Pos2::new(0.0, -0.85 * scale_by_radius),
                    ],
                    Stroke::new(scale_by_radius / 100.0, Color32::WHITE),
                );
                let angle = needle_rotation(tick, self.max_speed);
                line.rotate(angle);
                line.translate(Vec2::new(center.x, center.y));

                ui.painter().add(line);
            }
        }
    }

    fn draw_tick_label(
        &self,
        ui: &mut Ui,
        tick: u32,
        center: Pos2,
        scale_by_radius: f32,
        radius: f32,
    ) {
        let angle = tick_label_rotation(tick, self.max_speed);
        let scale = scale_by_radius / 347.0;

        ui.painter().text(
            Pos2::new(
                center.x + 0.76 * radius * angle.cos(),
                center.y + 0.76 * radius * angle.sin(),
            ),
            Align2::CENTER_CENTER,
            format!("{tick:<3}"),
            FontId::new(24.0 * scale, FontFamily::default()),
            Color32::WHITE,
        );
    }
}

fn draw_background(ui: &mut Ui, center: Pos2, radius: f32) {
    ui.painter().add(CircleShape {
        center,
        radius,
        fill: BACKGROUND_COLOR,
        stroke: Stroke::new(9.0, BACKGROUND_BORDER_COLOR),
    });
}

fn needle_rotation(n: u32, total: u32) -> f32 {
    let turns = n as f32 / total as f32;

    let degrees = (360.0 - SECTOR_DEGREES) * turns - (180.0 - SECTOR_DEGREES / 2.0);
    degrees * PI / 180.0
}

fn tick_label_rotation(n: u32, total: u32) -> f32 {
    let turns = n as f32 / total as f32;

    let degrees = (360.0 - SECTOR_DEGREES) * turns - 220.0;
    degrees * PI / 180.0
}
