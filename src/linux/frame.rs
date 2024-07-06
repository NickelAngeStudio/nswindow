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

use crate::frame::WindowFrameButtonMode;

use super::{wayland::frame::WaylandWindowFrame, x11::frame::X11WindowFrame};

#[derive(Debug, PartialEq)]
pub(crate) enum LinuxWindowFrame {
     /// X11 linux window frame
     X11(X11WindowFrame),

     /// Wayland linux window frame
     Wayland(WaylandWindowFrame)
}

impl LinuxWindowFrame {

    #[inline(always)]
    pub fn reset(&mut self) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.reset(),
            LinuxWindowFrame::Wayland(wf) => wf.reset(),
        }
    }

    #[inline(always)]
    pub fn show(&mut self) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.show(),
            LinuxWindowFrame::Wayland(wf) => wf.show(),
        }
    }

    #[inline(always)]
    pub fn hide(&mut self) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.hide(),
            LinuxWindowFrame::Wayland(wf) => wf.hide(),
        }
    }

    #[inline(always)]
    pub fn lock(&mut self) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.lock(),
            LinuxWindowFrame::Wayland(wf) => wf.lock(),
        }
    }

    #[inline(always)]
    pub fn unlock(&mut self) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.unlock(),
            LinuxWindowFrame::Wayland(wf) => wf.unlock(),
        }
    }

    #[inline(always)]
    pub fn set_button_min(&mut self, mode : WindowFrameButtonMode) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.set_button_min(mode),
            LinuxWindowFrame::Wayland(wf) => wf.set_button_min(mode),
        }
    }

    #[inline(always)]
    pub fn set_button_max(&mut self, mode : WindowFrameButtonMode) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.set_button_max(mode),
            LinuxWindowFrame::Wayland(wf) => wf.set_button_max(mode),
        }
    }

    #[inline(always)]
    pub fn set_button_close(&mut self, mode : WindowFrameButtonMode) {
        match self {
            LinuxWindowFrame::X11(wf) => wf.set_button_close(mode),
            LinuxWindowFrame::Wayland(wf) => wf.set_button_close(mode),
        }
    }

}

