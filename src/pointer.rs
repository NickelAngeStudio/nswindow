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

//! Pointer / Mouse and cursor properties of [Window](crate::Window). 

use nscfg::match_cfg;

use crate::WindowError;

/// Default [WindowPointer] mode.
const WP_DEFAULT_MODE : WindowPointerMode = WindowPointerMode::Cursor;

/// Default [WindowPointer] visibility.
const WP_DEFAULT_VISIBILITY : bool = true;

/// Default [WindowPointer] confined.
const WP_DEFAULT_CONFINED : bool = false;

/// Default [WindowPointer] cursor.
const WP_DEFAULT_CURSOR : WindowCursor = WindowCursor::Normal;

/// The position of the [Window](crate::Window) pointer as pair of x,y
pub struct WindowPointerPosition {
    pub x : u32,
    pub y : u32
}


/// [Window](super::window::Window) pointer properties such as mode, position, etc.
#[derive(Debug, PartialEq)]
pub struct WindowPointer {

    /// Linux [WindowPointer] abstraction for calls. Is set as [Option] since [WindowBuilder] can use it.
    #[cfg(target_os = "linux")]
    pointer : Option<crate::linux::pointer::LinuxPointer>,

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
            pointer : None,
            mode: WP_DEFAULT_MODE, 
            visible: WP_DEFAULT_VISIBILITY, 
            confined: WP_DEFAULT_CONFINED,
            cursor: WP_DEFAULT_CURSOR,
        }
    }

    /// Returns the pointer [WindowPointerMode].
    pub fn mode(&self) -> WindowPointerMode {
        self.mode
    }

    /// Set the pointer [WindowPointerMode]. Does nothing if same mode as before.
    pub fn set_mode(&mut self, mode : WindowPointerMode) {

        if self.mode != mode {
            self.mode = mode;

            match_cfg! {
                linux => {
                    match self.pointer {
                        Some(ref mut lp) => lp.set_mode(mode),
                        None => {}, // Do nothing
                    }
                },
                _ => {},
            }
        }
        
    }

    /// Returns true if pointer is visible, false otherwise.
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Make the pointer visible. Does nothing if already is.
    pub fn show(&mut self) {

        if !self.visible {
            self.visible = true;

            match_cfg! {
                linux => {
                    match self.pointer {
                        Some(ref mut lp) => lp.show(),
                        None => {}, // Do nothing
                    }
                },
                _ => {},
            }
        }

    }

    /// Make the pointer invisible. Does nothing if already is.
    pub fn hide(&mut self) {

        if self.visible {
            self.visible = false;

            match_cfg! {
                linux => {
                    match self.pointer {
                        Some(ref mut lp) => lp.hide(),
                        None => {}, // Do nothing
                    }
                },
                _ => {},
            }
        }
        

    }

    /// Returns true if pointer is confined within [Window](crate::Window) boundaries, false otherwise.
    pub fn confined(&self) -> bool {
        self.confined
    }

    /// Confine the pointer within [Window](crate::Window) boundaries. Does nothing if already is.
    pub fn confine(&mut self) {

        if !self.confined {
            self.confined = true;

            match_cfg! {
                linux => {
                    match self.pointer {
                        Some(ref mut lp) => lp.confine(),
                        None => {}, // Do nothing
                    }
                },
                _ => {},
            }
        }

    }

    /// Release the pointer, making it escape [Window](crate::Window) boundaries. Does nothing if already is.
    pub fn release(&mut self) {

        if self.confined {
            self.confined = false;

            match_cfg! {
                linux => {
                    match self.pointer {
                        Some(ref mut lp) => lp.release(),
                        None => {}, // Do nothing
                    }
                },
                _ => {},
            }
        }
        

    }


    /// Returns the current [WindowCursor] glyph showed.
    pub fn cursor(&self) -> WindowCursor {
        self.cursor
    }

    /// Set the current [WindowCursor] glyph showed on [Window]. Does nothing if setting same cursor.
    pub fn set_cursor(&mut self, cursor : WindowCursor) {
        
        if self.cursor != cursor {
            self.cursor = cursor;

            match_cfg! {
                linux => {
                    match self.pointer {
                        Some(ref mut lp) => lp.set_cursor(cursor),
                        None => {}, // Do nothing
                    }
                },
                _ => {},
            }
        }
    }

    /// Returns the [WindowPointer] position on the window.
    /// 
    /// Returns [None] if pointer out of [Window](crate::Window) bounds. 
    pub fn position(&self) -> Option<WindowPointerPosition> {

        match_cfg! {
            linux => {
                match &self.pointer {
                    Some(lp) => lp.position(),
                    None => None,
                }
            },
            _ => {},
        }

    }

    /// Set the [WindowPointer] position within the [Window](crate::Window).
    /// 
    /// Returns Ok(true) if cursor was moved, Ok(false) if the cursor didn't move or 
    /// ['WindowError::WindowpointerOOB'] when trying to set the position while cursor is out of bounds.
    pub fn set_position(&mut self, position : WindowPointerPosition) -> Result<bool, WindowError> {

        match_cfg! {
            linux => {
                match self.pointer {
                    Some(ref mut lp) => lp.set_position(position),
                    None => Ok(false),
                }
            },
            _ => {},
        }

    }




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




/*************
* UNIT TESTS * 
*************/
#[cfg(test)]
mod tests{
    use nscfg::match_cfg;

    use crate::pointer::{WindowCursor, WindowPointer, WindowPointerMode, WP_DEFAULT_CONFINED, WP_DEFAULT_CURSOR, WP_DEFAULT_MODE, WP_DEFAULT_VISIBILITY};


    /// Unit tests [super::WindowPointer] default values.
    ///
    /// # Verification(s)
    /// V1 | Test each value on creation vs default values.
    #[test]
    fn ut_window_pointer_default() {
        let wp: WindowPointer = WindowPointer::new();

        // V1 | Test each value on creation vs default values.
        match_cfg! {
            linux => {
                assert!(wp.pointer == None);
            }, 
            _ => {},
        }

        assert!(wp.mode == WP_DEFAULT_MODE);
        assert!(wp.visible == WP_DEFAULT_VISIBILITY);
        assert!(wp.confined == WP_DEFAULT_CONFINED);
        assert!(wp.cursor == WP_DEFAULT_CURSOR);
        assert!(wp.position().is_none());

    }

    /// Unit tests [super::WindowPointer] values modifications.
    ///
    /// # Verification(s)
    /// V1 | Modifiy each value and compare.
    #[test]
    fn ut_window_pointer_update() {
        let mut wp: WindowPointer = WindowPointer::new();

        const MODE : WindowPointerMode = WindowPointerMode::Acceleration;
        const CURSOR : WindowCursor = WindowCursor::Select;

        // V1 | Modifiy each value and compare.

        // Mode
        assert!(wp.mode == WP_DEFAULT_MODE);
        wp.set_mode(MODE);
        assert!(wp.mode == MODE);

        // Visibility
        assert!(wp.visible());
        wp.hide();
        assert!(!wp.visible());
        wp.show();
        assert!(wp.visible());

        // Confine and release
        assert!(!wp.confined());
        wp.confine();
        assert!(wp.confined());
        wp.release();
        assert!(!wp.confined());
        
        // Cursor
        assert!(wp.cursor == WP_DEFAULT_CURSOR);
        wp.set_cursor(CURSOR);
        assert!(wp.cursor == CURSOR);
        
    }

}