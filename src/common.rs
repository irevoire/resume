use egui::{Color32, InputState, Key, Pos2, Rounding, Sense, Vec2};
use graphic::Graphic;
use window_rs::WindowBuffer;

pub struct InputWrapper<'a> {
    pub input: &'a InputState,
}

impl Graphic for InputWrapper<'_> {
    fn new(_name: &str, _width: usize, _height: usize) -> Self {
        todo!()
    }

    fn is_open(&self) -> bool {
        true
    }

    fn is_key_down(&self, key: graphic::Key) -> bool {
        match key {
            graphic::Key::Up => self.input.key_down(Key::ArrowUp),
            graphic::Key::Down => self.input.key_down(Key::ArrowDown),
            graphic::Key::Left => self.input.key_down(Key::ArrowLeft),
            graphic::Key::Right => self.input.key_down(Key::ArrowRight),
            graphic::Key::Escape => self.input.key_down(Key::Escape),
            graphic::Key::Quit => self.input.key_down(Key::Delete),
            graphic::Key::Space => self.input.key_down(Key::Space),
            graphic::Key::UpPlayer1 => self.input.key_down(Key::E),
            graphic::Key::DownPlayer1 => self.input.key_down(Key::D),
            graphic::Key::UpPlayer2 => self.input.key_down(Key::O),
            graphic::Key::DownPlayer2 => self.input.key_down(Key::L),
            graphic::Key::LeftPlayer2 => self.input.key_down(Key::K),
            graphic::Key::RightPlayer2 => self.input.key_down(Key::M),
            graphic::Key::Launch => self.input.key_down(Key::W),
            graphic::Key::Forward => self.input.key_down(Key::F),
            graphic::Key::Backward => self.input.key_down(Key::R),
            graphic::Key::Save => self.input.key_down(Key::S),
        }
    }

    fn update_with_buffer(&mut self, _windows: &WindowBuffer) {
        todo!()
    }

    fn is_key_pressed(&self, key: graphic::Key) -> bool {
        match key {
            graphic::Key::Up => self.input.key_pressed(Key::ArrowUp),
            graphic::Key::Down => self.input.key_pressed(Key::ArrowDown),
            graphic::Key::Left => self.input.key_pressed(Key::ArrowLeft),
            graphic::Key::Right => self.input.key_pressed(Key::ArrowRight),
            graphic::Key::Escape => self.input.key_pressed(Key::Escape),
            graphic::Key::Quit => self.input.key_pressed(Key::Delete),
            graphic::Key::Space => self.input.key_pressed(Key::Space),
            graphic::Key::UpPlayer1 => self.input.key_pressed(Key::E),
            graphic::Key::DownPlayer1 => self.input.key_pressed(Key::D),
            graphic::Key::UpPlayer2 => self.input.key_pressed(Key::O),
            graphic::Key::DownPlayer2 => self.input.key_pressed(Key::L),
            graphic::Key::LeftPlayer2 => self.input.key_pressed(Key::K),
            graphic::Key::RightPlayer2 => self.input.key_pressed(Key::M),
            graphic::Key::Launch => self.input.key_pressed(Key::W),
            graphic::Key::Forward => self.input.key_pressed(Key::F),
            graphic::Key::Backward => self.input.key_pressed(Key::R),
            graphic::Key::Save => self.input.key_pressed(Key::S),
        }
    }

    fn get_keys_released(&self) -> Vec<graphic::Key> {
        let mut ret = vec![];
        if self.input.key_released(Key::ArrowUp) == true {
            ret.push(graphic::Key::Up)
        }
        if self.input.key_released(Key::ArrowDown) == true {
            ret.push(graphic::Key::Down)
        }
        if self.input.key_released(Key::ArrowLeft) == true {
            ret.push(graphic::Key::Left)
        }
        if self.input.key_released(Key::ArrowRight) == true {
            ret.push(graphic::Key::Right)
        }
        if self.input.key_released(Key::Escape) == true {
            ret.push(graphic::Key::Escape)
        }
        if self.input.key_released(Key::Delete) == true {
            ret.push(graphic::Key::Quit)
        }
        if self.input.key_released(Key::Space) == true {
            ret.push(graphic::Key::Space)
        }
        if self.input.key_released(Key::E) == true {
            ret.push(graphic::Key::UpPlayer1)
        }
        if self.input.key_released(Key::D) == true {
            ret.push(graphic::Key::DownPlayer1)
        }
        if self.input.key_released(Key::O) == true {
            ret.push(graphic::Key::UpPlayer2)
        }
        if self.input.key_released(Key::L) == true {
            ret.push(graphic::Key::DownPlayer2)
        }
        if self.input.key_released(Key::K) == true {
            ret.push(graphic::Key::LeftPlayer2)
        }
        if self.input.key_released(Key::M) == true {
            ret.push(graphic::Key::RightPlayer2)
        }
        if self.input.key_released(Key::W) == true {
            ret.push(graphic::Key::Launch)
        }
        if self.input.key_released(Key::F) == true {
            ret.push(graphic::Key::Forward)
        }
        if self.input.key_released(Key::R) == true {
            ret.push(graphic::Key::Backward)
        }
        if self.input.key_released(Key::S) == true {
            ret.push(graphic::Key::Save)
        }
        ret
    }

    fn get_mouse_pos(&self, mouse: graphic::Mouse) -> Option<(f32, f32)> {
        let ret = self.input.pointer.interact_pos()?;
        let retured_pos = (ret[0], ret[1]);
        let clicked = match mouse {
            graphic::Mouse::Left => self.input.pointer.primary_clicked(),
            graphic::Mouse::Right => self.input.pointer.secondary_clicked(),
            graphic::Mouse::Discard => todo!(),
        };
        if clicked {
            Some(retured_pos)
        } else {
            None
        }
    }

    fn get_mouse_down(&self, mouse: graphic::Mouse) -> bool {
        match mouse {
            graphic::Mouse::Left => self.input.pointer.primary_down(),
            graphic::Mouse::Right => self.input.pointer.secondary_clicked(),
            graphic::Mouse::Discard => todo!(),
        }
    }
}

pub fn draw_window_buffer(ui: &mut egui::Ui, window: &WindowBuffer) {
    let size = 30;
    egui::ScrollArea::both().show(ui, |ui| {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(
                (size * window.width()) as f32,
                (size * window.height()) as f32,
            ),
            Sense::hover(),
        );

        let base_position = response.rect.left_top().to_vec2();
        for x in 0..window.width() {
            for y in 0..window.height() {
                let color = window[(x, y)];

                let rect = egui::Rect {
                    min: Pos2 {
                        x: x as f32 * size as f32,
                        y: y as f32 * size as f32,
                    },
                    max: Pos2 {
                        x: (x + 1) as f32 * size as f32,
                        y: (y + 1) as f32 * size as f32,
                    },
                };

                let rect = rect.translate(base_position);

                let [r, g, b, a] = color.to_ne_bytes();
                painter.rect_filled(
                    rect,
                    Rounding::ZERO,
                    Color32::from_rgba_premultiplied(r, g, b, a),
                );
            }
        }
    });
}
