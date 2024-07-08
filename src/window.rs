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



use crate::{ display::{Display, DisplayHandle}, frame::WindowFrame, keyboard::WindowKeyboard, linux::pointer, pointer::WindowPointer, WindowBuilder, WindowError};

/// Window handle used by the [WindowManager](crate::WindowManager).
/// 
/// It is the same handle used by the operating system.
pub type WindowHandle = *const usize;

/// [Window](crate::Window) size as width and height.
#[derive(Debug, PartialEq, Clone)]
pub struct WindowSize {
    pub width : u32,
    pub height : u32,
}

/// [Window](crate::Window) position according to x and y axis.
#[derive(Debug, PartialEq, Clone)]
pub struct WindowPosition {
    pub x : i32,
    pub y : i32,
}


/// Possible [Window](crate::Window) relative positions.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowRelativePosition {
    /// Position window on desktop from an absolute pair of x,y coordinates.
    Desktop(WindowPosition),

    /// Position window on a specific display from an absolute pair of x,y coordinates.
    Display(DisplayHandle, WindowPosition),

    /// Position window in the center of given display.
    DisplayCenter(DisplayHandle),

    /// Position window relative to parent window. All [Window] have parent, up to the root which is the desktop.
    Parent(WindowPosition),

    /// Position window in the center of parent window. All [Window] have parent, up to the root which is the desktop.
    ParentCenter,

    /// Position window on the center of primary display.
    PrimaryCenter,

    /// Position window on the primary display with [WindowPosition].
    Primary(WindowPosition),
}

/// Possible [Window](crate::Window) fullscreen modes.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowFullScreenMode {

    /// Window will be set fullscreen in the current display this window belong to.
    Current,

    /// Window will be set fullscreen in the primary display.
    Primary,

    /// Window will be set fullscreen for entire desktop which can be set across multiple display.
    Desktop,

    /// Window will be set fullscreen for the specified display
    Display(DisplayHandle)
}


/// [Window] is used to manipulate an individual window.
/// 
/// TODO: Develop more
pub struct Window {
    /// Linux [Window] abstraction for calls.
    #[cfg(target_os = "linux")]
    pub(crate) window : crate::linux::window::LinuxWindow,  

    /// Handle of the [Window]
    pub(crate) handle : WindowHandle,

    /// Parent [WindowHandle]
    pub(crate) parent : Option<WindowHandle>,

    /// [Window] frame properties.
    pub(crate) frame : WindowFrame,

    /// [Window] keyboard properties.
    pub(crate) keyboard : WindowKeyboard,

    /// [Window] pointer properties.
    pub(crate) pointer : WindowPointer,

    /// Title of the [Window]
    pub(crate) title : String,

    /// Minimum size of the [Window].
    pub(crate) min_size : WindowSize,

    /// Maximum size of the [Window].
    pub(crate) max_size : WindowSize,

    /// Is Window Fullscreen
    pub(crate) fullscreen : bool,

    /// Is Window minimized
    pub(crate) minimized : bool,

    /// Is Window maximized
    pub(crate) maximized : bool,

    /// Window is currently visible.
    pub(crate) visible : bool,

    /// Show the window in the taskbar.
    pub(crate) taskbar : bool,
}


impl Window {

    /// Returns the [WindowHandle] of the [Window].
    pub fn handle(&self) -> WindowHandle {
        self.handle
    }

    /// Returns the parent [WindowHandle] if any.
    /// 
    /// [Window::parent] can be set by [WindowManager::set_parent()](crate::WindowManager::set_parent()).
    pub fn parent(&self) -> Option<WindowHandle> {
        self.parent
    }

    /// Returns the [Window] title.
    pub fn title(&self) -> &str {
        todo!()
    }

    /// Set the [Window] title.
    pub fn set_title(&mut self, title : &str) -> Result<bool, WindowError>{
        todo!()
    }

    /// Get [WindowSize] of [Window].
    pub fn size(&self) -> WindowSize {
        todo!()
    }

    /// Set [WindowSize] of [Window].
    pub fn set_size(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        todo!()
    }

    /// Get the minimum [WindowSize] of [Window].
    pub fn size_min(&self) -> WindowSize {
        todo!()
    }

    /// Set the minimum [WindowSize] of [Window].
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowMinSizeBiggerThanMax`] if min `size` is bigger than `max` size.
    pub fn set_size_min(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        todo!()
    }

    /// Get the maximum [WindowSize] of [Window].
    pub fn size_max(&self) -> WindowSize {
        todo!()
    }

    /// Set the maximum [WindowSize] of [Window].
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowMinSizeBiggerThanMax`] if min `size` is bigger than `max` size.
    pub fn set_size_max(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        todo!()
    }

     /// Set the [Window] icon from a [std::io::Read] source.
     /// 
     /// Set the `icon` parameter to [None] to display default icon.
     pub fn set_icon(&mut self, icon : Option<&mut dyn std::io::Read>) {
        todo!()
    }

    /// Returns true if the [Window] is showed in the taskbar.
    pub fn taskbar(&self) -> bool {
        self.taskbar
    }

    /// Set if the [Window] is showed in the taskbar.
    pub fn set_taskbar(&self, show : bool) {
        todo!()
    }

    /// Returns true if [Window] is visible.
    pub fn visible(&self) -> bool {
        todo!()
    }

    /// Restore the [Window], removing minimize, maximize and/or fullscreen modes.
    pub fn restore(&mut self) -> Result<bool, WindowError> {
        todo!()
    }

    /// Show the [Window] on display.
    pub fn show(&mut self) -> Result<bool, WindowError> {
        todo!()
    }

    /// Hide the [Window] on display.
    pub fn hide(&mut self) -> Result<bool, WindowError> {
        todo!()
    }


    /// [WindowPosition] of the [Window] on the desktop.
    pub fn position(&mut self) -> WindowPosition {
        todo!()
    }

    /// Set the position of the [Window] with a [WindowRelativePosition].
    /// 
    /// Returns Ok([WindowPosition]) with the position on the desktop or [WindowError::WindowRelativePositionOOB] if position is invalid.
    pub fn set_position(&mut self, position : WindowRelativePosition) -> Result<WindowPosition, WindowError> {
        todo!()
    }

    /// Returns immutable reference to the [WindowFrame].
    pub fn frame(&self) -> &WindowFrame {
        &self.frame
    }

    /// Returns mutable reference to the [WindowFrame].
    pub fn frame_mut(&mut self) -> &mut WindowFrame {
        &mut self.frame
    }

    /// Returns immutable reference to the [WindowKeyboard].
    pub fn keyboard(&self) -> &WindowKeyboard {
        &self.keyboard
    }

    /// Returns mutable reference to the [WindowKeyboard].
    pub fn keyboard_mut(&mut self) -> &mut WindowKeyboard {
        &mut self.keyboard
    }

    /// Returns immutable reference to the [WindowPointer].
    pub fn pointer(&self) -> &WindowPointer {
        &self.pointer
    }

    /// Returns mutable reference to the [WindowPointer].
    pub fn pointer_mut(&mut self) -> &mut WindowPointer {
        &mut self.pointer
    }

    /// Returns true if the [Window] is currently in fullscreen mode.
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Set the [Window] fullscreen mode. Use [Window::restore()] to exit fullscreen mode.
    pub fn set_fullscreen(&mut self, fsmode : WindowFullScreenMode) -> &mut Self {
        todo!()
    }

    /// Returns true if [Window] is currently minimized.
    pub fn minimized(&self) -> bool {
        self.minimized
    }

    /// Minimize the [Window] in the taskbar.
    pub fn minimize(&mut self)  {
        todo!()
    }
    
    /// Returns true if [Window] is currently maximized.
    pub fn maximized(&mut self) -> bool {
        self.maximized
    }

    /// Maximize the [Window] in it's current display.
    pub fn maximize(&mut self) {
        todo!()
    }

}