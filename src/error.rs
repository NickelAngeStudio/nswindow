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

/// Errors that can happens within `nswindow`.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowError {

    // **********
    // * WINDOW *
    // **********

    /// Happens when using an invalid [WindowHandle](crate::WindowHandle).
    InvalidWindowHandle = 1,

    /// Happens when no system window manager is available.
    NoWindowManager,

    /// Happens when a system window manager is not supported
    WindowManagerNotSupported,

    /// Happens when a given [WindowRelativePosition](crate::WindowRelativePosition) is out of bound.
    WindowRelativePositionOOB,

    /// Happens when a given [WindowSize](crate::WindowSize) is out of bound.
    WindowSizeOOB,

    /// Happen when minimum [WindowSize](crate::WindowSize) is greater than maximum size.
    WindowMinSizeBiggerThanMax,

    

    /// Happens when trying to make a [Window](crate::Window) it's own parent.
    WindowParentSelf,

    /// Happens when parent [WindowHandle](crate::WindowHandle) result in a loop.
    /// 
    /// # Example
    /// Trying to set Window1 as parent of Window2 while Window2 parent of Window1.
    WindowParentLoop,

    // ***********
    // * DISPLAY *
    // ***********

    /// Happens when display information cannot be fetched.
    DisplayInformationError,




}