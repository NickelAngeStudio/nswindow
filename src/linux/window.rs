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

use crate::{WindowBuilder, WindowError, WindowFullScreenMode, WindowHandle, WindowPosition, WindowRelativePosition, WindowSize};

use super::{wayland::window::WaylandWindow, x11::window::X11Window};


/// Match abstraction of possible linux Window.
///
/// Match abstraction are WAY faster that [dyn] vtable.
#[derive(Debug, PartialEq)]
pub(crate) enum LinuxWindow {

    /// X11 linux window
    X11(X11Window),

    /// Wayland linux window
    Wayland(WaylandWindow)

}

impl LinuxWindow {

    
    #[inline(always)]
    pub fn set_title(&mut self, title : &str) -> Result<bool, WindowError>{
        todo!()
    }

    #[inline(always)]
    pub fn set_size(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        todo!()
    }

    #[inline(always)]
    pub fn set_size_min(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        todo!()
    }

    #[inline(always)]
    pub fn set_size_max(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        todo!()
    }

    #[inline(always)]
    pub fn set_icon(&mut self, icon : Option<&mut dyn std::io::Read>) {
        todo!()
    }

    #[inline(always)]
    pub fn set_taskbar(&self, show : bool) {
        todo!()
    }

    #[inline(always)]
    pub fn restore(&mut self) -> Result<bool, WindowError> {
        todo!()
    }

    #[inline(always)]
    pub fn show(&mut self) -> Result<bool, WindowError> {
        todo!()
    }

    #[inline(always)]
    pub fn hide(&mut self) -> Result<bool, WindowError> {
        todo!()
    }

    #[inline(always)]
    pub fn set_position(&mut self, position : WindowRelativePosition) -> Result<WindowPosition, WindowError> {
        todo!()
    }


    #[inline(always)]
    pub fn set_fullscreen(&mut self, fsmode : WindowFullScreenMode) -> &mut Self {
        todo!()
    }

    #[inline(always)]
    pub fn minimize(&mut self)  {
        todo!()
    }

    #[inline(always)]
    pub fn maximize(&mut self) {
        todo!()
    }

    
}