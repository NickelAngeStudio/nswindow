/* 
Copyright (c) 2024  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/nswindow

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


use nscfg::{match_cfg, target_cfg};

use crate::{display::Displays, WindowError, Window, WindowBuilder, WindowHandle, event::WindowManagerEvent};


/// [WindowManager] is used to create and manipulate [Window].
/// 
/// It is also used to fetch [WindowManagerEvent], get [Desktop] and [Displays] informations.
pub struct WindowManager {
    /// Linux [WindowManager] abstraction for calls.
    #[cfg(target_os = "linux")]
    wm : crate::linux::manager::LinuxWindowManager,
}

impl WindowManager {

    /// Create a new window manager used to create [Window], fetch [WindowManagerEvent], get [Desktop] and [Displays] informations.
    pub fn new() -> Result<WindowManager, crate::WindowError> {

        match_cfg! {
            linux => {  // Linux implementation try to open a Wayland connection first then fallback to X11.
                match WindowManager::new_wayland() {    // Try wayland first
                    Ok(wm) => Ok(wm),
                    Err(_) => match WindowManager::new_x11() {  // Fallback to X11 else
                        Ok(wm) => Ok(wm),
                        Err(_) => Err(WindowError::NoWindowManager),
                    },
                }
            },
            _ => {
                unimplemented!()
            }
        }

    }

    target_cfg! {
        linux & !doc => {
            /// Create a new x11 [WindowManager].
            pub fn new_x11() -> Result<WindowManager, crate::WindowError> {
                if super::linux::x11::x11_supported() {
                    
                    match super::linux::x11::manager::X11WindowManager::new() {
                        Ok(wm) => Ok(WindowManager{ wm: super::linux::manager::LinuxWindowManager::X11(wm)}),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(WindowError::WindowManagerNotSupported)
                }
            }

            /// Create a new wayland [WindowManager].
            pub fn new_wayland() -> Result<WindowManager, crate::WindowError> {
                if super::linux::wayland::wayland_supported() {
                    todo!()
                } else {
                    Err(WindowError::WindowManagerNotSupported)
                }
            }
        }
    }


    /// Build a [Window] from a [WindowBuilder].
    pub(crate) fn build(&mut self, builder : &WindowBuilder) -> Result<WindowHandle, WindowError> {
        todo!()
    }

    /// Rebuild a [Window] from a [WindowHandle] and a [WindowBuilder].
    pub(crate) fn rebuild(&mut self, handle : WindowHandle, builder : &WindowBuilder) -> Result<WindowHandle, WindowError> {
        todo!()
    }


    /// Poll an event from the window manager.
    /// 
    /// Returns Some(WindowManagerEvent) if any, [Option::None] if no event.
    /// 
    /// # Note
    /// If the feature `nswindow_application` is enabled, [Option::None] will never
    /// be returned and the request will lock until an event occurred.
    #[inline(always)]
    pub fn event(&self) -> Option<WindowManagerEvent> {
        self.wm.event()
    }

    /// Returns an immutable reference to [Window] if [WindowHandle] is valid, 
    /// err([NSWNDError::InvalidWindowHandle]) otherwise.
    #[inline(always)]
    pub fn window(&mut self, window : WindowHandle) -> Result<&Window, WindowError> {
        self.wm.window(window)
    } 
    
    /// Returns a mutable reference to [Window] if [WindowHandle] is valid, 
    /// err([NSWNDError::InvalidWindowHandle]) otherwise.
    #[inline(always)]
    pub fn window_mut(&mut self, window : WindowHandle) -> Result<&mut Window, WindowError> {
        self.wm.window_mut(window)
    }

    /// Close the [Window] of the [WindowHandle], removing it from display and clearing the ressources.
    /// 
    /// [WindowHandle] will be invalid if invoked.
    pub fn close(&mut self, window : WindowHandle) -> Result<bool, WindowError> {
        todo!()
    }

    /// Set a [Window] parent.
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::InvalidWindowHandle`] if [WindowHandle] doesn't refer to any [Window].
    /// [WindowBuilder::rebuild] returns [`WindowError::WindowParentSelf`] if [WindowHandle] if the same as the [Window] itself.
    /// [WindowBuilder::build] returns [`WindowError::WindowParentLoop`] if a parent loop would occur upon creation.
    pub fn set_parent(&self, child : WindowHandle, parent : Option<WindowHandle>) -> Result<bool, WindowError> {
        todo!()
    }


    /// Get the client screens informations. 
    #[inline(always)]
    pub fn displays(&self) -> &Displays {
        self.wm.displays()
    }

    

}
