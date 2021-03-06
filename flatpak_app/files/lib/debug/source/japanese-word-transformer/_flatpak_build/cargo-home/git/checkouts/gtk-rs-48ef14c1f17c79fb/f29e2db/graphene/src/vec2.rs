// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Vec2;
use glib::translate::*;

impl Vec2 {
    #[doc(alias = "graphene_vec2_init_from_float")]
    pub fn init_from_float(&mut self, src: &[f32; 2]) {
        unsafe {
            ffi::graphene_vec2_init_from_float(self.to_glib_none_mut().0, src as *const _);
        }
    }

    #[doc(alias = "graphene_vec2_init")]
    pub fn new(x: f32, y: f32) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec2_alloc();
            ffi::graphene_vec2_init(alloc, x, y);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec2_init_from_vec2")]
    #[doc(alias = "new_from_vec2")]
    pub fn from_vec2(src: &Vec2) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec2_alloc();
            ffi::graphene_vec2_init_from_vec2(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec2_init_from_float")]
    #[doc(alias = "new_from_float")]
    pub fn from_float(src: &[f32; 2]) -> Vec2 {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_vec2_alloc();
            ffi::graphene_vec2_init_from_float(alloc, src as *const _);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_vec2_to_float")]
    pub fn to_float(&self) -> [f32; 2] {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            ffi::graphene_vec2_to_float(self.to_glib_none().0, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
