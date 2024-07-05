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

//! Keyboard properties of [Window](crate::Window).

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


/// Possible [Window](crate::Window) keyboard mode for input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowKeyboardMode {
    /// Direct mode is faster and more suitable for games. Provides [EventKeyboard::KeyUp](super::event::keyboard::EventKeyboard)
    /// and [EventKeyboard::KeyDown](super::event::keyboard::EventKeyboard).
    Direct,

    /// Text mode is slower since it provides more information for text entry. Provides [EventKeyboard::KeyPress](super::event::keyboard::EventKeyboard).
    Text,
}




/*************
* UNIT TESTS * 
*************/
#[cfg(test)]
mod tests{
    use crate::{keyboard::{WKB_DEFAULT_MODE, WKB_DEFAULT_REPEAT, WindowKeyboard} };
    

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

}