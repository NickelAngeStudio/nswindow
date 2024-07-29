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

/// Builder unit tests
#[cfg(test)]
pub(crate) mod tests {
    include!("tests/builder.rs");
}

use crate::{frame::WindowFrame, keyboard::WindowKeyboard, pointer::WindowPointer, Window, WindowError, WindowFullScreenMode, WindowHandle, WindowManager, WindowRelativePosition, WindowSize};

/// Default title of [Window].
const WB_DEFAULT_TITLE : &str = "New window";

/// Default size of [Window].
const WB_DEFAULT_SIZE : WindowSize = WindowSize { width: 640, height: 480 };

/// Default minimum size of [Window]. Used for user resizing.
const WB_DEFAULT_MIN_SIZE : WindowSize = WindowSize { width: 320, height: 240 };

/// Default maximum size of [Window]. Used for user resizing.
const WB_DEFAULT_MAX_SIZE : WindowSize = WindowSize { width: u16::MAX as u32, height: u16::MAX as u32 };

/// Default position of [Window].
const WB_DEFAULT_POSITION : WindowRelativePosition = WindowRelativePosition::PrimaryCenter;

/// Default parent of [Window].
const WB_DEFAULT_PARENT : Option<WindowHandle> = None;

/// Default fullscreen mode of [Window].
const WB_DEFAULT_FSMODE : Option<WindowFullScreenMode> = None;

/// Default minimized toggle of [Window].
const WB_DEFAULT_MINIMIZED : bool = false;

/// Default maximized toggle of [Window].
const WB_DEFAULT_MAXIMIZED : bool = false;

/// Default visibility toggle of [Window].
const WB_DEFAULT_VISIBLE : bool = true;

/// Default visibility toggle of [Window] taskbar.
const WB_DEFAULT_TASKBAR : bool = true;



/// [WindowBuilder] used to create new [Window] or recreate an actual one.
pub struct WindowBuilder {

    /// Buffer of the icon of [Window].
    icon : Option<Vec<u8>>,

    /// Title of the [Window]
    title : String,

    /// Minimum size of the [Window].
    min_size : WindowSize,

    /// Maximum size of the [Window].
    max_size : WindowSize,

    /// Size of the [Window]
    size : WindowSize,

    /// Relative position of the [Window].
    position : WindowRelativePosition,

    /// Parent of the [Window].
    parent : Option<WindowHandle>,

    /// Frame properties for [Window].
    frame : WindowFrame,

    /// Keyboard properties for [Window].
    keyboard : WindowKeyboard,

    /// Pointer properties for [Window]
    pointer : WindowPointer,

    /// Fullscreen mode
    fsmode : Option<WindowFullScreenMode>,

    /// Window minimized
    minimized : bool,

    /// Window maximized
    maximized : bool,

    /// Window will be showed on desktop when created.
    visible : bool,

    /// Window will be showed in the taskbar
    taskbar : bool,
}

impl WindowBuilder {

    /// Create a new [WindowBuilder] instance with default parameters. See 
    pub fn new() -> WindowBuilder {
        WindowBuilder { 
            icon : None,
            title: WB_DEFAULT_TITLE.to_string(), 
            min_size : WB_DEFAULT_MIN_SIZE,
            max_size : WB_DEFAULT_MAX_SIZE,
            size: WB_DEFAULT_SIZE, 
            position: WB_DEFAULT_POSITION, 
            parent: WB_DEFAULT_PARENT, 
            keyboard: WindowKeyboard::new(), 
            pointer: WindowPointer::new(), 
            frame: WindowFrame::new(),
            fsmode: WB_DEFAULT_FSMODE,
            minimized: WB_DEFAULT_MINIMIZED,
            maximized: WB_DEFAULT_MAXIMIZED, 
            visible : WB_DEFAULT_VISIBLE,
            taskbar : WB_DEFAULT_TASKBAR,
        }
    }

    /// Set the [Window] title of the new [Window].
    pub fn title(&mut self, title : &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    /// Set the minimum [WindowSize] a user can shrink the window to.
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowMinSizeBiggerThanMax`] if min `size` is bigger than `max` size.
    pub fn size_min(&mut self, size : WindowSize) -> &mut Self {
        self.min_size = size;
        self
    }

     /// Set the maximum [WindowSize] a user can enlarge the window to.
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowMinSizeBiggerThanMax`] if min `size` is bigger than `max` size.
    pub fn size_max(&mut self, size : WindowSize) -> &mut Self {
        self.max_size = size;
        self
    }

    /// Set the [WindowSize] of the new window.
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowSizeOOB`] if `size` isn't between [Desktop::min](crate::display::Desktop::min) and [Desktop::max](crate::display::Desktop::max)
    /// and / or `size` isn't between [WindowBuilder::size_min()] and [WindowBuilder::size_max()].
    pub fn size(&mut self, size : WindowSize) -> &mut Self {
        self.size = size;
        self
    }

    /// Set the [WindowRelativePosition] of the window to be build. 
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowRelativePositionOOB`] if position would be out of desktop bound.
    pub fn position(&mut self, position : WindowRelativePosition) -> &mut Self {
        self.position = position;
        self
    }

    /// Override the default [WindowPointer].
    pub fn pointer(&mut self, pointer : WindowPointer) -> &mut Self {
        self.pointer = pointer;
        self
    }

    /// Override the default [WindowKeyboard].
    pub fn keyboard(&mut self, keyboard : WindowKeyboard) -> &mut Self {
        self.keyboard = keyboard;
        self
    }

    /// Override the default [WindowFrame].
    pub fn frame(&mut self, frame : WindowFrame)-> &mut Self {
        self.frame = frame;
        self
    }

     /// Set the [Window] icon from a [std::io::Read] source.
    /// 
    /// Set the `icon` parameter to [None] to display default [Window] icon.
    pub fn icon(&mut self, icon : Option<&mut dyn std::io::Read>)  -> &mut Self {
        
        match icon {
            Some(icon) => {
                let mut buf : Vec<u8> = Vec::new();
                let _ = icon.read_to_end(&mut buf);
                self.icon = Some(buf);
            },
            None => self.icon = None,
        }
        self
    }


    /// [WindowHandle] of the parent of the new [Window].
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::InvalidWindowHandle`] if [WindowHandle] doesn't refer to any [Window].
    /// [WindowBuilder::rebuild] returns [`WindowError::WindowParentSelf`] if [WindowHandle] if the same as the [Window] itself.
    /// [WindowBuilder::build] returns [`WindowError::WindowParentLoop`] if a parent loop would occur upon creation.
    pub fn parent(&mut self, parent : Option<WindowHandle>) -> &mut Self {
        self.parent = parent;
        self
    }
   
    /// Set the [Window] fullscreen mode. This mode is applied when [Window] is showed.
    pub fn fullscreen(&mut self, fsmode : Option<WindowFullScreenMode>) -> &mut Self {
        self.fsmode = fsmode;
        self
    }

    /// [Window] will be showed as minimized.
    pub fn minimize(&mut self) -> &mut Self {
        self.minimized = true;
        self
    }

    /// [Window] will be showed as maximized.
    pub fn maximize(&mut self) -> &mut Self {
        self.maximized = true;
        self
    }

    /// [Window] built will not be showed on desktop upon creation.
    /// 
    /// By default, [Window] created are visible.
    pub fn hide(&mut self) -> &mut Self {
        self.visible = false;
        self
    }

    /// Toggle showing the [Window] is the taskbar or not.
    pub fn taskbar(&mut self, show : bool) -> &mut Self {
        self.taskbar = show;
        self
    }

    /// Reset the [WindowBuilder] with default values. 
    pub fn reset(&mut self) -> &mut Self {
        self.icon = None;
        self.title = WB_DEFAULT_TITLE.to_string();
        self.min_size = WB_DEFAULT_MIN_SIZE;
        self.max_size = WB_DEFAULT_MAX_SIZE;
        self.size = WB_DEFAULT_SIZE;
        self.position = WB_DEFAULT_POSITION; 
        self.parent = WB_DEFAULT_PARENT;
        self.keyboard = WindowKeyboard::new(); 
        self.pointer = WindowPointer::new();
        self.frame = WindowFrame::new();
        self.fsmode = WB_DEFAULT_FSMODE;
        self.minimized = WB_DEFAULT_MINIMIZED;
        self.maximized = WB_DEFAULT_MAXIMIZED;
        self.visible = WB_DEFAULT_VISIBLE;
        self.taskbar = WB_DEFAULT_TASKBAR;
        self
    }

    /// Build a new [Window] within the given [WindowManager].
    pub fn build(&mut self, wm : &mut WindowManager) -> Result<WindowHandle, WindowError> {
        wm.build(self)
    }

    /// Rebuild a [Window] from the [WindowBuilder] parameters.
    pub fn rebuild(&mut self, w : &mut Window) -> Result<WindowHandle, WindowError> {
        w.rebuild(self)
    }

}