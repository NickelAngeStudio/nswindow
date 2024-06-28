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



use crate::{ display::{Display, DisplayHandle}, NSWindowError, WindowHandle};

 
/// [Window] size as width and height.
 #[derive(Debug, PartialEq, Clone)]
pub struct WindowSize {
    pub width : u32,
    pub height : u32,
}

/// [Window] position according to x and y axis.
#[derive(Debug, PartialEq, Clone)]
pub struct WindowPosition {
    pub x : i32,
    pub y : i32,
}


/// Enumeration of possible window positions when setting position.
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
}

/// [Window] fullscreen modes.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowFullScreenMode {

    /// Window will be set fullscreen in the current display this window belong to.
    Current,

    /// Window will be set fullscreen in the primary displat.
    Primary,

    /// Window will be set fullscreen for entire desktop which can be set across multiple display.
    Desktop,

    /// Window will be set fullscreen for the specified display
    Display(DisplayHandle)
}

/// [Window] is used to manipulate an individual window.
/// 
/// It is possible to get the operatin system window manager handle via [Window::get_os_handle].
/// TODO: Develop more
pub struct Window {
    /// Linux [Window] abstraction for calls.
    #[cfg(target_os = "linux")]
    window : crate::linux::LinuxWindow,  


    /// Parent [WindowHandle]
    parent : Option<WindowHandle>,

}


impl Window {

    /// Get [WindowHandle].
    fn handle(&self) -> WindowHandle {
        todo!()
    }

    /// Get [Window] title.
    fn title(&self) -> &str {
        todo!()
    }

    /// Set the [Window] title.
    fn set_title(&mut self, title : &str) -> Result<bool, NSWindowError>{
        todo!()
    }

    /// Get [WindowSize] of [Window].
    fn size(&self) -> WindowSize {
        todo!()
    }

    /// Set [WindowSize] of [Window].
    fn set_size(&mut self, size : WindowSize) -> Result<bool, NSWindowError> {
        todo!()
    }

    /// Get [Window] height.
    fn height(&self) -> u32 {
        todo!()
    }

    /// Set [Window] height.
    fn set_height(&mut self) -> Result<bool, NSWindowError> {
        todo!()
    }

    /// Get [Window] width.
    fn width(&self) -> u32 {
        todo!()
    }

    /// Set [Window] width.
    fn set_width(&mut self) -> Result<bool, NSWindowError> {
        todo!()
    }

     /// Returns true if [Window] if visible (show() was called)wm
     fn visible(&mut self) -> Result<bool, NSWindowError> {
        todo!()
    }

    /// Show the [Window] on display.
    fn show(&mut self) -> Result<bool, NSWindowError> {
        todo!()
    }

    /// Close [Window], removing it from display.
    fn close(&mut self) -> Result<bool, NSWindowError> {
        todo!()
    }

    /// Position of the [Window] on the desktop as [WindowPosition] tuple.
    fn position(&mut self) -> WindowPosition {
        todo!()
    }

    /// Set the position of the [Window] with a [WindowRelativePosition].
    /// 
    /// Returns Ok([WindowPosition]) with the position on the desktop or [NSWindowError::WindowRelativePositionOOB] if position is invalid.
    fn set_position(&mut self, position : WindowRelativePosition) -> Result<WindowPosition, NSWindowError> {
        todo!()
    }


    // TODO: Determine window position handling
    // Get relative position of window
    //fn position() -> ;

    // Set relative position of window
    //fn set position() -> ;

    // Get absolute position of window
    //fn position_absolute()

    // Set absolute position of window
    //fn set_position_absolute;

    /// Returns true if [Window] has decoration.
    fn decoration(&self) -> bool {
        todo!()
    }




    // Show window decoration
    // TODO:Rest of functions
/*
    WindowPropertySet::Position(option) => self.set_position(option),
    WindowPropertySet::ShowDecoration => self.show_decoration(),
    WindowPropertySet::HideDecoration => self.hide_decoration(),
    WindowPropertySet::Minimize => self.minimize(),
    WindowPropertySet::Maximized => self.maximize(),
    WindowPropertySet::Fullscreen(fsmode) => self.set_fullscreen(fsmode.clone()),
    WindowPropertySet::Restore => self.restore(),
    WindowPropertySet::Keyboard(kb_property) => self.set_keyboard_property(kb_property),
    WindowPropertySet::Pointer(p_property) => self.set_pointer_property(p_property),
*/

}