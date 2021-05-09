// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Display;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>) @extends gio::AppLaunchContext;

    match fn {
        type_ => || ffi::gdk_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    #[doc(alias = "gdk_app_launch_context_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_app_launch_context_get_display(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_desktop")]
    pub fn set_desktop(&self, desktop: i32) {
        unsafe {
            ffi::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop);
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_icon")]
    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon(
                self.to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_icon_name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_app_launch_context_set_timestamp")]
    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp);
        }
    }
}

#[derive(Clone, Default)]
pub struct AppLaunchContextBuilder {
    display: Option<Display>,
}

impl AppLaunchContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> AppLaunchContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        glib::Object::new::<AppLaunchContext>(&properties)
            .expect("Failed to create an instance of AppLaunchContext")
    }

    pub fn display(mut self, display: &Display) -> Self {
        self.display = Some(display.clone());
        self
    }
}

impl fmt::Display for AppLaunchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppLaunchContext")
    }
}
