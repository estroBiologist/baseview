#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod win;
#[cfg(target_os = "linux")]
pub mod x11;

mod clipboard;
mod event;
mod keyboard;
mod mouse_cursor;
mod window;
mod window_info;
mod window_open_options;

#[cfg(feature = "opengl")]
pub mod gl;

pub use clipboard::*;
pub use event::*;
pub use mouse_cursor::MouseCursor;
pub use window::*;
pub use window_info::*;
pub use window_open_options::*;
