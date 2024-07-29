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
