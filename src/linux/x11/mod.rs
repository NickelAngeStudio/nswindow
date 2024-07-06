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

use std::{panic::catch_unwind, thread};

/// XLib bindings
pub(crate) mod xlib;

/// X11 Window Manager
pub(crate) mod manager;

/// X11 Window handle
pub(crate) mod window;

/// X11 Window frame management
pub(crate) mod frame;

/// X11 Screen informations
pub(crate) mod display;

/// X11 Atoms informations
pub(crate) mod atom;

/// X11 Event fetch
pub(crate) mod event;

/// X11 pointer
pub(crate) mod pointer;

/// X11 Keyboard
pub(crate) mod keyboard;

/// This function spawn a new thread and try to connect to X11 server to see if available.
/// 
/// Return true if x11 server is available and supported. False otherwise.
pub fn x11_supported() -> bool { 

    unsafe {
        // Spawn thread to connect to x11.
        let thread_join_handle = thread::spawn(move || {
            // Try to call C function with error handling.
            let result = catch_unwind(|| {
                xlib::XOpenDisplay(std::ptr::null())
            }); 

            match result {
                Ok(display) => {
                    if display == std::ptr::null_mut() {
                        false
                    } else {
                        // Disconnect display before returning true
                        xlib::XCloseDisplay(display);

                        true
                    }
                },

                // Error occurred, not compatible.
                Err(_) => false,
            }
        });

        match thread_join_handle.join() {
            Ok(value) => {  // Use returned thread value.
                value
            },
            Err(_) => {
                // X11 Not supported
                false
            },
        }
    }
}