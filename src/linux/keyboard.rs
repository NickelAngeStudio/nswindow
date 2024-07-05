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

use crate::keyboard::WindowKeyboardMode;

use super::x11::keyboard::X11Keyboard;

use super::wayland::keyboard::WaylandKeyboard;

/// Match abstraction of possible linux Window keyboard properties.
///
/// Match abstraction are WAY faster that [dyn] vtable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum LinuxKeyboard {

        /// X11 linux window
        X11(X11Keyboard),

        /// Wayland linux window
        Wayland(WaylandKeyboard)

}

impl LinuxKeyboard {

    #[inline(always)]
    pub fn set_mode(&mut self, mode : WindowKeyboardMode) {
        match self {
            LinuxKeyboard::X11(wkb) => wkb.set_mode(mode),
            LinuxKeyboard::Wayland(wkb) => wkb.set_mode(mode),
        }
    }

    #[inline(always)]
    pub fn enable_repeat(&mut self) {
        match self {
            LinuxKeyboard::X11(wkb) => wkb.enable_repeat(),
            LinuxKeyboard::Wayland(wkb) => wkb.enable_repeat(),
        }
    }

    #[inline(always)]
    pub fn disable_repeat(&mut self) {
        match self {
            LinuxKeyboard::X11(wkb) => wkb.disable_repeat(),
            LinuxKeyboard::Wayland(wkb) => wkb.disable_repeat(),
        }
    }

}