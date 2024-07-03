use crate::{
    cv::Cv,
};
use egui::Layout;
use serde::Deserialize;

#[derive(Default, Clone)]
pub struct Resume {
    viewing: View,
    cv: Cv,
    //snake: Snake,
    //pong: Pong,
    //maze: Maze,
    //life: Life,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum View {
    #[default]
    Resume,
    Snake,
    Pong,
    Maze,
    Life,
}

impl Resume {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Resume {
            viewing: View::default(),
            cv: Cv::default(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.selectable_value(&mut self.viewing, View::Resume, "Resume");
                ui.selectable_value(&mut self.viewing, View::Snake, "Play a game of Snake");
                ui.selectable_value(&mut self.viewing, View::Pong, "Pong");
                ui.selectable_value(&mut self.viewing, View::Maze, "Maze");
                ui.selectable_value(&mut self.viewing, View::Life, "Life");
            });
        });
        match self.viewing {
            View::Resume => Cv::cv(ctx),
            View::Snake => todo!(),
            View::Pong => todo!(),
            View::Maze => todo!(),
            View::Life => todo!(),
        }
    }
}

impl eframe::App for Resume {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui(ctx, frame);
    }
}
