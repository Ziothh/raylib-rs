use crate::ffi;

pub use ffi::enums::KeyboardKey;

impl KeyboardKey {
    fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub fn is_down(&self) -> bool {
        unsafe { ffi::IsKeyDown(self.to_i32()) }
    }
}
