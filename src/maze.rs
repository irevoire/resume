use egui::{color_picker::{color_edit_button_rgba, Alpha}, Rgba, Ui};
use maze::{display, MazeConfig, Player};
use rand::{Rng, SeedableRng};
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;
use maze::start_end_generator;

use crate::{draw_window_buffer, InputWrapper};

pub struct Maze {
    buffer: WindowBuffer,
    seed: u64,
    config: MazeConfig,
    player: Player,
    start_point: (usize, usize),
    update_time_wait: Instant,
}

impl Default for Maze {
    fn default() -> Self {
        let mut buffer: WindowBuffer = WindowBuffer::new(30, 30);

        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
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
            seed,
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

            ui.label("Wall:");
            let rgba:u32 = self.config.wall_color;
            let [r, g, b, a] = rgba.to_ne_bytes();
            let mut colour = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour);
            let wall_color = u32::from_ne_bytes(convert_color);
            self.config.wall_color = wall_color;

            ui.label("Path:");
            let rgba = self.config.path_color;
            let [r, g, b, a] = rgba.to_ne_bytes();
            let mut colour = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour);
            let path_color = u32::from_ne_bytes(convert_color);
            self.config.path_color = path_color;

            let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
            self.config.generate(&mut self.buffer, &mut rng);
            self.player.maze_config = self.config.clone();
            display(&self.player, &mut self.buffer);

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
