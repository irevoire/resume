#![warn(clippy::all, rust_2018_idioms)]

mod cv;
mod life;
mod maze;
mod pong;
mod resume;
mod snake;
mod common;
pub use resume::Resume;
pub use common::{InputWrapper, draw_window_buffer};
