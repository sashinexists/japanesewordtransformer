// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point;
use crate::Quad;
use crate::Rect;
use glib::translate::*;

impl Quad {
    #[doc(alias = "graphene_quad_init_from_points")]
    pub fn init_from_points(&mut self, points: &[&Point; 4]) {
        unsafe {
            let points = [
                *points[0].to_glib_none().0,
                *points[1].to_glib_none().0,
                *points[2].to_glib_none().0,
                *points[3].to_glib_none().0,
            ];
            ffi::graphene_quad_init_from_points(self.to_glib_none_mut().0, &points as *const _);
        }
    }

    #[doc(alias = "graphene_quad_init")]
    pub fn new(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Quad {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quad_alloc();
            ffi::graphene_quad_init(
                alloc,
                p1.to_glib_none().0,
                p2.to_glib_none().0,
                p3.to_glib_none().0,
                p4.to_glib_none().0,
            );
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quad_init_from_rect")]
    #[doc(alias = "new_from_rect")]
    pub fn from_rect(r: &Rect) -> Quad {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_quad_alloc();
            ffi::graphene_quad_init_from_rect(alloc, r.to_glib_none().0);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_quad_init_from_points")]
    #[doc(alias = "new_from_points")]
    pub fn from_points(points: &[&Point; 4]) -> Quad {
        assert_initialized_main_thread!();
        unsafe {
            let points = [
                *points[0].to_glib_none().0,
                *points[1].to_glib_none().0,
                *points[2].to_glib_none().0,
                *points[3].to_glib_none().0,
            ];
            let alloc = ffi::graphene_quad_alloc();
            ffi::graphene_quad_init_from_points(alloc, &points as *const _);
            from_glib_full(alloc)
        }
    }
}
