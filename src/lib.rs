extern crate urlencoding;

mod ffi;

use ffi::*;
use std::ffi::NulError;
use std::os::raw::c_void;
use std::fmt::Display;
use urlencoding::encode;

pub enum Content<S: Into<String> + Display> {
    Url(S),
    Html(S),
}

pub struct WebView {
    window: Box<Window>,
}

impl WebView {
    pub fn new<S: Into<String>, SD: Into<String> + Display>(
        title: S,
        content: Content<SD>,
        width: i32,
        height: i32,
        resizable: bool,
        debug: bool,
    ) -> Result<Self, NulError> {
        let url = {
            match content {
                Content::Url(url) => url.into(),
                Content::Html(html) => format!("data:text/html,{}", encode(&html.into())),
            }
        };

        let priv_size = unsafe { get_priv_size() };

        let window = Box::new(Window {
            url: to_cstring_ptr(url)?,
            title: to_cstring_ptr(title)?,
            width: width,
            height: height,
            resizable: resizable as i32,
            debug: debug as i32,
            private: vec![0; priv_size],
        });

        unsafe {
            r_webview_init(&window);
        }

        Ok(WebView { window: window })
    }

    pub fn dispatch<T: Fn(&WebView) -> () + 'static>(&self, cb: T) {
        let window = &self.window;
        let boxed_self = Box::new(self);

        let data = Box::new(DispatchUserData {
            callback: Box::new(cb),
            view: Box::into_raw(boxed_self),
        });

        let raw = Box::into_raw(data);

        unsafe {
            r_webview_dispatch(window, dispatcher, raw as *mut c_void);
        }
    }

    pub fn set_title<S: Into<String>>(&self, title: S) -> Result<(), NulError> {
        let window = &self.window;
        let title = to_cstring_ptr(title.into())?;

        unsafe {
            r_webview_set_title(window, title);
        }

        Ok(())
    }

    pub fn eval<S: Into<String>>(&self, js: S) -> Result<(), NulError> {
        let window = &self.window;
        let js = to_cstring_ptr(js.into())?;

        unsafe {
            r_webview_eval(window, js);
        }

        Ok(())
    }

    pub fn inject_css<S: Into<String>>(&self, css: S) -> Result<(), NulError> {
        let window = &self.window;
        let css = to_cstring_ptr(css.into())?;

        unsafe {
            r_webview_inject_css(window, css);
        }

        Ok(())
    }

    pub fn join(&self) {
        let window = &self.window;

        unsafe { while r_webview_loop(window, 1) == 0 {} }
    }

    pub fn exit(&self) {
        let window = &self.window;

        unsafe {
            r_webview_terminate(window);
        }
    }

    pub fn loop_once(&self, blocking: bool) -> bool {
        let window = &self.window;

        unsafe { r_webview_loop(window, blocking as i32) == 0 }
    }
}

impl Drop for WebView {
    fn drop(&mut self) {
        let window = &self.window;

        unsafe {
            r_webview_exit(window);
        }
    }
}
