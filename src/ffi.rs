use std::ffi::{CString, NulError};
use std::os::raw::{c_char, c_int, c_void};
use std::mem;
use WebView;

#[repr(C)]
pub enum DialogType {
    Open,
    Save,
    Alert,
}

#[repr(C)]
pub struct Window {
    pub url: *const c_char,
    pub title: *const c_char,
    pub width: c_int,
    pub height: c_int,
    pub resizable: c_int,
    pub debug: c_int,
    pub private: Vec<u8>,
}

#[repr(C)]
pub struct DispatchUserData<'a> {
    pub callback: Box<Fn(&WebView) -> ()>,
    pub view: *mut &'a WebView,
}

pub fn to_cstring_ptr<S: Into<String>>(string: S) -> Result<*const c_char, NulError> {
    let string = CString::new(string.into())?;
    let ptr = string.as_ptr();
    mem::forget(string);

    Ok(ptr)
}

pub extern "C" fn dispatcher(window: &Window, data: *mut DispatchUserData) -> () {
    let data = unsafe { Box::from_raw(data) as Box<DispatchUserData> };
    unsafe { (data.callback)(*data.view) };
}

pub const WEBVIEW_DIALOG_FLAG_FILE: c_int = (0 << 0);
pub const WEBVIEW_DIALOG_FLAG_DIRECTORY: c_int = (1 << 0);
pub const WEBVIEW_DIALOG_FLAG_INFO: c_int = (1 << 1);
pub const WEBVIEW_DIALOG_FLAG_WARNING: c_int = (2 << 1);
pub const WEBVIEW_DIALOG_FLAG_ERROR: c_int = (3 << 1);
pub const WEBVIEW_DIALOG_FLAG_ALERT_MASK: c_int = (3 << 1);
pub const DEFAULT_URL: &str = "data:text/html,%3Cbody%3E%3C%2Fbody%3E";

extern "C" {
    pub fn get_priv_size() -> usize;
    pub fn r_webview_init(webview: &Window) -> c_int;
    pub fn r_webview_loop(webview: &Window, blocking: c_int) -> c_int;
    pub fn r_webview_exit(webview: &Window);
    pub fn r_webview_eval(webview: &Window, js: *const c_char) -> c_int;
    pub fn r_webview_inject_css(webview: &Window, css: *const c_char) -> c_int;
    pub fn r_webview_set_title(webview: &Window, title: *const c_char) -> ();
    pub fn r_webview_set_fullscreen(webview: &Window, fullscreen: c_int) -> ();
    pub fn r_webview_dialog(
        webview: &Window,
        dlgtype: DialogType,
        flags: c_int,
        title: *const c_char,
        arg: *const c_char,
        result: *mut c_char,
        resultsz: isize,
    ) -> ();
    pub fn r_webview_dispatch(
        webview: &Window,
        cb: extern "C" fn(window: &Window, arg: *mut DispatchUserData) -> (),
        arg: *mut c_void,
    ) -> ();

    pub fn r_webview_terminate(window: &Window) -> ();
}
