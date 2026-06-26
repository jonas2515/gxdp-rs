use glib::translate::*;
use std::ffi::CString;

pub use gxdp_sys as ffi;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GxdpServiceClientType")]
pub enum ServiceClientType {
    #[doc(alias = "GXDP_SERVICE_CLIENT_TYPE_NONE")]
    None,
    #[doc(alias = "GXDP_SERVICE_CLIENT_TYPE_PORTAL_BACKEND")]
    PortalBackend,
    #[doc(alias = "GXDP_SERVICE_CLIENT_TYPE_FILE_CHOOSER")]
    FileChooser,
    #[doc(alias = "GXDP_SERVICE_CLIENT_TYPE_GLOBAL_SHORTCUTS")]
    GlobalShortcuts,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for ServiceClientType {
    type GlibType = ffi::GxdpServiceClientType;

    #[inline]
    fn into_glib(self) -> ffi::GxdpServiceClientType {
        match self {
            Self::None => ffi::GXDP_SERVICE_CLIENT_TYPE_NONE,
            Self::PortalBackend => ffi::GXDP_SERVICE_CLIENT_TYPE_PORTAL_BACKEND,
            Self::FileChooser => ffi::GXDP_SERVICE_CLIENT_TYPE_FILE_CHOOSER,
            Self::GlobalShortcuts => ffi::GXDP_SERVICE_CLIENT_TYPE_GLOBAL_SHORTCUTS,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GxdpServiceClientType> for ServiceClientType {
    #[inline]
    unsafe fn from_glib(value: ffi::GxdpServiceClientType) -> Self {
        match value {
            ffi::GXDP_SERVICE_CLIENT_TYPE_NONE => Self::None,
            ffi::GXDP_SERVICE_CLIENT_TYPE_PORTAL_BACKEND => Self::PortalBackend,
            ffi::GXDP_SERVICE_CLIENT_TYPE_FILE_CHOOSER => Self::FileChooser,
            ffi::GXDP_SERVICE_CLIENT_TYPE_GLOBAL_SHORTCUTS => Self::GlobalShortcuts,
            value => Self::__Unknown(value),
        }
    }
}

glib::wrapper! {
    #[doc(alias = "GxdpExternalWindow")]
    pub struct ExternalWindow(Object<ffi::GxdpExternalWindow, ffi::GxdpExternalWindowClass>);

    match fn {
        type_ => || ffi::gxdp_external_window_get_type(),
    }
}

impl ExternalWindow {
    #[doc(alias = "gxdp_external_window_new_from_handle")]
    pub fn new_from_handle(handle_str: &str) -> Option<Self> {
        let handle = CString::new(handle_str).ok()?;
        unsafe { from_glib_full(ffi::gxdp_external_window_new_from_handle(handle.as_ptr())) }
    }

    #[doc(alias = "gxdp_external_window_set_parent_of")]
    pub fn set_parent_of(&self, surface: &gdk4::Surface) {
        unsafe {
            ffi::gxdp_external_window_set_parent_of(
                self.to_glib_none().0,
                surface.to_glib_none().0,
            );
        }
    }
}

#[doc(alias = "gxdp_init_gtk")]
pub fn init_gtk(
    service_client_type: ServiceClientType,
    portal_interfaces: &[&str],
) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();
        let ret = ffi::gxdp_init_gtk(
            service_client_type.into_glib(),
            if portal_interfaces.is_empty() {
                std::ptr::null_mut()
            } else {
                portal_interfaces.to_glib_full()
            },
            &mut error,
        );
        if from_glib(ret) {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}
