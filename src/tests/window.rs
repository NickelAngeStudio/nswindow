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

// Window unit tests



use std::rc::Rc;

use crate::{display::{tests::create_displays, Displays}, Window, WindowError, WindowPosition, WindowRelativePosition, WindowSize};


fn assert_position(position : &WindowPosition, expected_x : i32, expected_y : i32) {
    assert!(position.x == expected_x && position.y == expected_y, "Expected ({},{}), got ({},{})", expected_x, expected_y, position.x, position.y);
}

/// Window::get_window_desktop_position() -> WindowRelativePosition::Desktop
#[test]
fn window_gwdp_desktop() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };

    let position = WindowRelativePosition::Desktop(WindowPosition { x: 50, y: 125 });
    match Window::get_window_desktop_position(size, displays.clone(), position, None, true) {
        Ok(position) => assert_position(&position, 50, 125),
        Err(_) => assert!(false),
    }
}

/// Window::get_window_desktop_position() -> WindowRelativePosition::DesktopCenter
#[test]
fn window_gwdp_desktop_center() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };

    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DesktopCenter, None, true) {
        Ok(position) => assert_position(&position, 560, 780),
        Err(_) => assert!(false),
    }
}

/// Window::get_window_desktop_position() -> WindowRelativePosition::Display
#[test]
fn window_gwdp_display() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };

    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(0, WindowPosition::new(100, 200)), None, true) {
        Ok(position) => assert_position(&position, 100, 1280),
        Err(_) => assert!(false),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(1, WindowPosition::new(100, 200)), None, true) {
        Ok(position) => assert_position(&position, 100,200),
        Err(_) => assert!(false),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(2, WindowPosition::new(100, 200)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::DisplayInvalidHandle)"),
        Err(err) => assert!(err == WindowError::DisplayInvalidHandle),
    }
}

/// Window::get_window_desktop_position() -> WindowRelativePosition::DisplayCenter
#[test]
fn window_gwdp_display_center() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };
    
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DisplayCenter(0), None, true) {
        Ok(position) => assert_position(&position, 560, 1320),
        Err(_) => assert!(false),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DisplayCenter(1), None, true) {
        Ok(position) => assert_position(&position, 560, 240),
        Err(_) => assert!(false),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DisplayCenter(2), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::DisplayInvalidHandle)"),
        Err(err) => assert!(err == WindowError::DisplayInvalidHandle),
    }
}

/// Window::get_window_desktop_position() -> WindowRelativePosition::Parent
#[test]
fn window_gwdp_parent() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };
    let pps : Option<(WindowPosition, WindowSize)> = Some((WindowPosition { x: 400, y: 150 }, WindowSize { width: 1024, height: 768 }));
    
    // Without parent fallback to desktop
    let position = WindowRelativePosition::Parent(WindowPosition { x: 50, y: 125 });
    match Window::get_window_desktop_position(size, displays.clone(), position, None, true) {
        Ok(position) => assert_position(&position, 50, 125),
        Err(_) => assert!(false),
    }

    // With parent
    let position = WindowRelativePosition::Parent(WindowPosition { x: 50, y: 125 });
    match Window::get_window_desktop_position(size, displays.clone(), position, pps, true) {
        Ok(position) => assert_position(&position, 450, 275),
        Err(_) => assert!(false),
    }

}

/// Window::get_window_desktop_position() -> WindowRelativePosition::ParentCenter
#[test]
fn window_gwdp_parent_center() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };
    let pps : Option<(WindowPosition, WindowSize)> = Some((WindowPosition { x: 400, y: 150 }, WindowSize { width: 1024, height: 768 }));

    // Without parent fallback to desktop
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::ParentCenter, None, true) {
        Ok(position) => assert_position(&position, 560, 780),
        Err(_) => assert!(false),
    }
    
    // With parent
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::ParentCenter, pps, true) {
        Ok(position) => assert_position(&position, 512, 234),
        Err(_) => assert!(false),
    }
}

/// Window::get_window_desktop_position() -> WindowRelativePosition::Primary
#[test]
fn window_gwdp_primary() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };
    
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Primary(WindowPosition::new(255, 311)), None, true) {
        Ok(position) => assert_position(&position, 255, 1391),
        Err(_) => assert!(false),
    }
}

/// Window::get_window_desktop_position() -> WindowRelativePosition::PrimaryCenter
#[test]
fn window_gwdp_primary_center() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };
    
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::PrimaryCenter, None, true) {
        Ok(position) => assert_position(&position, 560, 1320),
        Err(_) => assert!(false),
    }
}




/// Unit tests get_window_desktop_position OOB
///
/// # Verification(s)
/// V1 | WindowError::WindowRelativePositionOOB for each type with position.
/// V2 | WindowError::WindowRelativePositionOOB unchecked for each type with position.
#[test]
fn window_gwdp_oob() {
    let displays : Rc<Displays> = Rc::new(create_displays(true));
    let size = WindowSize { width: 800, height: 600 };
    let pps : Option<(WindowPosition, WindowSize)> = Some((WindowPosition { x: 400, y: 150 }, WindowSize { width: 1024, height: 768 }));

    // V1 | WindowError::WindowRelativePositionOOB for each type with position.
    // Desktop
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(i32::MIN / 2, 0)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(i32::MAX / 2, 0)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(0, i32::MIN / 2)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(0, i32::MAX / 2)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // Display
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(0, WindowPosition::new(i32::MIN / 2, 0)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(1, WindowPosition::new(i32::MAX / 2, 0)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(0, WindowPosition::new(0, i32::MIN / 2)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(1, WindowPosition::new(0, i32::MAX / 2)), None, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // Parent
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(i32::MIN / 2, 0)), pps, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(i32::MAX / 2, 0)), pps, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(0, i32::MIN / 2)), pps, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(0, i32::MAX / 2)), pps, true) {
        Ok(_) => assert!(false, "Expected Err(WindowError::WindowRelativePositionOOB)"),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // V2 | WindowError::WindowRelativePositionOOB unchecked for each type with position.
    // Desktop
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(i32::MIN / 2, 0)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(i32::MAX / 2, 0)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(0, i32::MIN / 2)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition::new(0, i32::MAX / 2)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }

    // Display
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(0, WindowPosition::new(i32::MIN / 2, 0)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(1, WindowPosition::new(i32::MAX / 2, 0)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(0, WindowPosition::new(0, i32::MIN / 2)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(1, WindowPosition::new(0, i32::MAX / 2)), None, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }

    // Parent
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(i32::MIN / 2, 0)), pps, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(i32::MAX / 2, 0)), pps, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(0, i32::MIN / 2)), pps, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Parent(WindowPosition::new(0, i32::MAX / 2)), pps, false) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false, "Should not be OOB"),
    }

}

/// Unit tests get_window_desktop_position limit
///
/// # Verification(s)
/// V1 | Test each type with no displays or desktop
#[test]
fn window_gwdp_limit() {
    let displays : Rc<Displays> = Rc::new(Displays { list: Vec::new(), desktop: None });
    let size = WindowSize { width: 800, height: 600 };
    let pps : Option<(WindowPosition, WindowSize)> = Some((WindowPosition { x: 400, y: 150 }, WindowSize { width: 1024, height: 768 }));

    // WindowRelativePosition::Desktop
    let position = WindowRelativePosition::Desktop(WindowPosition { x: 50, y: 125 });
    match Window::get_window_desktop_position(size, displays.clone(), position, None, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }
    // unchecked
    let position = WindowRelativePosition::Desktop(WindowPosition { x: 50, y: 125 });
    match Window::get_window_desktop_position(size, displays.clone(), position, None, false) {
        Ok(position) => assert_position(&position, 50, 125),
        Err(_) => assert!(false),
    }

    // WindowRelativePosition::DesktopCenter
    // checked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DesktopCenter, None, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // unchecked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DesktopCenter, None, false) {
        Ok(position) => assert_position(&position, 0, 0),
        Err(_) => assert!(false),
    }

    // WindowRelativePosition::Display
    // Checked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(0, WindowPosition::new(100, 200)), None, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::DisplayInvalidHandle),
    }

    // UnChecked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(0, WindowPosition::new(100, 200)), None, false) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::DisplayInvalidHandle),
    }

    // WindowRelativePosition::DisplayCenter
    // Checked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DisplayCenter(0), None, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::DisplayInvalidHandle),
    }

    // UnChecked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DisplayCenter(0), None, false) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::DisplayInvalidHandle),
    }

    // WindowRelativePosition::Parent

    // Checked
    let position = WindowRelativePosition::Parent(WindowPosition { x: 50, y: 125 });
    match Window::get_window_desktop_position(size, displays.clone(), position, pps, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // UnChecked
    let position = WindowRelativePosition::Parent(WindowPosition { x: 50, y: 125 });
    match Window::get_window_desktop_position(size, displays.clone(), position, pps, false) {
        Ok(position) => assert_position(&position, 450, 275),
        Err(_) => assert!(false),
    }

    // WindowRelativePosition::ParentCenter
    // Checked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::ParentCenter, pps, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // UnChecked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::ParentCenter, pps, false) {
        Ok(position) => assert_position(&position, 512, 234),
        Err(_) => assert!(false),
    }

    // WindowRelativePosition::Primary
    // Checked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Primary(WindowPosition::new(255, 311)), None, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // UnChecked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Primary(WindowPosition::new(255, 311)), None, false) {
        Ok(position) => assert!(position.x == 255 && position.y == 311, " Result={:?}", position),
        Err(_) => assert!(false),
    }

    // WindowRelativePosition::PrimaryCenter
    // Checked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::PrimaryCenter, None, true) {
        Ok(_) => assert!(false),
        Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
    }

    // Unchecked
    match Window::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::PrimaryCenter, None, false) {
        Ok(position) => assert!(position.x == 0 && position.y == 0, " Result={:?}", position),
        Err(_) => assert!(false),
    }
}