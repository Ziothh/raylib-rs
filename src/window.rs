use crate::{ffi, Vector2};

pub use ffi::Camera2D;

impl Camera2D {
    /// Creates a new `Camera2D` as sets it at coords `(0, 0)`
    pub const fn new_zeroed(zoom: f32) -> Self {
        return Self {
            offset: Vector2::ZERO,
            target: Vector2::ZERO,
            rotation: 0.,
            zoom,
        };
    }

    // /// Creates a new `Camera2D` that's centered to the provided window dimsensions
    // pub fn new_centered(width: i32, height: i32, zoom: f32) -> Self {
    //     return Self { offset };
    // }
}
