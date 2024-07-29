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


use crate::{display::Displays, event::WindowManagerEvent, WindowBuilder, WindowError, WindowHandle};

use super::{atom::X11Atoms, xlib::Display, xlib::Window, xlib::XEvent, xlib::XOpenDisplay};

pub(crate) struct X11WindowManager {

    /// Used to fetch X11 events
    event : XEvent,  

    /// [WindowManagerEvent] kept in queue and fetched before X11 events.
    queue : Vec<WindowManagerEvent>,

    /// X11 server display connection pointer
    x11display : *mut Display,

    /// List of display
    displays : Displays, 

    /// Atoms for handling x11 window properties
    atoms : X11Atoms,

    /// [Window] collection of this manager.
    windows : Vec<Window>,

}

impl Drop for X11WindowManager {
    fn drop(&mut self) {
        unsafe {
            // Close display connection
            super::xlib::XCloseDisplay(self.x11display);
        }
    }
}

impl X11WindowManager {
    #[inline(always)]
    pub fn new() -> Result<X11WindowManager, WindowError> {

        unsafe {
            let event = XEvent { type_: 0 };
            let display = XOpenDisplay(std::ptr::null());
            let atoms = X11Atoms::new(display);

            match super::display::x11_displays() {
                Ok(screens) => Ok(X11WindowManager { 
                    event,
                    queue : Vec::new(), 
                    x11display: display, 
                    displays: screens,
                    atoms,
                    windows : Vec::new()
                }),
                Err(err) => Err(err),
            }
        }
    }
    

    #[inline(always)]
    pub fn event(&self) -> Option<&WindowManagerEvent> {
        todo!()
    }

    #[inline(always)]
    pub fn event_wait(&self) -> &WindowManagerEvent {
        todo!()
    }


    #[inline(always)]
    pub(crate) fn build(&mut self, builder : &WindowBuilder) -> Result<WindowHandle, WindowError> {
        todo!()
    }

    #[inline(always)]
    pub fn window(&mut self, window : WindowHandle) -> Result<&Window, WindowError> {
        todo!()
    } 

    #[inline(always)]
    pub fn window_mut(&mut self, window : WindowHandle) -> Result<&mut Window, WindowError> {
        todo!()
    }


    #[inline(always)]
    pub fn displays(&self) -> &Displays {
        &self.displays
    }
}
