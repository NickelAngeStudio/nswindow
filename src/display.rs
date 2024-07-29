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


//! Hardware display information.

/// Display unit tests
#[cfg(test)]
pub(crate) mod tests {
    include!("tests/display.rs");
}

/// Handle of a display
pub type DisplayHandle = usize;

/// Refresh rate of display
pub type DisplayRefreshRate = u32;

/// Resolution expressed as width and height.
#[derive(Debug, PartialEq, Clone)]
pub struct DisplayResolution {
    /// Width resolution of the display.
    pub width : usize,

    /// Height resolution of the display.
    pub height : usize
}

/// Display physical size in millimeters.
#[derive(Debug, PartialEq, Clone)]
pub struct DisplaySizeMM {
    /// Width in millimeters of the display.
    pub width : usize,

    /// Height in millimeters of the display.
    pub height : usize
}

/// Current position of that display on the extended desktop
#[derive(Debug, PartialEq, Clone)]
pub struct DisplayDesktopPosition {
    /// X position of the display.
    pub x : i32,

    /// Y position of the display.
    pub y : i32
}

/// Contains informations about the desktop.
#[derive(Debug, PartialEq, Clone)]
pub struct Desktop {

    /// Minimum pixels resolution the desktop support
    pub min : DisplayResolution,

    /// Maximum pixels resolution the desktop support
    pub max : DisplayResolution,

    /// Current pixels resolution of the desktop
    pub current : DisplayResolution,


}

/// Contains list of all available display to show a [Window]
/// and information about the desktop.
#[derive(Debug)]
pub struct Displays {
    /// Inner list of screens
    pub list : Vec<Display>,

    /// Client desktop information
    pub desktop : Option<Desktop>,
}

impl Displays {
    /// Create a display list from combined resolution and vector of display.
    pub(crate) fn create(list : Vec<Display>, desktop : Option<Desktop>) -> Displays {
        Displays{ list, desktop }
    }

    /// Get primary display reference.
    /// 
    /// Returns Some([display]) with primary display or None if no primary display.
    pub fn primary(&self) -> Option<&Display> {

        self.list.iter().filter(|display| display.primary ).next()

    }

}



/// Details of a display.
/// 
/// # Note(s)
/// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
#[derive(Debug, PartialEq, Clone)]
pub struct Display {

    /// Handle of that display
    pub handle : DisplayHandle,

    /// Identifier of that display
    pub identifier : String,

    /// Physical size of display in millimeters
    pub size : DisplaySizeMM,

    /// Current position of that display on extended desktop
    pub position : DisplayDesktopPosition,

    /// Current resolution of that display
    pub resolution : DisplayResolution,

     /// Current refresh rate of that display
     pub refresh_rate : DisplayRefreshRate,
   
    /// Return true if primary display
    pub primary : bool,    

    /// Display supported resolutions with refresh rates
    pub supported : Vec<DisplaySupportedResolution>,

}

/// Hardware display supported resolution with available refresh rate for that [DisplayResolution].
/// 
/// # Note(s)
/// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
#[derive(Debug, PartialEq, Clone)]
pub struct DisplaySupportedResolution {

    /// Resolution supported by display
    pub resolution : DisplayResolution,

    /// If true, resolution is interlaced and not progressive.
    /// 
    /// Reference(s)
    /// <https://en.wikipedia.org/wiki/Interlaced_video>
    pub interlaced : bool,

    /// Refresh rate supported in that resolution.
    pub refresh_rates : Vec<DisplayRefreshRate>,
}

impl DisplaySupportedResolution {
    /// Create a new [ScreenResolution] with fields.
    pub fn new(resolution : DisplayResolution, interlaced : bool) -> DisplaySupportedResolution {
        DisplaySupportedResolution { resolution, interlaced, refresh_rates: Vec::new() }
    }

    /// Add a supported refresh rate to resolution.
    pub(crate) fn add_refresh_rate(&mut self, rate : DisplayRefreshRate){

        // Don't add if already exOksts
        for rf in &self.refresh_rates {
            if *rf == rate {
                return;
            }
        }

        self.refresh_rates.push(rate);
    }
   
}