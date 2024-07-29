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

//! Properties of the border surrounding a [Window](crate::Window)


/// Frame unit tests
#[cfg(test)]
pub(crate) mod tests {
    include!("tests/frame.rs");
}


use nscfg::match_cfg;

/// [WindowFrame] default visibility.
const WF_DEFAULT_VISIBILITY : bool = true;

/// [WindowFrame] default resize toggle
const WF_DEFAULT_RESIZABLE : bool = true;

/// [WindowFrame] default mode for all buttons.
const WF_DEFAULT_BUTTON : WindowFrameButtonMode = WindowFrameButtonMode::Default;


/// Mode of a button on the [WindowFrame].
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WindowFrameButtonMode {
    /// Button will be showed as default OS settings and act according to OS parameters.
    Default,

    /// Button is hidden on the frame.
    Hidden,

    /// Button is visible but does nothing. Will be grayed out if possible.
    Disable,

    /// Button is visible but won't act according to OS parameters. 
    /// Events will be generated instead.
    /// 
    /// # Events
    /// - [`WindowEvent::MinimizeButtonPressed`](crate::event::WindowEvent::MinimizeButtonPressed) happens when user click the min button.
    /// - [`WindowEvent::MaximizeButtonPressed`](crate::event::WindowEvent::MaximizeButtonPressed) happens when user click the max button.
    /// - [`WindowEvent::CloseButtonPressed`](crate::event::WindowEvent::CloseButtonPressed) happens when user click the close button.
    Overriden,
}

/// Properties of the [Window](crate::Window) frame.
/// 
/// Frame is the borders that surround the [Window](crate::Window).
/// 
/// # Note
/// On linux, [WindowFrame] properties might not all works depending on the linux flavour used.
#[derive(Debug, PartialEq)]
pub struct WindowFrame {

    /// Linux [WindowFrame] abstraction for calls. Is set as [Option] since [WindowBuilder] can use it.
    #[cfg(target_os = "linux")]
    frame : Option<crate::linux::frame::LinuxWindowFrame>,

    /// Is window frame visible
    pub(crate)  visible : bool,

    /// Can the frame be used to resize window.
    pub(crate)  resizable : bool,

    /// Mode of the min button 
    pub(crate)  min_button : WindowFrameButtonMode,

    /// Mode of the max button
    pub(crate)  max_button : WindowFrameButtonMode,

    /// Mode of the close button
    pub(crate)  close_button : WindowFrameButtonMode,

}


impl WindowFrame {

    pub fn new() -> WindowFrame {
        WindowFrame { 
            frame: None, 
            visible: WF_DEFAULT_VISIBILITY, 
            resizable: WF_DEFAULT_RESIZABLE, 
            min_button: WF_DEFAULT_BUTTON, 
            max_button: WF_DEFAULT_BUTTON, 
            close_button: WF_DEFAULT_BUTTON 
        }
    }
    
    /// Returns true if the [WindowFrame] is visible around the [Window](crate::Window).
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Show the [WindowFrame] around the [Window](crate::Window). Does nothing if already visible.
    pub fn show(&mut self) {

        if !self.visible {

            self.visible = true;

            match_cfg! {
                linux => {
                    match self.frame {
                        Some(ref mut wf) => wf.show(),
                        None => {},
                    }
                },
                _ => {
                    unimplemented!()
                }
            }

        }

    }

    /// Hide the [WindowFrame] around the [Window](crate::Window). Does nothing if already hidden.
    pub fn hide(&mut self) {

        if self.visible {

            self.visible = false;

            match_cfg! {
                linux => {
                    match self.frame {
                        Some(ref mut wf) => wf.hide(),
                        None => {},
                    }
                },
                _ => {
                    unimplemented!()
                }
            }

        }

    }

    /// Returns true if the [WindowFrame] border can be used to resize [Window](crate::Window).
    pub fn resizable(&self)  -> bool {
        self.resizable
    }

    /// Prevent the [WindowFrame] from been resized by the user. Does nothing if already locked.
    pub fn lock(&mut self) {

        if self.resizable {

            self.resizable = false;

            match_cfg! {
                linux => {
                    match self.frame {
                        Some(ref mut wf) => wf.lock(),
                        None => {},
                    }
                },
                _ => {
                    unimplemented!()
                }
            }

        }

    }

    /// Make the [WindowFrame] resizable by the user. Does nothing if already unlocked.
    pub fn unlock(&mut self) {
        
        if !self.resizable {

            self.resizable = true;

            match_cfg! {
                linux => {
                    match self.frame {
                        Some(ref mut wf) => wf.unlock(),
                        None => {},
                    }
                },
                _ => {
                    unimplemented!()
                }
            }

        }

    }

    /// Returns [WindowFrame] minimum [WindowFrameButtonMode]. 
    pub fn button_min(&self) -> WindowFrameButtonMode {
        self.min_button
    }

    /// Set a new [WindowFrameButtonMode] for the minimum button. Does nothing is same mode as before.
    pub fn set_button_min(&mut self, mode : WindowFrameButtonMode) {

        if self.min_button != mode {

            self.min_button = mode;

            match_cfg! {
                linux => {
                    match self.frame {
                        Some(ref mut wf) => wf.set_button_min(mode),
                        None => {},
                    }
                },
                _ => {
                    unimplemented!()
                }
            }

        }

    }

    /// Returns [WindowFrame] maximum [WindowFrameButtonMode]. 
    pub fn button_max(&self) -> WindowFrameButtonMode {
        self.max_button
    }

    /// Set a new [WindowFrameButtonMode] for the maximum button. Does nothing is same mode as before.
    pub fn set_button_max(&mut self, mode : WindowFrameButtonMode) {

        if self.max_button != mode {

            self.max_button = mode;

            match_cfg! {
                linux => {
                    match self.frame {
                        Some(ref mut wf) => wf.set_button_max(mode),
                        None => {},
                    }
                },
                _ => {
                    unimplemented!()
                }
            }

        }


    }

    /// Returns [WindowFrame] close [WindowFrameButtonMode]. 
    pub fn button_close(&self) -> WindowFrameButtonMode {
        self.close_button
    }

    /// Set a new [WindowFrameButtonMode] for the close button. Does nothing is same mode as before.
    pub fn set_button_close(&mut self, mode : WindowFrameButtonMode) {

        if self.close_button != mode {

            self.close_button = mode;

            match_cfg! {
                linux => {
                    match self.frame {
                        Some(ref mut wf) => wf.set_button_close(mode),
                        None => {},
                    }
                },
                _ => {
                    unimplemented!()
                }
            }

        }

    }

   

    /// Reset the [WindowFrame] to default settings.
    pub fn reset(&mut self) {
        self.visible = WF_DEFAULT_VISIBILITY;
        self.resizable = WF_DEFAULT_RESIZABLE;
        self.min_button = WF_DEFAULT_BUTTON;
        self.max_button = WF_DEFAULT_BUTTON;
        self.close_button = WF_DEFAULT_BUTTON;

        match_cfg! {
            linux => {
                match self.frame {
                    Some(ref mut wf) => wf.reset(),
                    None => {},
                }
            },
            _ => {
                unimplemented!()
            }
        }
        


    }
}