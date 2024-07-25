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


//! Events polled by a [WindowManager](crate::WindowManager).

use crate::{WindowHandle, WindowPosition, WindowSize};

/// [WindowEvent] with [WindowHandle] returned with [WindowManager::event()](crate::WindowManager::event()).
pub struct WindowManagerEvent {
    
    /// [WindowHandle] of [Window](crate::Window) which [Event] refer to.
    pub window : WindowHandle,

    /// [WindowEvent] that occurred.
    pub event : WindowEvent,

    /// Timestamp of event
    #[cfg(feature = "event_ts")]
    pub timestamp : std::time::SystemTime,

}

/// [Window](crate::Window) coordinate where the [WindowEvent] occurred.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WindowCoordinate {
    pub x : i32,
    pub y : i32,
}


/// Possible [WindowEvent] that can occur.
pub enum WindowEvent {

    /// Events that refer to keyboard presses
    Keyboard(WindowKeyboardEvent),

    /// Event that refer to the pointer / cursor device.
    Pointer(WindowPointerEvent),

    /// Happens when window is created.
    Created,

    /// Happens when Window is shown.
    Shown,

    /// Happens when Window is hidden.
    Hidden,

    /// Happens when Window is exposed/damaged, meaning part of drawing is lost and need to be redraw.
    /// Provides position (x, y) and size (width, height) of region exposed. 
    Exposed(WindowCoordinate, WindowSize),

    /// Happens when Window is moved. Provides (x,y) of new position.
    Moved(WindowPosition),

    /// Happens when Window is moved and resized. Provides (x,y) of new position and (height, width) of new size.
    MovedResized(WindowPosition, WindowSize),

    /// Happens when Window is Resized. Provides (height, width) of new size.
    Resized(WindowSize),

    /// Happens when Window is minimized.
    /// 
    /// # Known issue(s)
    /// * `(Linux only)` Won't trigger if window is maximized.
    Minimized,

    /// Happens when Window is maximized.
    Maximized,

    /// Happens when Window is set fullscreen.
    Fullscreen,

    /// Happens when Window is restored from minimized, maximized or fullscreen.
    Restored,

    /// Happens when pointer enter Window.
    PointerEnter,

    /// Happens when pointer leave Window.
    PointerLeave,

    /// Happens when Window gain focus.
    Focus,

    /// Happens when Window lose focus.
    Blur,

    /// Happens when a user press on a overriden close button.
    CloseButtonPressed,

    /// Happens when a user press on a overriden minimize button.
    MinimizeButtonPressed,

    /// Happens when a user press on a overriden maximize button.
    MaximizeButtonPressed,

    /// Happens when a window was closed.
    Closed,

    /// Happens when a child window closed
    ChildWindowClosed,

    /// Happens when a Modal subwindow showed
    ModalShowed,

    /// Happens when a Modal subwindow closed
    ModalClosed,

}

/// Keyboard keycode
pub type KeyCode = u32;

/// Possible [WindowKeyboardEvent] that can occur.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowKeyboardEvent {
    // Keyboard key down event of direct input mode. Provides keycode as u32.
    KeyDown(KeyCode),

    // Keyboard key down event of direct input mode. Provides keycode as u32.
    KeyUp(KeyCode),

    // KeyPress happens provides [Key] struct
    //KeyPress(Key),
}

/// Pointer button
pub type PointerButton = u32;

/// Pointer Acceleration as pair of x,y axis.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PointerAcceleration {
    pub x : i32,
    pub y : i32,
}

/// Possible [WindowPointerEvent] that can occur.
pub enum WindowPointerEvent {
    /// Pointer move event. Provides new (x, y) position. Only when in pointer mode.
    Move(WindowCoordinate),

    /// Pointer acceleration event.  Provides delta (x, y). Only when in acceleration mode.
    Acceleration(PointerAcceleration),

    /// Pointer button down event. Provides button number and cursor position (x,y).
    ButtonDown(PointerButton, WindowCoordinate),

    /// Pointer button up event. Provides button number and cursor position (x,y).
    ButtonUp(PointerButton, WindowCoordinate),
}