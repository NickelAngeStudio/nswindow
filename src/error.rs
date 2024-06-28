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

/// Enumeration of possible nswindow errors.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowError {

    // **********
    // * WINDOW *
    // **********

    /// Happens when using an invalid [WindowHandle]
    InvalidWindowHandle = 1,

    /// Happens when no system window manager is available.
    NoWindowManager,

    /// Happens when a system window manager is not supported
    WindowManagerNotSupported,

    /// Happens when a given [WindowRelativePosition] is out of bound.
    WindowRelativePositionOOB,

    /// Happens when a given [WindowSize] is out of bound.
    WindowSizeOOB,

    /// Happens when trying to make a [Window] it's own parent
    WindowParentSelf,

    /// Happens when parent [WindowHandle] result in a loop.
    /// Example : W1 < W2 < W3 < W1
    WindowParentLoop,

    // ***********
    // * DISPLAY *
    // ***********

    /// Happens when screens informations cannot be fetched
    DisplayInformationError,




}