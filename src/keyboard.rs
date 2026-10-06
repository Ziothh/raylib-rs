use crate::ffi;

pub use ffi::enums::KeyboardKey;

impl KeyboardKey {
    fn to_c_int(self) -> std::os::raw::c_int {
        self as i32
    }

    fn from_c_int(keycode: std::os::raw::c_int) -> Self {
        unsafe { std::mem::transmute::<i32, ffi::enums::KeyboardKey>(keycode) }
    }

    pub fn is_down(&self) -> bool {
        unsafe { ffi::IsKeyDown(self.to_c_int()) }
    }

    pub fn is_up(&self) -> bool {
        unsafe { ffi::IsKeyUp(self.to_c_int()) }
    }

    pub fn is_pressed(&self) -> bool {
        unsafe { ffi::IsKeyPressed(self.to_c_int()) }
    }

    pub fn is_released(&self) -> bool {
        unsafe { ffi::IsKeyReleased(self.to_c_int()) }
    }

    pub fn is_pressed_repeat(&self) -> bool {
        unsafe { ffi::IsKeyPressedRepeat(self.to_c_int()) }
    }

    pub fn get_pressed() -> Self {
        unsafe { KeyboardKey::from_c_int(ffi::GetKeyPressed()) }
    }
}
