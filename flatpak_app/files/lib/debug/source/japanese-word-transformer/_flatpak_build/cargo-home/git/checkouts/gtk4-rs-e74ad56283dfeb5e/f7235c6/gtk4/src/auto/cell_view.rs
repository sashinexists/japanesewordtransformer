// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::CellArea;
use crate::CellAreaContext;
use crate::CellLayout;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Orientable;
use crate::Orientation;
use crate::Overflow;
use crate::TreeModel;
use crate::TreePath;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct CellView(Object<ffi::GtkCellView>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_view_get_type(),
    }
}

impl CellView {
    #[doc(alias = "gtk_cell_view_new")]
    pub fn new() -> CellView {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_cell_view_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_cell_view_new_with_context")]
    #[doc(alias = "new_with_context")]
    pub fn with_context<P: IsA<CellArea>, Q: IsA<CellAreaContext>>(
        area: &P,
        context: &Q,
    ) -> CellView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_context(
                area.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_markup")]
    #[doc(alias = "new_with_markup")]
    pub fn with_markup(markup: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_markup(markup.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_text")]
    #[doc(alias = "new_with_text")]
    pub fn with_text(text: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_text(text.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_new_with_texture")]
    #[doc(alias = "new_with_texture")]
    pub fn with_texture<P: IsA<gdk::Texture>>(texture: &P) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_texture(
                texture.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_cell_view_get_displayed_row")]
    #[doc(alias = "get_displayed_row")]
    pub fn displayed_row(&self) -> Option<TreePath> {
        unsafe { from_glib_full(ffi::gtk_cell_view_get_displayed_row(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_cell_view_get_draw_sensitive")]
    #[doc(alias = "get_draw_sensitive")]
    pub fn draws_sensitive(&self) -> bool {
        unsafe { from_glib(ffi::gtk_cell_view_get_draw_sensitive(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_cell_view_get_fit_model")]
    #[doc(alias = "get_fit_model")]
    pub fn fits_model(&self) -> bool {
        unsafe { from_glib(ffi::gtk_cell_view_get_fit_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_cell_view_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_cell_view_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_cell_view_set_displayed_row")]
    pub fn set_displayed_row(&self, path: &mut TreePath) {
        unsafe {
            ffi::gtk_cell_view_set_displayed_row(self.to_glib_none().0, path.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gtk_cell_view_set_draw_sensitive")]
    pub fn set_draw_sensitive(&self, draw_sensitive: bool) {
        unsafe {
            ffi::gtk_cell_view_set_draw_sensitive(
                self.to_glib_none().0,
                draw_sensitive.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_cell_view_set_fit_model")]
    pub fn set_fit_model(&self, fit_model: bool) {
        unsafe {
            ffi::gtk_cell_view_set_fit_model(self.to_glib_none().0, fit_model.into_glib());
        }
    }

    #[doc(alias = "gtk_cell_view_set_model")]
    pub fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>) {
        unsafe {
            ffi::gtk_cell_view_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "cell-area")]
    pub fn cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = glib::Value::from_type(<CellArea as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"cell-area\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `cell-area` getter")
        }
    }

    #[doc(alias = "cell-area-context")]
    pub fn cell_area_context(&self) -> Option<CellAreaContext> {
        unsafe {
            let mut value = glib::Value::from_type(<CellAreaContext as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"cell-area-context\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `cell-area-context` getter")
        }
    }

    #[doc(alias = "draw-sensitive")]
    pub fn connect_draw_sensitive_notify<F: Fn(&CellView) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_sensitive_trampoline<F: Fn(&CellView) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-sensitive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_sensitive_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fit-model")]
    pub fn connect_fit_model_notify<F: Fn(&CellView) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fit_model_trampoline<F: Fn(&CellView) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fit-model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fit_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&CellView) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&CellView) + 'static>(
            this: *mut ffi::GtkCellView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellView {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct CellViewBuilder {
    cell_area: Option<CellArea>,
    cell_area_context: Option<CellAreaContext>,
    draw_sensitive: Option<bool>,
    fit_model: Option<bool>,
    model: Option<TreeModel>,
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

impl CellViewBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CellView {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref cell_area) = self.cell_area {
            properties.push(("cell-area", cell_area));
        }
        if let Some(ref cell_area_context) = self.cell_area_context {
            properties.push(("cell-area-context", cell_area_context));
        }
        if let Some(ref draw_sensitive) = self.draw_sensitive {
            properties.push(("draw-sensitive", draw_sensitive));
        }
        if let Some(ref fit_model) = self.fit_model {
            properties.push(("fit-model", fit_model));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
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
        glib::Object::new::<CellView>(&properties)
            .expect("Failed to create an instance of CellView")
    }

    pub fn cell_area<P: IsA<CellArea>>(mut self, cell_area: &P) -> Self {
        self.cell_area = Some(cell_area.clone().upcast());
        self
    }

    pub fn cell_area_context<P: IsA<CellAreaContext>>(mut self, cell_area_context: &P) -> Self {
        self.cell_area_context = Some(cell_area_context.clone().upcast());
        self
    }

    pub fn draw_sensitive(mut self, draw_sensitive: bool) -> Self {
        self.draw_sensitive = Some(draw_sensitive);
        self
    }

    pub fn fit_model(mut self, fit_model: bool) -> Self {
        self.fit_model = Some(fit_model);
        self
    }

    pub fn model<P: IsA<TreeModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
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

impl fmt::Display for CellView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellView")
    }
}
