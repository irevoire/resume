#![warn(clippy::all, rust_2018_idioms)]

mod common;
mod cv;
mod life;
mod maze;
mod pong;
mod resume;
mod snake;
pub use common::{draw_window_buffer, InputWrapper};
pub use resume::Resume;
