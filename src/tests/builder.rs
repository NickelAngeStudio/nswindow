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

use std::io::Read;

use crate::{builder::{WB_DEFAULT_FSMODE, WB_DEFAULT_MAXIMIZED, WB_DEFAULT_MAX_SIZE, WB_DEFAULT_MINIMIZED, WB_DEFAULT_MIN_SIZE, WB_DEFAULT_PARENT, WB_DEFAULT_POSITION, WB_DEFAULT_SIZE, WB_DEFAULT_TASKBAR, WB_DEFAULT_TITLE, WB_DEFAULT_VISIBLE}, frame::{WindowFrame, WindowFrameButtonMode}, keyboard::{WindowKeyboard, WindowKeyboardMode}, pointer::{WindowCursor, WindowPointer, WindowPointerMode}, WindowBuilder, WindowFullScreenMode, WindowHandle, WindowPosition, WindowRelativePosition, WindowSize};

struct Icon {
    index : usize,
    content : [u8;10],
}

impl Read for Icon {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {

        let written : usize = if buf.len() > self.content.len() - self.index {
            self.content.len() - self.index
        } else {
            buf.len()
        };

        buf[0..written].copy_from_slice(&self.content[self.index..self.index+written]);
        self.index += written;

        Ok(written)
    }
}

/// Test default values of WindowBuilder
fn test_defaults(wb : &WindowBuilder) {
    let wkb = WindowKeyboard::new();
    let wp = WindowPointer::new();
    let wf = WindowFrame::new();
    

    assert!(wb.icon == None);
    assert!(wb.title == WB_DEFAULT_TITLE.to_string());
    assert!(wb.min_size == WB_DEFAULT_MIN_SIZE);
    assert!(wb.max_size == WB_DEFAULT_MAX_SIZE);
    assert!(wb.size == WB_DEFAULT_SIZE);
    assert!(wb.position == WB_DEFAULT_POSITION);

    nscfg::match_cfg! {
        !single_opt:ft => {
            let sub = crate::sub::SubWindow::new();
            assert!(wb.sub == sub);
            assert!(wb.parent == WB_DEFAULT_PARENT);
        }, 
        _ => {}
    }
    
    assert!(wb.fsmode == WB_DEFAULT_FSMODE);
    assert!(wb.minimized == WB_DEFAULT_MINIMIZED);
    assert!(wb.maximized == WB_DEFAULT_MAXIMIZED);
    assert!(wb.visible == WB_DEFAULT_VISIBLE);
    assert!(wb.taskbar == WB_DEFAULT_TASKBAR);
    assert!(wb.keyboard == wkb);
    assert!(wb.pointer == wp);
    assert!(wb.frame == wf);

}

/// Unit tests [super::WindowBuilder] default values.
///
/// # Verification(s)
/// V1 | Test each value on creation vs default values. (except pointer and keyboard that are tested on their own)
/// V2 | Test each pointer values.
/// V3 | Test each keyboard values.
#[test]
fn ut_window_builder_default() {
    let wb = WindowBuilder::new();

    // V1 | Test each value on creation vs default values. (except pointer and keyboard that are tested on their own)
    test_defaults(&wb);

}

/// Unit tests [super::WindowBuilder] modification of values.
///
/// # Verification(s)
/// V1 | Allocate value to each parameter.
/// V2 | Compare values VS Allocated values.
/// V3 | Reset and test default values.
#[test]
fn ut_window_builder_modifications() {
    const TITLE : &str = "TEST TITLE";
    const MIN_SIZE : WindowSize = WindowSize { width: 111, height: 222 };
    const MAX_SIZE : WindowSize = WindowSize { width: 5555, height: 6666 };
    const SIZE : WindowSize = WindowSize { width: 123, height: 456 };
    const POSITION : WindowRelativePosition = WindowRelativePosition::Primary(WindowPosition { x: 987, y: 345 });
    const FSMODE : Option<WindowFullScreenMode> = Some(WindowFullScreenMode::Primary);

    const WKB_MODE : WindowKeyboardMode = WindowKeyboardMode::Text;
    const WKB_REPEAT : bool = true;

    const WP_MODE : WindowPointerMode = WindowPointerMode::Acceleration;
    const WP_CONFINED : bool = true;
    const WP_VISIBLE : bool = false;
    const WP_CURSOR : WindowCursor = WindowCursor::Move;

    let mut icon = Icon{ index:0, content: [0,1,2,3,4,5,6,7,8,9] };

    let mut wkb = WindowKeyboard::new();
    wkb.mode = WKB_MODE;
    wkb.auto_repeat = WKB_REPEAT;

    let mut wp = WindowPointer::new();
    wp.mode = WP_MODE;
    wp.confined = WP_CONFINED;
    wp.visible = WP_VISIBLE;
    wp.cursor = WP_CURSOR;

    let mut wf = WindowFrame::new();
    wf.visible = false;
    wf.resizable = false;
    wf.min_button = WindowFrameButtonMode::Disable;
    wf.max_button = WindowFrameButtonMode::Hidden;
    wf.close_button = WindowFrameButtonMode::Overriden;    

    // V1 | Allocate value to each parameter.
    let mut wb = WindowBuilder::new();
            wb.title(TITLE)
            .icon(Some(&mut icon))
            .size_min(MIN_SIZE)
            .size_max(MAX_SIZE)
            .size(SIZE)
            .position(POSITION)
            .pointer(wp)
            .keyboard(wkb)
            .frame(wf)
            .fullscreen(FSMODE)
            .minimize()
            .maximize()
            .hide()
            .taskbar(false);

    
    // V2 | Compare values VS Allocated values.
    let mut wkb = WindowKeyboard::new();
    wkb.mode = WKB_MODE;
    wkb.auto_repeat = WKB_REPEAT;

    let mut wp = WindowPointer::new();
    wp.mode = WP_MODE;
    wp.confined = WP_CONFINED;
    wp.visible = WP_VISIBLE;
    wp.cursor = WP_CURSOR;

    let mut wf = WindowFrame::new();
    wf.visible = false;
    wf.resizable = false;
    wf.min_button = WindowFrameButtonMode::Disable;
    wf.max_button = WindowFrameButtonMode::Hidden;
    wf.close_button = WindowFrameButtonMode::Overriden;
    
    assert!(wb.icon.is_some());
    if let Some(icon) = &wb.icon {
        assert!(icon.len() == 10);
    }
    
    assert!(wb.title == TITLE.to_string());
    assert!(wb.min_size == MIN_SIZE);
    assert!(wb.max_size == MAX_SIZE);
    assert!(wb.size == SIZE);
    assert!(wb.position == POSITION);
    assert!(wb.fsmode == FSMODE);
    assert!(wb.minimized == true);
    assert!(wb.maximized == true);
    assert!(wb.visible == false);
    assert!(wb.keyboard == wkb);
    assert!(wb.pointer == wp);
    assert!(wb.frame == wf);
    assert!(!wb.taskbar);


    nscfg::match_cfg! {
        !single_opt:ft => {    // For parent and sub
            const PARENTHANDLE : usize = 1;
            const PARENT : Option<WindowHandle> = Some(&PARENTHANDLE);

            let mut sub = crate::sub::SubWindow::new();
            sub.block_parent_until_closed = true;
            sub.minimize_with_parent = false;
            sub.restore_with_parent = false;
            sub.show_with_parent = true; 
            sub.hide_with_parent = false;
            sub.close_with_parent = false;

            wb.parent(PARENT);
            wb.subwindow(sub.clone());

            assert!(wb.parent == PARENT);
            assert!(wb.sub == sub);
        },
        _ => {}
    }

    // V3 | Reset and test default values.
    wb.reset();
    test_defaults(&wb);

}
