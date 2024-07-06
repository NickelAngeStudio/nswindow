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

/// [WindowFrame] default visibility.
const WF_DEFAULT_VISIBILITY : bool = true;

const WF_DEFAULT_RESIZABLE : bool = true;

const WF_DEFAULT_ICON : WindowFrameIcon = WindowFrameIcon::Default;

const WF_DEFAULT_BUTTON : WindowFrameButton = WindowFrameButton::Default;


/// Mode of the icon showed on the [WindowFrame].
pub enum WindowFrameIcon {
    /// Icon will be showed according to default OS settings.
    Default,

    /// Icon will be hidden (if possible).
    Hidden,

    /// Icon has been changed via 
    Custom,
}



/// Mode of a button on the [WindowFrame].
pub enum WindowFrameButton {
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
    /// - [`WindowEvent::RequestMinimize`](crate::event::WindowEvent::RequestMinimize) happens when user click the min button.
    /// - [`WindowEvent::RequestMazimize`](crate::event::WindowEvent::RequestMinimize) happens when user click the max button.
    /// - [`WindowEvent::RequestClose`](crate::event::WindowEvent::RequestMinimize) happens when user click the close button.
    Overriden,
}

/// Properties of the [Window](crate::Window) frame.
/// 
/// Frame is the borders that surround the [Window](crate::Window).
/// 
/// # Note
/// On linux, [WindowFrame] properties might not all works depending on the linux flavour used.
pub struct WindowFrame {

    /// Linux [WindowFrame] abstraction for calls. Is set as [Option] since [WindowBuilder] can use it.
    #[cfg(target_os = "linux")]
    frame : Option<crate::linux::frame::LinuxWindowFrame>,

    /// Is window frame visible
    visible : bool,

    /// Can the frame be used to resize window.
    resizable : bool,

    /// Frame icon mode
    icon : WindowFrameIcon,

    /// Mode of the min button 
    min_button : WindowFrameButton,

    /// Mode of the max button
    max_button : WindowFrameButton,

    /// Mode of the close button
    close_button : WindowFrameButton,

}

impl WindowFrame {

    pub fn new() -> WindowFrame {
        WindowFrame { 
            frame: None, 
            visible: WF_DEFAULT_VISIBILITY, 
            resizable: WF_DEFAULT_RESIZABLE, 
            icon: WF_DEFAULT_ICON, 
            min_button: WF_DEFAULT_BUTTON, 
            max_button: WF_DEFAULT_BUTTON, 
            close_button: WF_DEFAULT_BUTTON 
        }
    }

    /// Set the [WindowFrame] icon. If [WindowFrameIcon::Custom], an implemented [std::io::Read] must be given.
    pub fn set_icon(&mut self, mode : WindowFrameIcon, read : Option<&dyn std::io::Read>) {
        todo!()
    }

    /// Reset the [WindowFrame] to default settings.
    pub fn reset(&mut self) {

    }
}


/*************
* UNIT TESTS * 
*************/
#[cfg(test)]
mod tests{
    use super::WindowFrame;


    /// Unit tests [super::WindowFrame] default values.
    ///
    /// # Verification(s)
    /// V1 | Test each value on creation vs default values.
    #[test]
    fn ut_window_frame_default() {
        let mut wf = WindowFrame::new();

        wf.set_icon(super::WindowFrameIcon::Default, None);
        

    }

   

}

