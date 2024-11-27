mod frame_history;
mod shape;
mod speedometer;

use crate::frame_history::FramesHistory;
use crate::speedometer::SpeedometerUi;

use eframe::egui::{self, Theme};
use std::time::{Duration, Instant};

const MAX_SPEED: u32 = 200;
const MIN_SPEED: u32 = 0;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Egui Dashboard",
        options,
        Box::new(|cc| Ok(Box::new(Dashboard::new(cc)))),
    )
}

struct Dashboard {
    frame_history: FramesHistory,
    speed: u32,
    last_update: Instant,
}

impl Dashboard {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(Theme::Dark);

        Self {
            frame_history: FramesHistory::new(),
            speed: 0,
            last_update: Instant::now(),
        }
    }
}

impl eframe::App for Dashboard {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        let mean_frame_time_string = format!(
            "Mean CPU usage: {:.2} ms / frame",
            1e3 * self.frame_history.mean_frame_time()
        );

        let fps_string = format!("FPS: {:.0}", self.frame_history.fps());

        if self.last_update.elapsed() > Duration::from_secs(1) {
            self.speed += 1;
            self.last_update = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(mean_frame_time_string);
            ui.label(fps_string);
            ui.speedometer(self.speed, MIN_SPEED, MAX_SPEED);
        });

        ctx.request_repaint(); // Нужен только для правильного подсчёта fps.
    }
}
