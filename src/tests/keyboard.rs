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