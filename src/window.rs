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


#[cfg(test)]
mod tests{
    // Window unit tests
    include!("tests/window.rs");
}



use std::rc::Rc;

use nscfg::meta_cfg;

use crate::{ display::{Desktop, DisplayHandle, DisplayResolution, Displays}, frame::WindowFrame, keyboard::WindowKeyboard, pointer::WindowPointer, WindowBuilder, WindowError};



/// Window handle used by the [WindowManager](crate::WindowManager).
/// 
/// It is the same handle used by the operating system.
pub type WindowHandle = *const usize;

/// [Window](crate::Window) size as width and height.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct WindowSize {
    pub width : u32,
    pub height : u32,
}

impl WindowSize {
    /// Create a new [WindowSize] from width and height.
    pub fn new(width : u32, height : u32) -> WindowSize {
        WindowSize { width, height }
    }
}


/// [Window](crate::Window) position according to x and y axis.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WindowPosition {
    pub x : i32,
    pub y : i32,
}

impl WindowPosition {
    /// Create a new [WindowPosition] from x and y.
    pub fn new(x : i32, y : i32) -> WindowPosition {
        WindowPosition { x, y }
    }
}



/// Possible [Window](crate::Window) relative positions.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowRelativePosition {
    /// Position window on desktop from an absolute pair of x,y coordinates.
    Desktop(WindowPosition),

    /// Position window on the center of desktop.
    DesktopCenter,

    /// Position window on a specific display from an absolute pair of x,y coordinates.
    Display(DisplayHandle, WindowPosition),

    /// Position window in the center of given display.
    DisplayCenter(DisplayHandle),

    /// Position window relative to parent window. All [Window] have parent, up to the root which is the desktop.
    #[cfg(any(doc, not(feature = "single_opt")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "single_opt"))))]
    Parent(WindowPosition),

    /// Position window in the center of parent window. All [Window] have parent, up to the root which is the desktop.
    #[cfg(any(doc, not(feature = "single_opt")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "single_opt"))))]
    ParentCenter,

    /// Position window on the primary display with [WindowPosition].
    Primary(WindowPosition),

    /// Position window on the center of primary display.
    PrimaryCenter,
}

/// Possible [Window](crate::Window) fullscreen modes.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowFullScreenMode {

    /// Window will be set fullscreen in the current display this window belong to.
    Current,

    /// Window will be set fullscreen in the primary display.
    Primary,

    /// Window will be set fullscreen for entire desktop which can be set across multiple display.
    Desktop,

    /// Window will be set fullscreen for the specified display
    Display(DisplayHandle)
}


/// [Window] is used to manipulate an individual window.
/// 
/// TODO: Develop more
#[derive(Debug)]
pub struct Window {
    /// Linux [Window] abstraction for calls.
    #[cfg(target_os = "linux")]
    pub(crate) window : crate::linux::window::LinuxWindow,

    /// Reference counter to displays
    pub(crate) displays : Rc<Displays>,  

    /// Handle of the [Window]
    pub(crate) handle : WindowHandle,

    /// Parent [WindowHandle]
    #[cfg(any(doc, not(feature = "single_opt")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "single_opt"))))]
    pub(crate) parent : Option<WindowHandle>,


    /// Child [WindowHandle] array.
    #[cfg(any(doc, not(feature = "single_opt")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "single_opt"))))]
    pub(crate) childs : Vec<WindowHandle>,

    /// Modal [WindowHandle] blocking this [Window].
    #[cfg(any(doc, not(feature = "single_opt")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "single_opt"))))]
    pub(crate) modal : Option<WindowHandle>,

    /// [SubWindow] properties
    #[cfg(any(doc, not(feature = "single_opt")))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "single_opt"))))]
    pub(crate) sub : Option<crate::sub::SubWindow>,

    /// [Window] frame properties.
    pub(crate) frame : WindowFrame,

    /// [Window] keyboard properties.
    pub(crate) keyboard : WindowKeyboard,

    /// [Window] pointer properties.
    pub(crate) pointer : WindowPointer,

    /// Title of the [Window]
    pub(crate) title : String,

    /// Size of the [Window].
    pub(crate) size : WindowSize,

    /// Minimum size of the [Window].
    pub(crate) min_size : WindowSize,

    /// Maximum size of the [Window].
    pub(crate) max_size : WindowSize,

    /// Position of the [Window] on the desktop.
    pub(crate) desktop_position : WindowPosition,

    /// Is Window Fullscreen
    pub(crate) fullscreen : bool,

    /// Is Window minimized
    pub(crate) minimized : bool,

    /// Is Window maximized
    pub(crate) maximized : bool,

    /// Window is currently visible.
    pub(crate) visible : bool,

    /// Show the window in the taskbar.
    pub(crate) taskbar : bool,
}


impl Window {

    /// Function invoked by the builder when rebuilding a [Window].
    pub(crate) fn rebuild(&mut self, builder : &WindowBuilder) -> Result<WindowHandle, WindowError> {
        self.window.rebuild(builder)
    }

    /// Returns the [WindowHandle] of the [Window].
    pub fn handle(&self) -> WindowHandle {
        self.handle
    }

    /// Returns immutable array of [Window] childs.
    #[meta_cfg(!single_opt:ft)]
    pub fn childs(&self) -> &Vec<WindowHandle> {
        &self.childs
    }

    /// Returns the parent [WindowHandle] if any.
    #[meta_cfg(!single_opt:ft)]
    pub fn parent(&self) -> Option<WindowHandle> {
        self.parent
    }
    
    /// Set a [Window] parent.
    /// 
    /// Returns Ok(true) if the parent changed, false otherwise.
    /// 
    /// # Errors
    /// - Returns [`WindowError::InvalidWindowHandle`] if parent [WindowHandle] doesn't refer to any [Window].
    /// - Returns [`WindowError::WindowParentSelf`] if [WindowHandle] if the same as the [Window] itself.
    /// - Returns [`WindowError::WindowParentLoop`] if a parent loop would occur.
    #[meta_cfg(!single_opt:ft)]
    pub fn set_parent(&mut self, parent : Option<WindowHandle>) -> Result<bool, WindowError> {

        match parent {
            Some(new_parent) => {
                match self.parent {
                    Some(old_parent) => {
                        if new_parent == old_parent {
                            Ok(false)
                        } else  if new_parent == self.handle {
                            Err(WindowError::WindowParentSelf)
                        } else {
                            self.parent = Some(new_parent);
                            self.window.set_parent(parent)
                        }
                    }
                    None => {
                        if new_parent == self.handle {
                            Err(WindowError::WindowParentSelf)
                        } else {
                            self.parent = Some(new_parent);
                            self.window.set_parent(parent)
                        }
                    },
                }
            },
            None => {
                if self.parent.is_some() {
                    self.parent = None;
                    self.window.set_parent(parent)
                } else {
                    Ok(false)
                }
            },
        }
    }

    /// Returns the [Window] title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the [Window] title.
    pub fn set_title(&mut self, title : &str) -> Result<bool, WindowError>{
        self.title = title.to_string();
        self.window.set_title(title)
    }

    /// Get [WindowSize] of [Window].
    pub fn size(&self) -> WindowSize {
        self.size
    }

    /// Set [WindowSize] of [Window].
    /// 
    /// # Errors
    /// Returns Err(['WindowError::WindowSizeOOB']) if size is not between min and max.
    pub fn set_size(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        
        if self.size > self.max_size || self.size < self.min_size {
            Err(WindowError::WindowSizeOOB)
        } else {
            self.size = size;
            self.window.set_size(size)
        }
    }

    /// Get the minimum [WindowSize] of [Window].
    pub fn size_min(&self) -> WindowSize {
        self.min_size
    }

    /// Set the minimum [WindowSize] of [Window].
    /// 
    /// Returns Ok(true) if changed, Ok(false) otherwise.
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowMinSizeBiggerThanMax`] if min `size` is bigger than `max` size.
    pub fn set_size_min(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        
        if size > self.max_size {
            Err(WindowError::WindowMinSizeBiggerThanMax)
        } else {
            if self.min_size != size {
                self.min_size = size;
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    /// Get the maximum [WindowSize] of [Window].
    pub fn size_max(&self) -> WindowSize {
        self.max_size
    }

    /// Set the maximum [WindowSize] of [Window].
    /// 
    /// Returns Ok(true) if changed, Ok(false) otherwise.
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowMinSizeBiggerThanMax`] if min `size` is bigger than `max` size.
    pub fn set_size_max(&mut self, size : WindowSize) -> Result<bool, WindowError> {
        
        if size < self.min_size {
            Err(WindowError::WindowMinSizeBiggerThanMax)
        } else {
            if self.max_size != size {
                self.max_size = size;
                Ok(true)
            } else {
                Ok(false)
            }
        }

    }

     /// Set the [Window] icon from a [std::io::Read] source.
     /// 
     /// Set the `icon` parameter to [None] to display default icon.
     pub fn set_icon(&mut self, icon : Option<&mut dyn std::io::Read>) {
        self.window.set_icon(icon)
    }

    /// Returns true if the [Window] is showed in the taskbar.
    pub fn taskbar(&self) -> bool {
        self.taskbar
    }

    /// Set if the [Window] is showed in the taskbar.
    pub fn set_taskbar(&mut self, show : bool) {
        if self.taskbar != show {
            self.taskbar = show;
            self.window.set_taskbar(show)
        }
    }

    /// Returns true if [Window] is visible.
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Restore the [Window], removing minimize, maximize and/or fullscreen modes.
    pub fn restore(&mut self) {
        
        if self.fullscreen || self.minimized || self.maximized {
            self.fullscreen = false;
            self.minimized = false;
            self.maximized = false;
            self.window.restore();
        }

    }

    /// Show the [Window] on display.
    pub fn show(&mut self) {
        if !self.visible {
            self.window.show();
        }
    }

    /// Hide the [Window] on display.
    pub fn hide(&mut self) {
        if self.visible {
            self.window.hide();
        }
    }

    /// Close the [Window], removing it from display and clearing the ressources.
    /// 
    /// [WindowHandle] will be invalid if invoked again.
    pub fn close(&mut self) {
        self.window.close();
    }


    /// [WindowPosition] of the [Window] on the desktop.
    pub fn position(&mut self) -> WindowPosition {
        self.desktop_position
    }

    /// Set the position of the [Window] with a [WindowRelativePosition] and checking for desktop out of bounds.
    /// 
    /// Returns Ok([WindowPosition]) with the position on the desktop.
    /// 
    /// # Errors
    /// Returns Err([`WindowError::WindowRelativePositionOOB`]) if position would get out of desktop bound.
    /// Returns Err([`WindowError::DisplayInvalidHandle`]) if an invalid display handle was given.
    pub fn set_position(&mut self, position : WindowRelativePosition) -> Result<WindowPosition, WindowError> {
        self.set_window_position(position, true)
    }
    
    // Set the position of the [Window] with a [WindowRelativePosition] without checking for desktop out of bounds.
    /// 
    /// Returns Ok([WindowPosition]) with the position on the desktop.
    /// 
    /// # Errors
    /// Returns Err([`WindowError::DisplayInvalidHandle`]) if an invalid display handle was given.
    pub fn set_position_unchecked(&mut self, position : WindowRelativePosition) -> Result<WindowPosition, WindowError> {
        self.set_window_position(position, true)
    }


    /// Returns immutable reference to the [WindowFrame].
    pub fn frame(&self) -> &WindowFrame {
        &self.frame
    }

    /// Returns mutable reference to the [WindowFrame].
    pub fn frame_mut(&mut self) -> &mut WindowFrame {
        &mut self.frame
    }

    /// Returns immutable reference to the [WindowKeyboard].
    pub fn keyboard(&self) -> &WindowKeyboard {
        &self.keyboard
    }

    /// Returns mutable reference to the [WindowKeyboard].
    pub fn keyboard_mut(&mut self) -> &mut WindowKeyboard {
        &mut self.keyboard
    }

    /// Returns immutable reference to the [WindowPointer].
    pub fn pointer(&self) -> &WindowPointer {
        &self.pointer
    }

    /// Returns mutable reference to the [WindowPointer].
    pub fn pointer_mut(&mut self) -> &mut WindowPointer {
        &mut self.pointer
    }

    /// Returns true if the [Window] is currently in fullscreen mode.
    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Set the [Window] fullscreen mode. Use [Window::restore()] to exit fullscreen mode.
    pub fn set_fullscreen(&mut self, fsmode : WindowFullScreenMode) {
        
        if !self.fullscreen {
            self.window.set_fullscreen(fsmode);
            self.fullscreen = true;
        }

    }

    /// Returns true if [Window] is currently minimized.
    pub fn minimized(&self) -> bool {
        self.minimized
    }

    /// Minimize the [Window] in the taskbar.
    pub fn minimize(&mut self)  {
        
        if !self.minimized {
            self.window.minimize();
            self.minimized = true;
        }

    }
    
    /// Returns true if [Window] is currently maximized.
    pub fn maximized(&mut self) -> bool {
        self.maximized
    }

    /// Maximize the [Window] in it's current display.
    pub fn maximize(&mut self) {
        
        if !self.maximized {
            self.window.maximize();
            self.maximized = true;
        }

    }

    /// Set the window desktop position with or without out of bound checks.
    fn set_window_position(&mut self, position : WindowRelativePosition, check_oob : bool) -> Result<WindowPosition, WindowError> {

        match match &position {
            #[cfg (any (doc , not (feature = "single_opt")))]
            WindowRelativePosition::Parent(_) | WindowRelativePosition::ParentCenter => {
                match self.parent {
                    Some(parent) => Self::get_window_desktop_position(self.size, self.displays.clone(), 
                        position, Some(self.window.get_window_pos_size(parent)), check_oob),
                    None => {
                        match &self.displays.desktop {
                            Some(desktop) => Self::get_window_desktop_position(self.size, self.displays.clone(), 
                                position, Some((WindowPosition { x: 0, y: 0 }, WindowSize { width: desktop.current.width as u32, height: desktop.current.height as u32 })), check_oob),
                            None => Self::get_window_desktop_position(self.size, self.displays.clone(), 
                                position, Some((WindowPosition { x: 0, y: 0 }, WindowSize{ width: 0, height: 0 })), check_oob),
                        }
                        
                    },
                }
                
            },
            _ => Self::get_window_desktop_position(self.size, self.displays.clone(), position, None, check_oob)
        } {
            Ok(position) => {
                self.window.set_position(position);
                Ok(position)
            },
            Err(err) => Err(err),
        }

    }


    /// Get the window desktop position according to [WindowRelativePosition].
    /// `pps` means Parent position size.
    /// 
    /// Returns Err([WindowError::WindowRelativePositionOOB]) if any parts overflow from desktop.
    /// Returns Err([WindowError::DisplayInvalidHandle]) if a display handle doesn't exists.
    fn get_window_desktop_position(size : WindowSize, displays : Rc<Displays>, position : WindowRelativePosition, pps : Option<(WindowPosition, WindowSize)>, check_oob : bool) -> Result<WindowPosition, WindowError> {
        let position : Result<WindowPosition, WindowError> = match position {
            WindowRelativePosition::Desktop(position) => Ok(position),
            WindowRelativePosition::DesktopCenter => {
                match &displays.desktop {
                    Some(desktop) => {
                        let position = WindowPosition { x: ((desktop.current.width as i32 - size.width as i32) / 2) as i32, y: ((desktop.current.height as i32 - size.height as i32) / 2) as i32 };
                        Ok(position)
                    },
                    // No desktop is put to 0,0
                    None => Self::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(WindowPosition { x: 0, y: 0 }), None, check_oob),
                }
            }
            WindowRelativePosition::Display(handle, position) => if handle < displays.list.len() {
                let display = &displays.list[handle];
                Ok(WindowPosition { x: position.x + display.position.x, y: position.y + display.position.y })
            } else {
                Err(WindowError::DisplayInvalidHandle)
            },
            WindowRelativePosition::DisplayCenter(handle) => if handle < displays.list.len() {
                let display = &displays.list[handle];
                let position = WindowPosition { x: ((display.resolution.width as i32 - size.width as i32) / 2) as i32, y: ((display.resolution.height as i32 - size.height as i32) / 2) as i32 };
                Ok(WindowPosition { x: position.x + display.position.x, y: position.y + display.position.y })
            } else {
                Err(WindowError::DisplayInvalidHandle)
            },
            #[cfg (any (doc , not (feature = "single_opt")))]
            WindowRelativePosition::Parent(position) => match pps {
                Some(pps) => {
                    Ok(WindowPosition { x: position.x + pps.0.x, y: position.y + pps.0.y })
                },
                // No parent is returned to desktop
                None => Self::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(position), None, check_oob),
            },
            #[cfg (any (doc , not (feature = "single_opt")))]
            WindowRelativePosition::ParentCenter => match pps {
                Some(pps) => {
                    let position = WindowPosition { x: ((pps.1.width as i32 - size.width as i32) / 2) as i32, y: ((pps.1.height as i32 - size.height as i32) / 2) as i32 };
                    Ok(WindowPosition { x: position.x + pps.0.x, y: position.y + pps.0.y })
                },
                // No parent is returned to desktop
                None => Self::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DesktopCenter, None, check_oob),
            },
            WindowRelativePosition::PrimaryCenter => {
                match displays.primary() {
                    Some(display) => Self::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DisplayCenter(display.handle), None, check_oob),
                    // If no primary display, revert to desktop center
                    None => Self::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::DesktopCenter, None, check_oob),
                }
            },
            WindowRelativePosition::Primary(position) => {
                match displays.primary() {
                    Some(display) => Self::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Display(display.handle, position), None, check_oob),
                    // If no primary display, revert to desktop
                    None => Self::get_window_desktop_position(size, displays.clone(), WindowRelativePosition::Desktop(position), None, check_oob),
                }
            },
        };

        if !check_oob {  // If test for out of bound not enabled
            position
        } else {
            match position {
                Ok(position) => {
                    // Test for OOB
                    let desktop : Desktop = match &displays.desktop {
                        Some(desktop) => desktop.clone(),
                        None => Desktop { min: DisplayResolution { width: 0, height: 0 }, max: DisplayResolution { width: 0, height: 0 }, current: DisplayResolution { width: 0, height: 0 } },
                    };
                    if position.x < 0 || position.x + size.width as i32 > desktop.current.width as i32 || position.y < 0 || position.y + size.height as i32 > desktop.current.height as i32 {
                        Err(WindowError::WindowRelativePositionOOB)
                    } else {
                        Ok(position)
                    }
                },
                Err(err) => Err(err),
            }
        }

        

        
    }

}