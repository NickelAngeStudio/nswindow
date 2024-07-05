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

use nscfg::match_cfg;

/// Default [WindowKeyboardMode].
const WKB_DEFAULT_MODE : WindowKeyboardMode = WindowKeyboardMode::Direct;

/// Default auto repeat.
const WKB_DEFAULT_REPEAT : bool = false;

/// [Window](crate::Window) keyboard properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowKeyboard {

    /// Linux [WindowKeyboard] abstraction for calls. Is set as [Option] since [WindowBuilder] can use it.
    #[cfg(target_os = "linux")]
    keyboard : Option<crate::linux::keyboard::LinuxKeyboard>,

    /// [KeyboardMode] of the keyboard. Use [KeyboardMode::DirectInput] by default.
    pub(crate) mode : WindowKeyboardMode,

    /// If enabled, keys are repeated when pressed down. Disabled by default.
    pub(crate) auto_repeat : bool,

}

impl WindowKeyboard {
    /// Create new instance of keyboard property with auto repeat to false.
    pub fn new() -> WindowKeyboard {
        WindowKeyboard { keyboard : None,  mode : WKB_DEFAULT_MODE, auto_repeat : WKB_DEFAULT_REPEAT}
    }

    /// Returns the [WindowKeyboardMode] of the [Window](crate::Window).
    pub fn mode(&self) -> WindowKeyboardMode {
        self.mode
    }

    /// Set the [WindowKeyboardMode] of the [Window](crate::Window). Does nothing if already the same mode.
    pub fn set_mode(&mut self, mode : WindowKeyboardMode) {
        
        if self.mode != mode {
            self.mode = mode;

            match_cfg! {
                linux => {
                    match self.keyboard {
                        Some(mut wkb) => wkb.set_mode(mode),
                        None => {},
                    }
                },
                _ => {};
            }
        }
    }

    /// Returns true if keyboard will repeat a key that is pressed down.
    pub fn repeat(&self) -> bool {
        self.auto_repeat
    }

    /// Enable keyboard press auto repeat. Does nothing if already is.
    pub fn enable_repeat(&mut self) {
        if !self.auto_repeat {
            self.auto_repeat = true;

            match_cfg! {
                linux => {
                    match self.keyboard {
                        Some(mut wkb) => wkb.enable_repeat(),
                        None => {},
                    }
                },
                _ => {};
            }

        }
    }

    /// Disable keyboard press auto repeat. Does nothing if already is.
    pub fn disable_repeat(&mut self) {
        if self.auto_repeat {
            self.auto_repeat = false;

            match_cfg! {
                linux => {
                    match self.keyboard {
                        Some(mut wkb) => wkb.disable_repeat(),
                        None => {},
                    }
                },
                _ => {};
            }

        }
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
    use crate::keyboard::{WindowKeyboard, WindowKeyboardMode, WKB_DEFAULT_MODE, WKB_DEFAULT_REPEAT};
    

    /// Unit tests [super::WindowKeyboard] default values.
    ///
    /// # Verification(s)
    /// V1 | Test each value on creation vs default values.
    #[test]
    fn ut_window_keyboard_default() {
        let wkb = WindowKeyboard::new();

        // V1 | Test each value on creation vs default values.
        assert!(wkb.keyboard == None);
        assert!(wkb.mode == WKB_DEFAULT_MODE);
        assert!(wkb.auto_repeat == WKB_DEFAULT_REPEAT);

    }

    /// Unit tests [super::WindowKeyboard] value update.
    ///
    /// # Verification(s)
    /// V1 | Modify and test each value.
    #[test]
    fn ut_window_keyboard_update() {
        let mut wkb = WindowKeyboard::new();

        // V1 | Modify and test each value.
        
        // Mode
        const MODE : WindowKeyboardMode = WindowKeyboardMode::Text;

        assert!(wkb.mode == WKB_DEFAULT_MODE);
        wkb.mode = MODE;
        assert!(wkb.mode == MODE);

        // Auto repeat
        assert!(wkb.auto_repeat == WKB_DEFAULT_REPEAT);
        wkb.enable_repeat();
        assert!(wkb.auto_repeat == !WKB_DEFAULT_REPEAT);
        wkb.disable_repeat();
        assert!(wkb.auto_repeat == WKB_DEFAULT_REPEAT);

    }

}