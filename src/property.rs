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

/// Enumeration of possible keyboard mode for input.
/// 
/// fsdssf
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyboardMode {
    /// Direct input is faster and more suitable for games. Provides [EventKeyboard::KeyUp](super::event::keyboard::EventKeyboard)
    /// and [EventKeyboard::KeyDown](super::event::keyboard::EventKeyboard).
    DirectInput,

    /// Text mode is slower since it provides more information for text entry. Provides [EventKeyboard::KeyPress](super::event::keyboard::EventKeyboard).
    TextInput,
}

/// Contains keyboard properties.
#[derive(Debug, Clone, Copy)]
pub struct KeyboardProperty {

    /// [KeyboardMode] of the keyboard. Use [KeyboardMode::DirectInput] by default.
    pub mode:KeyboardMode,

    /// If enabled, keys are repeated when pressed down. Disabled by default.
    pub auto_repeat : bool,

}

impl KeyboardProperty {
    /// Create new instance of keyboard property with auto repeat to false.
    pub(crate) fn new() -> KeyboardProperty {
        KeyboardProperty { mode : KeyboardMode::DirectInput, auto_repeat: false }
    }
}

/// Enumeration of possible [Window](super::window::Window) pointer mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerMode {
    /// [EventMouse](super::event::EventMouse) events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces interactions.
    Cursor,

    /// [EventMouse](super::event::EventMouse) events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}


/// [Window](super::window::Window) cursor properties such as mode, position, etc.
#[derive(Debug, Clone, Copy)]
pub struct PointerProperty {
    /// [PointerMode] used for [EventMouse](super::event::EventMouse) events.
    pub mode : PointerMode,

    /// Indicate if cursor is visible or hidden.
    pub visible : bool,

    /// Indicate if cursor is confined to the window boundaries or not.
    pub confined : bool, 
}


impl PointerProperty {
    /// Create a new [PointerProperty] with default values.
    pub(crate) fn new() -> PointerProperty {
        PointerProperty{ 
            mode: PointerMode::Cursor, 
            visible: true, 
            confined: false,
        }
    }
}

