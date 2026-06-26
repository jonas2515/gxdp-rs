use gdk4_sys::GdkSurface;
use glib_sys::{GError, GType, gboolean};
use gobject_sys::{GObject, GObjectClass};
use libc::{c_char, c_int};

pub type GxdpServiceClientType = c_int;
pub const GXDP_SERVICE_CLIENT_TYPE_NONE: GxdpServiceClientType = 0;
pub const GXDP_SERVICE_CLIENT_TYPE_PORTAL_BACKEND: GxdpServiceClientType = 1;
pub const GXDP_SERVICE_CLIENT_TYPE_FILE_CHOOSER: GxdpServiceClientType = 2;
pub const GXDP_SERVICE_CLIENT_TYPE_GLOBAL_SHORTCUTS: GxdpServiceClientType = 3;

#[repr(C)]
pub struct GxdpExternalWindow {
    pub parent_instance: GObject,
}

#[repr(C)]
pub struct GxdpExternalWindowClass {
    pub parent_class: GObjectClass,
    pub set_parent_of: Option<
        unsafe extern "C" fn(external_window: *mut GxdpExternalWindow, surface: *mut GdkSurface),
    >,
}

unsafe extern "C" {
    pub fn gxdp_external_window_get_type() -> GType;

    pub fn gxdp_external_window_new_from_handle(
        handle_str: *const c_char,
    ) -> *mut GxdpExternalWindow;

    pub fn gxdp_external_window_set_parent_of(
        external_window: *mut GxdpExternalWindow,
        surface: *mut GdkSurface,
    );

    pub fn gxdp_init_gtk(
        service_client_type: GxdpServiceClientType,
        portal_interfaces: *mut *const c_char,
        error: *mut *mut GError,
    ) -> gboolean;
}
