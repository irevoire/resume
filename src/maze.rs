use egui::{Color32, InputState, Key, Pos2, Rounding, Sense, Vec2};
use maze::{display, MazeConfig, Player};
use rand::rngs::ThreadRng;
use web_time::{Duration, Instant};
use window_rs::WindowBuffer;
use maze::start_end_generator;
use graphic::Graphic;

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

struct InputWrapper<'a> {
    input: &'a InputState
}

impl Graphic for InputWrapper<'_> {
    fn new(name: &str, width: usize, height: usize) -> Self {
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

    fn update_with_buffer(&mut self, windows: &WindowBuffer) {
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
        todo!()
    }

    fn get_mouse_pos(&self, mouse: graphic::Mouse) -> Option<(f32, f32)> {
        todo!()
    }

    fn get_mouse_down(&self, mouse: graphic::Mouse) -> bool {
        todo!()
    }
}

impl Maze {
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
        egui::CentralPanel::default().show(ctx, |ui| {
            Self::draw_window_buffer(ui, &self.buffer)
        });
        
        
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
}
