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

use crate::{display::Displays, event::WindowManagerEvent, Window, WindowBuilder, WindowError, WindowHandle};

use super::{wayland::manager::WaylandWindowManager, x11::manager::X11WindowManager};


/// Match abstraction of possible linux Window managers.
///
/// Match abstraction are WAY faster that [dyn] vtable.
pub(crate) enum LinuxWindowManager {

    /// X11 linux window server
    X11(X11WindowManager),

    /// Wayland linux window server
    Wayland(WaylandWindowManager)

}

impl LinuxWindowManager {

    #[inline(always)]
    pub(crate) fn event(&self) -> Option<&WindowManagerEvent> {
        match self {
            LinuxWindowManager::X11(wm) => wm.event(),
            LinuxWindowManager::Wayland(wm) => wm.event(),
        }
    }

    #[inline(always)]
    pub(crate) fn event_wait(&self) -> &WindowManagerEvent {
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
        match self {
            LinuxWindowManager::X11(wm) => wm.displays(),
            LinuxWindowManager::Wayland(wm) => wm.displays(),
        }
    }




}

