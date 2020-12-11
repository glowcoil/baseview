use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::window_open_options::WindowOpenOptions;
use crate::WindowHandler;

#[cfg(target_os = "macos")]
use crate::macos as platform;
#[cfg(target_os = "windows")]
use crate::win as platform;
#[cfg(target_os = "linux")]
use crate::x11 as platform;

use std::marker::PhantomData;

pub trait WindowHandler {
    fn on_frame(&mut self);
    fn on_event(&mut self, window: &mut Window, event: Event);
}

pub struct Window<'a> {
    window: &'a mut platform::Window,
    // so that Window is !Send on all platforms
    phantom: PhantomData<*mut ()>,
}

impl<'a> Window<'a> {
    pub(crate) fn new(window: &mut platform::Window) -> Window {
        Window {
            window,
            phantom: PhantomData,
        }
    }

    pub fn open_parented<H, B>(parent: RawWindowHandle, options: WindowOpenOptions, build: B)
    where
        H: WindowHandler + 'static,
        B: FnOnce(&mut Window) -> H,
        B: Send + 'static,
    {
        platform::Window::open::<H, B>(options, build)
    }

    pub fn open_as_if_parented<H, B>(options: WindowOpenOptions, build: B) -> RawWindowHandle
    where
        H: WindowHandler + 'static,
        B: FnOnce(&mut Window) -> H,
        B: Send + 'static,
    {
        platform::Window::open::<H, B>(options, build)
    }

    pub fn open_blocking<H, B>(parent: RawWindowHandle, options: WindowOpenOptions, build: B)
    where
        H: WindowHandler + 'static,
        B: FnOnce(&mut Window) -> H,
        B: Send + 'static,
    {
        platform::Window::open::<H, B>(options, build)
    }
}

unsafe impl<'a> HasRawWindowHandle for Window<'a> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}
