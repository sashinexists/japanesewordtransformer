// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::Button;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Orientable;
use crate::Orientation;
use crate::Overflow;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ScaleButton(Object<ffi::GtkScaleButton, ffi::GtkScaleButtonClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_scale_button_get_type(),
    }
}

impl ScaleButton {
    #[doc(alias = "gtk_scale_button_new")]
    pub fn new(min: f64, max: f64, step: f64, icons: &[&str]) -> ScaleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_button_new(
                min,
                max,
                step,
                icons.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct ScaleButtonBuilder {
    adjustment: Option<Adjustment>,
    icons: Option<Vec<String>>,
    value: Option<f64>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    orientation: Option<Orientation>,
}

impl ScaleButtonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ScaleButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref adjustment) = self.adjustment {
            properties.push(("adjustment", adjustment));
        }
        if let Some(ref icons) = self.icons {
            properties.push(("icons", icons));
        }
        if let Some(ref value) = self.value {
            properties.push(("value", value));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<ScaleButton>(&properties)
            .expect("Failed to create an instance of ScaleButton")
    }

    pub fn adjustment<P: IsA<Adjustment>>(mut self, adjustment: &P) -> Self {
        self.adjustment = Some(adjustment.clone().upcast());
        self
    }

    pub fn icons(mut self, icons: Vec<String>) -> Self {
        self.icons = Some(icons);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_SCALE_BUTTON: Option<&ScaleButton> = None;

pub trait ScaleButtonExt: 'static {
    #[doc(alias = "gtk_scale_button_get_adjustment")]
    #[doc(alias = "get_adjustment")]
    fn adjustment(&self) -> Adjustment;

    #[doc(alias = "gtk_scale_button_get_minus_button")]
    #[doc(alias = "get_minus_button")]
    fn minus_button(&self) -> Button;

    #[doc(alias = "gtk_scale_button_get_plus_button")]
    #[doc(alias = "get_plus_button")]
    fn plus_button(&self) -> Button;

    #[doc(alias = "gtk_scale_button_get_popup")]
    #[doc(alias = "get_popup")]
    fn popup(&self) -> Widget;

    #[doc(alias = "gtk_scale_button_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> f64;

    #[doc(alias = "gtk_scale_button_set_adjustment")]
    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P);

    #[doc(alias = "gtk_scale_button_set_icons")]
    fn set_icons(&self, icons: &[&str]);

    #[doc(alias = "gtk_scale_button_set_value")]
    fn set_value(&self, value: f64);

    fn icons(&self) -> Vec<glib::GString>;

    #[doc(alias = "popdown")]
    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popdown(&self);

    #[doc(alias = "popup")]
    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_popup(&self);

    #[doc(alias = "value-changed")]
    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "adjustment")]
    fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icons")]
    fn connect_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ScaleButton>> ScaleButtonExt for O {
    fn adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_adjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn minus_button(&self) -> Button {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_minus_button(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn plus_button(&self) -> Button {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_plus_button(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn popup(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_popup(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn value(&self) -> f64 {
        unsafe { ffi::gtk_scale_button_get_value(self.as_ref().to_glib_none().0) }
    }

    fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: &P) {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(
                self.as_ref().to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_icons(&self, icons: &[&str]) {
        unsafe {
            ffi::gtk_scale_button_set_icons(self.as_ref().to_glib_none().0, icons.to_glib_none().0);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_scale_button_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn icons(&self) -> Vec<glib::GString> {
        unsafe {
            let mut value =
                glib::Value::from_type(<Vec<glib::GString> as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"icons\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icons` getter")
        }
    }

    #[doc(alias = "popdown")]
    fn connect_popdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popdown_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScaleButton,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ScaleButton>,
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popdown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    popdown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_popdown(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("popdown", &[])
                .unwrap()
        };
    }

    #[doc(alias = "popup")]
    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScaleButton,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ScaleButton>,
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_popup(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("popup", &[])
                .unwrap()
        };
    }

    #[doc(alias = "value-changed")]
    fn connect_value_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P, f64) + 'static>(
            this: *mut ffi::GtkScaleButton,
            value: libc::c_double,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ScaleButton>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ScaleButton::from_glib_borrow(this).unsafe_cast_ref(),
                value,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "adjustment")]
    fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScaleButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ScaleButton>,
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icons")]
    fn connect_icons_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icons_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScaleButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ScaleButton>,
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icons_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkScaleButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<ScaleButton>,
        {
            let f: &F = &*(f as *const F);
            f(&ScaleButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ScaleButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ScaleButton")
    }
}
