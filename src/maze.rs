use egui::{Color32, InputState, Key, Pos2, Rounding, Sense, Ui, Vec2};
use maze::{display, MazeConfig, Player};
use rand::rngs::ThreadRng;
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;
use maze::start_end_generator;
use graphic::Graphic;

use crate::{draw_window_buffer, InputWrapper};

pub struct Maze {
    buffer: WindowBuffer,
    rng: ThreadRng,
    config: MazeConfig,
    player: Player,
    start_point: (usize, usize),
    update_time_wait: Instant,
}

impl Default for Maze {
    fn default() -> Self {
        let mut buffer: WindowBuffer = WindowBuffer::new(30, 30);

        let mut rng = rand::thread_rng();
        let config = MazeConfig::default();
        config.generate(&mut buffer, &mut rng);
        let mut player = Player::new(
            (0, 0),
            (0, 0),
            maze::Direction::Still,
            (0, 0),
            config.clone(),
            false,
        );

        let start_point = start_end_generator(&mut buffer, &mut rng, &mut player);
        Self {
            rng,
            config,
            player,
            start_point,
            buffer,
            update_time_wait: Instant::now(),
        }
    }
}


impl Maze {
    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Reset maze:");
            if ui.add(egui::Button::new("Reset")).clicked() {
                *self = Maze::default();
            };
        })
        .response
    }
    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        let elapsed_time = Duration::from_millis(10 as u64);
        ctx.input(|i| {
            let _ = self.player.handle_user_input(&InputWrapper{input: i}, &self.start_point);

        });

        if self.update_time_wait.elapsed() >= elapsed_time {
            display(&mut self.player, &mut self.buffer);
            self.player.direction(&self.buffer);
            self.update_time_wait = Instant::now();
        }
        egui::SidePanel::right("Configuration").show(ctx, |ui| self.configuration(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            draw_window_buffer(ui, &self.buffer)
        });
    }
}
