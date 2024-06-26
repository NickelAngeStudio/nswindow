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



use crate::{event::WindowManagerEvent, display::Displays};

pub struct WaylandWindowManager {
    
}

impl WaylandWindowManager {
    /// Create a new instance of WaylandWindowManager
    pub fn new() -> WaylandWindowManager {
        WaylandWindowManager {}
    }

    pub fn event(&self) -> Option<WindowManagerEvent> {
        todo!()
    }

    #[inline(always)]
    pub fn displays(&self) -> &Displays {
        todo!()
    }
}

pub struct WaylandWindow {
    
}

/// This function spawn a new thread and try to connect to wayland server to see if available.
/// 
/// TODO:Create thread
/// 
/// Return true if wayland server is available and supported. False otherwise.
pub fn wayland_supported() -> bool { 
    false
}