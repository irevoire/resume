use egui::{color_picker::{color_edit_button_rgba, Alpha}, Rgba, Ui};
use pong::{creation_pongs, display, Cli, Difficulty, World};
use rand::SeedableRng;
use window_rs::WindowBuffer;
use std::time::Instant;

use crate::{draw_window_buffer, InputWrapper};

pub struct Pong {
    buffer: WindowBuffer,
    config: World,
    instant_ball: Instant,
    instant_pong: Instant,
    cli: Cli,
}

impl Default for Pong {
    fn default() -> Self {
        let cli = Cli {
            ball_speed: 20,
            pong_speed: 60,
            difficulty: Difficulty::Medium,
            number_of_points_to_reach: 10,
        };
        let buffer: WindowBuffer = WindowBuffer::new(50, 33);
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
            0x00FF0000,
            0xFF00FF00,
            0xFFFFFF00,
        );
        
        Self {
            buffer,
            config,
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

            ui.separator();

            ui.label("Ball speed:");
            ui.add(egui::DragValue::new(&mut self.cli.ball_speed).speed(1));

            ui.separator();

            ui.label("Pong speed:");
            ui.add(egui::DragValue::new(&mut self.cli.pong_speed).speed(1));

            ui.separator();

            ui.label("Colour player 1:");
            let rgba_player: u32 = self.config.player_1_colour;
            let [r, g, b, a] = rgba_player.to_le_bytes();
            let mut colour_player = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour_player, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour_player);
            let player_color = u32::from_le_bytes(convert_color);
            self.config.player_1_colour = player_color;

            ui.separator();

            ui.label("Colour player 2:");
            let rgba_player: u32 = self.config.player_2_colour;
            let [r, g, b, a] = rgba_player.to_le_bytes();
            let mut colour_player = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour_player, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour_player);
            let player_color = u32::from_le_bytes(convert_color);
            self.config.player_2_colour = player_color;

            ui.separator();

            ui.label("Colour ball:");
            let rgba_ball: u32 = self.config.ball_colour;
            let [r, g, b, a] = rgba_ball.to_le_bytes();
            let mut colour_ball = Rgba::from_srgba_premultiplied(r, g, b, a);
            color_edit_button_rgba(ui, &mut colour_ball, Alpha::Opaque);
            let convert_color = Rgba::to_srgba_unmultiplied(&colour_ball);
            let ball_color = u32::from_le_bytes(convert_color);
            self.config.ball_colour = ball_color;

            ui.separator();

        })
        .response
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.config.player_1_pong.is_empty() == true {
            creation_pongs(&mut self.config, &self.buffer)
        } 
        
        ctx.input(|i| {
            let _ = self
                .config
                .handle_user_input(&InputWrapper { input: i }, &self.buffer);
        });

        self.config.update(&mut self.buffer, &self.cli, &mut self.instant_pong, &mut self.instant_ball);
        display(&self.config, &mut self.buffer);
        ctx.request_repaint();

        egui::SidePanel::right("Configuration").show(ctx, |ui| self.configuration(ui));
        
        egui::CentralPanel::default().show(ctx, |ui| draw_window_buffer(ui, &self.buffer));
    }

}