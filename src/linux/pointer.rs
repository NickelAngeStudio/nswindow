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

use crate::pointer::{WindowCursor, WindowPointerMode};

use super::{wayland::pointer::WaylandPointer, x11::pointer::X11Pointer};

/// Match abstraction of possible linux Window managers pointers.
///
/// Match abstraction are WAY faster that [dyn] vtable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum LinuxPointer {

        /// X11 linux window
        X11(X11Pointer),

        /// Wayland linux window
        Wayland(WaylandPointer)

}

impl LinuxPointer {

    pub fn set_mode(&mut self, mode : WindowPointerMode) {
        match self {
            LinuxPointer::X11(lp) => lp.set_mode(mode),
            LinuxPointer::Wayland(lp) => lp.set_mode(mode),
        }
    }

    pub fn show(&mut self) {
        match self {
            LinuxPointer::X11(lp) => lp.show(),
            LinuxPointer::Wayland(lp) => lp.show(),
        }
    }

    pub fn hide(&mut self) {
        match self {
            LinuxPointer::X11(lp) => lp.hide(),
            LinuxPointer::Wayland(lp) => lp.hide(),
        }
    }

    pub fn confine(&mut self) {
        match self {
            LinuxPointer::X11(lp) => lp.confine(),
            LinuxPointer::Wayland(lp) => lp.confine(),
        }
    }

    pub fn release(&mut self) {
        match self {
            LinuxPointer::X11(lp) => lp.release(),
            LinuxPointer::Wayland(lp) => lp.release(),
        }
    }

    pub fn set_cursor(&mut self, cursor : WindowCursor) {
        match self {
            LinuxPointer::X11(lp) => lp.set_cursor(cursor),
            LinuxPointer::Wayland(lp) => lp.set_cursor(cursor),
        }
    }

}
