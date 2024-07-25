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
/// 
/// # Example
/// ```
/// 
/// ```
pub struct WindowManager {
    /// Linux [WindowManager] abstraction for calls.
    #[cfg(target_os = "linux")]
    wm : crate::linux::manager::LinuxWindowManager,
}

impl WindowManager {

    /// Create a new instance of window manager.
    /// 
    /// Returns Ok(WindowManager) on success.
    /// 
    /// # Errors
    /// Returns Err([`WindowError::NoWindowManager`]) if no suitable window manager available on system.
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
    #[inline(always)]
    pub(crate) fn build(&mut self, builder : &WindowBuilder) -> Result<WindowHandle, WindowError> {

        // WindowRelativePositionOOB

        // WindowSizeOOB

        // WindowMinSizeBiggerThanMax

        // WindowParentSelf
    
        // WindowParentLoop


        todo!()
    }


    /// Poll an event from the window manager.
    /// 
    /// Returns Some(WindowManagerEvent) if any, [Option::None] if no event.
    #[inline(always)]
    pub fn event(&mut self) -> Option<&WindowManagerEvent> {
        self.wm.event()
    }

    /// Wait an event from the window manager.
    /// 
    /// This will block code execution until a [WindowManagerEvent] occur.
    /// Mostly used for [Retained Mode](https://en.wikipedia.org/wiki/Retained_mode) GUI application.
    #[inline(always)]
    pub fn event_wait(&mut self) -> &WindowManagerEvent {
        self.wm.event_wait()
    }

    /// Returns an immutable reference to [Window] if [WindowHandle] is valid, 
    /// err([NSWNDError::InvalidWindowHandle]) otherwise.
    #[inline(always)]
    pub fn window(&self, window : WindowHandle) -> Result<&Window, WindowError> {
        self.wm.window(window)
    } 
    
    /// Returns a mutable reference to [Window] if [WindowHandle] is valid, 
    /// err([NSWNDError::InvalidWindowHandle]) otherwise.
    #[inline(always)]
    pub fn window_mut(&mut self, window : WindowHandle) -> Result<&mut Window, WindowError> {
        self.wm.window_mut(window)
    }

    /// Get the client screens informations. 
    #[inline(always)]
    pub fn displays(&self) -> &Displays {
        self.wm.displays()
    }

    

}


/*************
* UNIT TESTS * 
*************/
#[cfg(test)]
mod tests{
    use std::{i32, u32};

    use crate::{ WindowBuilder, WindowError, WindowHandle, WindowManager};

    /// WindowManager::build() unit test
    #[test]
    fn window_manager_ut_build() {
        match WindowManager::new() {
            Ok(mut wm) => {
                match WindowBuilder::new().build(&mut wm){
                    Ok(_) => { },
                    Err(err) => panic!("{:?}", err),
                }
            },
            Err(err) => assert!(false, "{:?}", err),
        }
    }

    /// WindowManager::event() unit test
    #[test]
    fn window_manager_ut_event() {
        match WindowManager::new() {
            Ok(mut wm) => {
                wm.event();
            },
            Err(err) => assert!(false, "{:?}", err),
        }
    }

    /// WindowManager::window() and WindowManager::window_mut() unit test
    #[test]
    fn window_manager_ut_window() {
        match WindowManager::new() {
            Ok(mut wm) => {
                match WindowBuilder::new().build(&mut wm){
                    Ok(wh) => {
                        match wm.window(wh){
                            Ok(_) => {},
                            Err(err) => panic!("{:?}", err),
                        }
                        
                        match wm.window_mut(wh){
                            Ok(_) => {},
                            Err(err) => panic!("{:?}", err),
                        }

                     },
                    Err(err) => panic!("{:?}", err),
                }
            },
            Err(err) => assert!(false, "{:?}", err),
        }
    }

    /// WindowManager::displays() unit test
    #[test]
    fn window_manager_ut_displays() {
        match WindowManager::new() {
            Ok(wm) => {
                wm.displays();
            },
            Err(err) => assert!(false, "{:?}", err),
        }
    }

    /// WindowManager::build() error unit tests
    #[test]
    fn window_manager_ut_build_error() {
        match WindowManager::new() {
            Ok(mut wm) => {
                let invalid_handle : WindowHandle = &0;

                // WindowRelativePositionOOB
                match WindowBuilder::new().hide()
                    .position(crate::WindowRelativePosition::Desktop(crate::WindowPosition { x: i32::MAX, y: i32::MAX }))
                    .build(&mut wm) {
                    Ok(_) => panic!("WindowRelativePositionOOB expected"),
                    Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
                } 

                // WindowSizeOOB
                match WindowBuilder::new().hide()
                    .size(crate::WindowSize { width: u32::MAX, height: u32::MAX })
                    .build(&mut wm) {
                    Ok(_) => panic!("WindowSizeOOB expected"),
                    Err(err) => assert!(err == WindowError::WindowSizeOOB),
                } 

                // WindowMinSizeBiggerThanMax
                match WindowBuilder::new().hide()
                    .size_min(crate::WindowSize { width: u32::MAX, height: u32::MAX })
                    .build(&mut wm) {
                    Ok(_) => panic!("WindowMinSizeBiggerThanMax expected"),
                    Err(err) => assert!(err == WindowError::WindowMinSizeBiggerThanMax),
                } 

                // InvalidWindowHandle for parent
                match WindowBuilder::new().hide()
                    .parent(Some(invalid_handle))
                    .build(&mut wm) {
                    Ok(_) => panic!("WindowMinSizeBiggerThanMax expected"),
                    Err(err) => assert!(err == WindowError::WindowMinSizeBiggerThanMax),
                }
                
            },
            Err(err) => assert!(false, "{:?}", err),
        }

    }

    /// WindowManager::window() and window_mut() error unit tests
    #[test]
    fn window_manager_ut_window_error() {
        match WindowManager::new() {
            Ok(mut wm) => {
                let invalid_handle : WindowHandle = &0;
                assert!(wm.window(invalid_handle).expect_err("InvalidWindowHandle expected!") == WindowError::InvalidWindowHandle);
                assert!(wm.window_mut(invalid_handle).expect_err("InvalidWindowHandle expected!") == WindowError::InvalidWindowHandle);
            },
            Err(err) => assert!(false, "{:?}", err),
        }
    }

}