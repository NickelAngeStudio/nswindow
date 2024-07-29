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

use crate::frame::{WindowFrameButtonMode, WF_DEFAULT_BUTTON, WF_DEFAULT_RESIZABLE};

use super::WindowFrame;

pub fn test_window_frame_default(wf : &WindowFrame) {

    assert!(wf.frame == None);
    assert!(wf.visible == WF_DEFAULT_RESIZABLE);
    assert!(wf.resizable == WF_DEFAULT_RESIZABLE);
    assert!(wf.min_button == WF_DEFAULT_BUTTON);
    assert!(wf.max_button == WF_DEFAULT_BUTTON);
    assert!(wf.close_button == WF_DEFAULT_BUTTON);

    assert!(wf.visible() == WF_DEFAULT_RESIZABLE);
    assert!(wf.resizable() == WF_DEFAULT_RESIZABLE);
    assert!(wf.button_min() == WF_DEFAULT_BUTTON);
    assert!(wf.button_max() == WF_DEFAULT_BUTTON);
    assert!(wf.button_close() == WF_DEFAULT_BUTTON);

}


/// Unit tests [super::WindowFrame] default values.
///
/// # Verification(s)
/// V1 | Test each value on creation vs default values.
#[test]
fn ut_window_frame_default() {
    let wf = WindowFrame::new();
    test_window_frame_default(&wf);
}

/// Unit tests [super::WindowFrame] values update.
///
/// # Verification(s)
/// V1 | Modify each value and compare.
/// V2 | Reset and verify default values
#[test]
fn ut_window_frame_update() {
    let mut wf = WindowFrame::new();


    const BTNMIN : WindowFrameButtonMode = WindowFrameButtonMode::Disable;
    const BTNMAX : WindowFrameButtonMode = WindowFrameButtonMode::Hidden;
    const BTNCLOSE : WindowFrameButtonMode = WindowFrameButtonMode::Overriden;


    // V1 | Modify each value and compare.

    // Visibility
    assert!(wf.visible);
    wf.hide();
    assert!(!wf.visible && !wf.visible());
    wf.show();
    assert!(wf.visible && wf.visible);

    // Resizable
    assert!(wf.resizable);
    wf.lock();
    assert!(!wf.resizable && !wf.resizable());
    wf.unlock();
    assert!(wf.resizable && wf.resizable());

    // Button min
    wf.set_button_min(BTNMIN);
    assert!(wf.min_button == BTNMIN && wf.button_min() == BTNMIN);

    // Button max
    wf.set_button_max(BTNMAX);
    assert!(wf.max_button == BTNMAX && wf.button_max() == BTNMAX);

    // Button close
    wf.set_button_close(BTNCLOSE);
    assert!(wf.close_button == BTNCLOSE && wf.button_close() == BTNCLOSE);

    // Make sure all buttons are different
    assert!(wf.min_button != wf.max_button && wf.min_button != wf.close_button && wf.min_button != wf.close_button && wf.max_button != wf.close_button);

    // V2 | Reset and verify default values
    wf.reset();
    test_window_frame_default(&wf);

}