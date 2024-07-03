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

/// Cursor glyph showed when cursor enter [Window](crate::Window).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowCursor {

    /// Normal top left arrow cursor.
    Normal,

    /// Text selection cursor.
    Select,

    /// Cursor telling that the program is working. 
    Working,

    /// Cursor telling that the program is working in background.
    WorkingInBackground,

    /// Cursor to move an object.
    Move,

    /// Cross presision cursor.
    Cross,

    /// Hand cursor usually for hyperlink.
    Hand,

    /// Cursor showing something is forbidden.
    Unavailable,

    /// Pencil cursor for editing.
    Pencil,

    /// Up arrow usually used for column selection.
    UpArrow,

    /// Diagonal resize top left bottom right.
    DiagonalResizeTLBR,

    /// Diagonal resize bottom left top right.
    DiagonalResizeBLTR,

    /// Horizontal resize cursor.
    HorizontalResize,

    /// Vertical resize cursor.
    VerticalResize,
    
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

    /// Position window on the center of primary screen.
    PrimaryCenter,

    /// Position window on the primary display according to a pair of x,y coordinates.
    Primary(WindowPosition),
}

/// Possible [Window](crate::Window) fullscreen modes.
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

/// Possible [Window](crate::Window) keyboard mode for input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowKeyboardMode {
    /// Direct mode is faster and more suitable for games. Provides [EventKeyboard::KeyUp](super::event::keyboard::EventKeyboard)
    /// and [EventKeyboard::KeyDown](super::event::keyboard::EventKeyboard).
    Direct,

    /// Text mode is slower since it provides more information for text entry. Provides [EventKeyboard::KeyPress](super::event::keyboard::EventKeyboard).
    Text,
}

/// Default [WindowKeyboardMode].
const WKB_DEFAULT_MODE : WindowKeyboardMode = WindowKeyboardMode::Direct;


/// Default auto repeat.
const WKB_DEFAULT_REPEAT : bool = false;

/// [Window](crate::Window) keyboard properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowKeyboard {

    /// [KeyboardMode] of the keyboard. Use [KeyboardMode::DirectInput] by default.
    pub(crate) mode : WindowKeyboardMode,

    /// If enabled, keys are repeated when pressed down. Disabled by default.
    pub(crate) auto_repeat : bool,

}

impl WindowKeyboard {
    /// Create new instance of keyboard property with auto repeat to false.
    pub fn new() -> WindowKeyboard {
        WindowKeyboard { mode : WKB_DEFAULT_MODE, auto_repeat : WKB_DEFAULT_REPEAT}
    }

    /// Returns the [WindowKeyboardMode] of the [Window](crate::Window).
    pub fn mode(&self) -> WindowKeyboardMode {
        self.mode
    }

}


/// Enumeration of possible [Window](super::window::Window) pointer mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowPointerMode {
    /// [EventMouse](super::event::EventMouse) events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces interactions.
    Cursor,

    /// [EventMouse](super::event::EventMouse) events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}

/// Default [WindowPointer] mode.
const WP_DEFAULT_MODE : WindowPointerMode = WindowPointerMode::Cursor;

/// Default [WindowPointer] visibility.
const WP_DEFAULT_VISIBILITY : bool = true;

/// Default [WindowPointer] confined.
const WP_DEFAULT_CONFINED : bool = false;

/// Default [WindowPointer] cursor.
const WP_DEFAULT_CURSOR : WindowCursor = WindowCursor::Normal;


/// [Window](super::window::Window) pointer properties such as mode, position, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowPointer {
    /// [PointerMode] used for [EventMouse](super::event::EventMouse) events.
    pub(crate) mode : WindowPointerMode,

    /// Indicate if cursor is visible or hidden.
    pub(crate) visible : bool,

    /// Indicate if cursor is confined to the window boundaries or not.
    pub(crate) confined : bool, 

    /// Cursor showed on the window.
    pub(crate) cursor : WindowCursor,
}



impl WindowPointer {
    /// Create a new [PointerProperty] with default values.
    pub(crate) fn new() -> WindowPointer {
        WindowPointer{ 
            mode: WP_DEFAULT_MODE, 
            visible: WP_DEFAULT_VISIBILITY, 
            confined: WP_DEFAULT_CONFINED,
            cursor: WP_DEFAULT_CURSOR,
        }
    }
}


/*************
* UNIT TESTS * 
*************/
#[cfg(test)]
mod tests{
    use crate::{property::{WKB_DEFAULT_MODE, WKB_DEFAULT_REPEAT, WP_DEFAULT_CONFINED, WP_DEFAULT_CURSOR, WP_DEFAULT_MODE, WP_DEFAULT_VISIBILITY}, WindowKeyboard, WindowPointer};

    

    /// Unit tests [super::WindowKeyboard] default values.
    ///
    /// # Verification(s)
    /// V1 | Test each value on creation vs default values.
    #[test]
    fn ut_window_keyboard_default() {
        let wkb = WindowKeyboard::new();

        // V1 | Test each value on creation vs default values.
        assert!(wkb.mode == WKB_DEFAULT_MODE);
        assert!(wkb.auto_repeat == WKB_DEFAULT_REPEAT);

    }

    /// Unit tests [super::WindowPointer] default values.
    ///
    /// # Verification(s)
    /// V1 | Test each value on creation vs default values.
    #[test]
    fn ut_window_pointer_default() {
        let wp: WindowPointer = WindowPointer::new();

        // V1 | Test each value on creation vs default values.
        assert!(wp.mode == WP_DEFAULT_MODE);
        assert!(wp.visible == WP_DEFAULT_VISIBILITY);
        assert!(wp.confined == WP_DEFAULT_CONFINED);
        assert!(wp.cursor == WP_DEFAULT_CURSOR);

    }

}