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

use crate::display::DisplayHandle;

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

/// Enumeration of possible keyboard mode for input.
/// 
/// fsdssf
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowKeyboardMode {
    /// Direct mode is faster and more suitable for games. Provides [EventKeyboard::KeyUp](super::event::keyboard::EventKeyboard)
    /// and [EventKeyboard::KeyDown](super::event::keyboard::EventKeyboard).
    Direct,

    /// Text mode is slower since it provides more information for text entry. Provides [EventKeyboard::KeyPress](super::event::keyboard::EventKeyboard).
    Text,
}

/*
/// Contains keyboard properties.
#[derive(Debug, Clone, Copy)]
pub struct WindowKeyboard {

    /// [KeyboardMode] of the keyboard. Use [KeyboardMode::DirectInput] by default.
    pub mode:WindowKeyboardMode,

    /// If enabled, keys are repeated when pressed down. Disabled by default.
    pub auto_repeat : bool,

}

impl WindowKeyboard {
    /// Create new instance of keyboard property with auto repeat to false.
    pub(crate) fn new() -> WindowKeyboard {
        WindowKeyboard { mode : WindowKeyboardMode::DirectInput, auto_repeat: false }
    }
}
*/

/// Enumeration of possible [Window](super::window::Window) pointer mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowCursorMode {
    /// [EventMouse](super::event::EventMouse) events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces interactions.
    Cursor,

    /// [EventMouse](super::event::EventMouse) events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}

/*
/// [Window](super::window::Window) cursor properties such as mode, position, etc.
#[derive(Debug, Clone, Copy)]
pub struct WindowCursor {
    /// [PointerMode] used for [EventMouse](super::event::EventMouse) events.
    pub mode : WindowCursorMode,

    /// Indicate if cursor is visible or hidden.
    pub visible : bool,

    /// Indicate if cursor is confined to the window boundaries or not.
    pub confined : bool, 
}



impl WindowCursor {
    /// Create a new [PointerProperty] with default values.
    pub(crate) fn new() -> WindowCursor {
        WindowCursor{ 
            mode: WindowCursorMode::Cursor, 
            visible: true, 
            confined: false,
        }
    }
}
*/
