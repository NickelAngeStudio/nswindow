#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]
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

// Used to generate target tags in doc.
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Nifty and Simple Window provides easy to use structs and objects to easily create window on the supported platform of your choice.


target_cfg! {
    linux => {
        /// Linux OS window managers functions.
        pub(crate) mod linux;
    }
}


use nscfg::target_cfg;

/// Possible nswmd errors.
#[doc(hidden)] 
pub mod error;

/// Window Manager
#[doc(hidden)] 
pub mod manager;

/// Window handle
#[doc(hidden)] 
pub mod window;

/// Window properties
#[doc(hidden)] 
pub mod property; 

/// Window builder
#[doc(hidden)] 
pub mod builder;

// Hardware display information
pub mod display;

/// Possible events that can be given by the [WindowManager].
#[doc(hidden)] 
pub mod event;

// Re-import
pub use builder::WindowBuilder as WindowBuilder;
pub use manager::WindowManager as WindowManager;
pub use window::Window as Window;
pub use property::WindowKeyboardMode as WindowKeyboardMode;
pub use property::WindowPointerMode as WindowPointerMode;
pub use property::WindowPointer as WindowPointer;
pub use property::WindowKeyboard as WindowKeyboard;
pub use property::WindowFullScreenMode as WindowFullScreenMode;
pub use property::WindowPosition as WindowPosition;
pub use property::WindowRelativePosition as WindowRelativePosition;
pub use property::WindowSize as WindowSize;
pub use error::WindowError as WindowError;
pub use event::WindowManagerEvent as WindowManagerEvent;

/// Window handle used by the [WindowManager].
pub type WindowHandle = *const usize;
