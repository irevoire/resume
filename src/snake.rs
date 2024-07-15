/* use snake::{Cli, World};
use window_rs::WindowBuffer;

use crate::{draw_window_buffer, InputWrapper};

pub struct Snake {
    buffer: WindowBuffer,
    config: World,
    cli: Cli,
}

impl Default for Snake {
    fn default() -> Self {
        let cli = Cli{
            width: todo!(),
            height: todo!(),
            snake_size_start: todo!(),
            file_path: todo!(),
            snake_speed: todo!(),
            speed_increase: todo!(),
            bad_berries: todo!(),
            ghost_mode: todo!(),
            two_players_mode: todo!(),
        }

    }
}

impl Snake {
    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Reset snake board:");
            if ui.add(egui::Button::new("Reset")).clicked() {
                *self = Snake::default();
            };

            ui.separator();

            ui.label("Snake starting size:");
            ui.add(egui::DragValue::new(&mut self.cli.ball_speed).speed(1).range(10));

            ui.separator();

            ui.label("Snake speed:");
            ui.add(egui::DragValue::new(&mut self.cli.pong_speed).speed(1));

            ui.separator();

            ui.label("Bad berries:");
            ui.add(egui::Checkbox::new(&mut my_bool, "Checked"));

            ui.separator();

            ui.label("Ghost mode:");
            ui.add(egui::Checkbox::new(&mut my_bool, "Checked"));

            ui.separator();

            ui.label("2 players:");
            ui.add(egui::Checkbox::new(&mut my_bool, "Checked"));

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
*/
