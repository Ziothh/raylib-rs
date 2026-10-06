use crate::ffi;

pub use crate::ffi::{RenderTexture, Texture};

impl RenderTexture {
    /// Load texture for rendering (framebuffer) into the GPU
    pub fn load(width: i32, height: i32) -> Self {
        unsafe { ffi::LoadRenderTexture(width, height) }
    }

    /// Unload render texture from GPU memory (VRAM)
    pub fn unload(self) {
        unsafe {
            ffi::UnloadRenderTexture(self);
        }
    }

    pub fn draw(&mut self, cb: fn()) -> &mut Self {
        unsafe {
            ffi::BeginTextureMode(*self);
        };
        cb();

        unsafe {
            ffi::EndTextureMode();
        }

        self
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        self.unload();
    }
}
