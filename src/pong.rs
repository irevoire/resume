use egui::Ui;
use pong::{creation_pongs, display, Cli, Difficulty, World};
use rand::SeedableRng;
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;

use crate::{draw_window_buffer, InputWrapper};

pub struct Pong {
    buffer: WindowBuffer,
    config: World,
    seed: u64,
    update_time_wait: Instant,
    instant_ball: Instant,
    instant_pong: Instant,
    cli: Cli,
}

impl Default for Pong {
    fn default() -> Self {
        let cli = Cli {
            ball_speed: 20,
            pong_speed: 0,
            difficulty: Difficulty::Medium,
            number_of_points_to_reach: 10,
        };
        let buffer: WindowBuffer = WindowBuffer::new(40, 40);
        let mut buff = [0; 8];
        getrandom::getrandom(&mut buff).unwrap();
        let seed: u64 = u64::from_be_bytes(buff);
        let config = World::new(
            Vec::new(),
            Vec::new(),
            0,
            0,
            pong::Direction::Still,
            pong::Direction::Still,
            Some((buffer.width() / 2, buffer.height() / 2)),
            pong::BallDirection::Still,
            false,
            Instant::now(),
            0,
            cli.pong_speed,
            cli.ball_speed,
            rand::rngs::StdRng::seed_from_u64(seed),
        );
        
        Self {
            buffer,
            config,
            seed,
            update_time_wait: Instant::now(),
            instant_ball: Instant::now(),
            instant_pong: Instant::now(),
            cli,
        }
    }
}

impl Pong {
    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Reset pong board:");
            if ui.add(egui::Button::new("Reset")).clicked() {
                *self = Pong::default();
            };

            ui.label("Ball speed:");
            ui.add(egui::DragValue::new(&mut self.cli.ball_speed).speed(1));

            ui.label("Pong speed:");
            ui.add(egui::DragValue::new(&mut self.cli.pong_speed).speed(1));

            ui.label("Numbers of point to reach");
            ui.add(egui::DragValue::new(&mut self.cli.number_of_points_to_reach).speed(1));

        })
        .response
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            let _ = self
                .config
                .handle_user_input(&InputWrapper { input: i }, &self.buffer);
        });

        creation_pongs(&mut self.config, &self.buffer);

        self.config.update(&mut self.buffer, &self.cli, &mut self.instant_pong, &mut self.instant_ball);
        display(&self.config, &mut self.buffer);

        egui::SidePanel::right("Configuration").show(ctx, |ui| self.configuration(ui));

        egui::CentralPanel::default().show(ctx, |ui| draw_window_buffer(ui, &self.buffer));
    }

}