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
    pub config: MazeConfig,
    player: Player,
    start_point: (usize, usize),
    update_time_wait: Instant,
    opened_walls: usize,
}

impl Default for Maze {
    fn default() -> Self {
        let mut buffer: WindowBuffer = WindowBuffer::new(30, 30);
        
        let mut buff = [0;8];
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
            let rgba_wall:u32 = self.config.wall_color;
            let [r, g, b, a] = rgba_wall.to_le_bytes();
            let mut colour_wall = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour_wall, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour_wall);
            let wall_color = u32::from_le_bytes(convert_color);
            self.config.wall_color = wall_color;

            ui.separator();

            ui.label("Path:");
            let rgba_path = self.config.path_color;
            let [r, g, b, a] = rgba_path.to_le_bytes();
            let mut colour_path = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour_path, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour_path);
            let mut path_color = u32::from_le_bytes(convert_color);
            if path_color == wall_color && wall_color != u32::MAX {
                path_color = path_color + 1;
            } else if path_color == wall_color && wall_color == u32::MAX {
                path_color = path_color - 1;
            }
            self.config.path_color = path_color;

            ui.separator();

            ui.label("Colour player:");
            let rgba_player:u32 = self.player.player_color;
            let [r, g, b, a] = rgba_player.to_le_bytes();
            let mut colour_player = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour_player, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour_player);
            let player_color = u32::from_le_bytes(convert_color);
            self.player.player_color = player_color;

            ui.separator();

            ui.label("Colour ending:");
            let rgba_ending:u32 = self.player.finish_color;
            let [r, g, b, a] = rgba_ending.to_le_bytes();
            let mut colour_ending = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour_ending, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour_ending);
            let ending_color = u32::from_le_bytes(convert_color);
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
