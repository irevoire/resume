use egui::Ui;
use maze::start_end_generator;
use maze::{display, MazeConfig, Player};
use rand::SeedableRng;
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;

use crate::{common::colour_changer, draw_window_buffer, InputWrapper};

pub struct Maze {
    buffer: WindowBuffer,
    seed: u64,
    pub config: MazeConfig,
    player: Player,
    start_point: (usize, usize),
    update_time_wait: Instant,
    opened_walls: usize,
}

impl Default for Maze {
    fn default() -> Self {
        let mut buffer: WindowBuffer = WindowBuffer::new(30, 30);

        let mut buff = [0; 8];
        getrandom::getrandom(&mut buff).unwrap();
        let seed: u64 = u64::from_be_bytes(buff);
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
            opened_walls: 0,
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
            let rgba_wall: u32 = self.config.wall_color;
            let wall_color = colour_changer(rgba_wall, ui);
            self.config.wall_color = wall_color;

            ui.separator();

            ui.label("Path:");
            let rgba_path = self.config.path_color;
            let mut path_color = colour_changer(rgba_path, ui);
            if path_color == wall_color && wall_color != u32::MAX {
                path_color += 1;
            } else if path_color == wall_color && wall_color == u32::MAX {
                path_color -= 1;
            }
            self.config.path_color = path_color;

            ui.separator();

            ui.label("Colour player:");
            let rgba_player: u32 = self.player.player_color;
            let player_color = colour_changer(rgba_player, ui);
            self.player.player_color = player_color;

            ui.separator();

            ui.label("Colour ending:");
            let rgba_ending: u32 = self.player.finish_color;
            let mut ending_color = colour_changer(rgba_ending, ui);
            if ending_color == wall_color && wall_color != u32::MAX {
                ending_color += 1;
            } else if ending_color == wall_color && wall_color == u32::MAX {
                ending_color -= 1;
            }
            self.player.finish_color = ending_color;

            ui.separator();

            ui.label("Difficulty:");
            ui.add(egui::DragValue::new(&mut self.opened_walls).speed(1));

            let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
            self.config.generate(&mut self.buffer, &mut rng);
            self.player.maze_config = self.config.clone();
            display(&self.player, &mut self.buffer);
        })
        .response
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let elapsed_time = Duration::from_millis(10_u64);
        ctx.input(|i| {
            let _ = self
                .player
                .handle_user_input(&InputWrapper { input: i }, &self.start_point);
        });

        if self.update_time_wait.elapsed() >= elapsed_time {
            display(&self.player, &mut self.buffer);
            self.player.direction(&self.buffer);
            self.update_time_wait = Instant::now();
        }
        egui::SidePanel::right("Configuration").show(ctx, |ui| self.configuration(ui));

        egui::CentralPanel::default().show(ctx, |ui| draw_window_buffer(ui, &self.buffer));
    }
}
