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

use crate::{Window, WindowCursorMode, WindowError, WindowFullScreenMode, WindowHandle, WindowKeyboardMode, WindowManager, WindowRelativePosition, WindowSize};



/// [WindowBuilder] used to create new [Window] or recreate an actual one.
pub struct WindowBuilder {

    title : String,

    size : WindowSize,

    position : WindowRelativePosition,

    parent : WindowHandle,
    
    cursor_mode : WindowCursorMode,

    cursor_visible : bool,

    cursor_confined : bool, 

    keyboard_mode : WindowKeyboardMode,

    keyboard_autorepeat : bool,
}

impl WindowBuilder {

    /// Create a new [WindowBuilder] instance.
    pub fn new() -> WindowBuilder {
        todo!()
    }

    /// [Window] cursor mode.
    pub fn cursor_mode(&mut self, mode : WindowCursorMode) -> &mut Self {
        todo!()
    }

    /// Toggle [Window] cursor visibility.
    pub fn cursor_visible(&mut self, visible : bool) -> &mut Self {
        todo!()
    }

    /// Toggle [Window] cursor confined within boundaries.
    pub fn cursor_confined(&mut self, confined : bool) -> &mut Self {
        todo!()
    }

    /// [Window] keyboard mode.
    pub fn keyboard_mode(&mut self, mode : WindowKeyboardMode) -> &mut Self {
        todo!()
    }

    /// Toggle [Window] keyboard auto repeat. Default : false
    pub fn keyboard_autorepeat(&mut self, autorepeat : bool) -> &mut Self {
        todo!()
    }


    /// [Window] title of the new [Window].
    pub fn title(&mut self, title : &str) -> &mut Self {
        todo!()
    }

    /// [WindowHandle] of the parent of the new [Window].
    /// 
    /// [WindowHandle] must be valid else building returns  [WindowError::InvalidWindowHandle].
    /// [WindowHandle] can't be the [Window] itself else building returns [WindowError::WindowParentSelf].
    /// [WindowHandle] can't create a parent loop else building returns [WindowError::WindowParentLoop].
    pub fn parent(&mut self) -> &mut Self {
        todo!()
    }

    /// [WindowSize] of the new window.
    /// 
    /// Size must be between [Desktop::min] and [Desktop::max] or
    /// building the [Window] will return [WindowError::WindowSizeOOB].
    pub fn size(&mut self, size : WindowSize) -> &mut Self {
        todo!()
    }

    /// [WindowRelativePosition] of the window to be build. 
    /// 
    /// Position must not be out of desktop bound or  building the [Window] will return [WindowError::WindowRelativePositionOOB].
    pub fn position(&mut self, position : WindowRelativePosition) -> &mut Self {
        todo!()
    }

    /// Set the [Window] fullscreen mode. This mode is applied when [Window] is showed.
    pub fn fullscreen(&mut self, fsmode : WindowFullScreenMode) -> &mut Self {
        todo!()
    }

    /// [Window] built will be showed on desktop upon creation.
    pub fn show(&mut self) -> &mut Self {
        todo!()
    }

    /// Reset the [WindowBuilder] with default values. 
    pub fn reset(&mut self) -> &mut Self {
        todo!()
    }

    /// Create a new [Window] within the [WindowManager].
    pub fn build(&mut self, wm : &mut WindowManager) -> Result<WindowHandle, WindowError> {
        todo!()
    }

    /// Rebuild a [Window] from the [WindowBuilder] parameters.
    pub fn rebuild(&mut self, w : &mut Window) -> Result<WindowHandle, WindowError> {
        todo!()
    }

}