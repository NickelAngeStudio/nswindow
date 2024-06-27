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
pub struct DisplaySize {
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

    /// Identifier of that display
    pub identifier : String,

    /// Physical size of display in millimeters
    pub size : DisplaySize,

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
    /// https://en.wikipedia.org/wiki/Interlaced_video
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


/*************
* UNIT TESTS * 
*************/

/// display unit tests
#[cfg(test)]
mod tests{
    use super::{Desktop, Display, DisplayDesktopPosition, DisplayResolution, DisplaySize, DisplaySupportedResolution, Displays, DisplayRefreshRate };

    fn create_empty_displays() -> Displays {
        let list : Vec<Display> = Vec::new();
        Displays::create(list, None)
    }

    /// Create [Displays] from an actual known source for tests.
    fn create_displays(primary_first : bool) -> Displays {

        let mut list : Vec<Display> = Vec::new();

        // First display
        let mut capacities : Vec<DisplaySupportedResolution> = Vec::new();
        capacities.push(create_supported(1920, 1080, vec![6003, 6001, 5997, 5996, 5993, 4799]));
        capacities.push(create_supported(1680,1050, vec![6000,5995,5988]));  
        capacities.push(create_supported(1400,1050, vec![5998,6000]));
        capacities.push(create_supported(1600,900, vec![5999,5994,5995,5982]));
        capacities.push(create_supported(1280,1024, vec![5995,6002]));
        capacities.push(create_supported(1400,900, vec![5996,5988])); 
        capacities.push(create_supported(1280,960, vec![6000,5999]));  
        capacities.push(create_supported(1440,810, vec![6000,5997]));  
        capacities.push(create_supported(1368,768, vec![5988,5985]));  
        capacities.push(create_supported(1280,800, vec![5999,5997,5981,5991]));
        capacities.push(create_supported(1152,864, vec![5997]));
        capacities.push(create_supported(1280,720, vec![6000,5999,5986,5974]));
        capacities.push(create_supported(1024,768, vec![6004,6000,5995]));
        capacities.push(create_supported(960,720, vec![6000]));
        capacities.push(create_supported(928,696, vec![6005]));  
        capacities.push(create_supported(896,672, vec![6001]));  
        capacities.push(create_supported(1024,576, vec![5995,5996,5990,5982]));
        capacities.push(create_supported(960,600, vec![5993,6000]));
        capacities.push(create_supported(960,540, vec![5996,5999,5963,5982]));
        capacities.push(create_supported(800,600, vec![6000,6032,5996,5625]));  
        capacities.push(create_supported(840,525, vec![6001,5988]));
        capacities.push(create_supported(864,486, vec![5992,5957]));  
        capacities.push(create_supported(700,525, vec![5998]));
        capacities.push(create_supported(800,450, vec![5995,5982]));  
        capacities.push(create_supported(640,512, vec![6002]));
        capacities.push(create_supported(700,450, vec![5996,5988]));  
        capacities.push(create_supported(640,480, vec![6000,5994]));  
        capacities.push(create_supported(720,405, vec![5951,5899]));  
        capacities.push(create_supported(684,384, vec![5988,5985]));  
        capacities.push(create_supported(640,400, vec![5988,5998]));  
        capacities.push(create_supported(640,360, vec![5986,5983,5984,5932]));  
        capacities.push(create_supported(512,384, vec![6000]));  
        capacities.push(create_supported(512,288, vec![6000,5992]));  
        capacities.push(create_supported(480,270, vec![5963,5982]));  
        capacities.push(create_supported(400,300, vec![6032,5634]));  
        capacities.push(create_supported(432,243, vec![5992,5957]));  
        capacities.push(create_supported(320,240, vec![6005]));  
        capacities.push(create_supported(360,202, vec![5951,5913]));  
        capacities.push(create_supported(320,180, vec![5984,5932]));  
        list.push(Display { identifier: "eDP-1".to_owned(), 
                            size: DisplaySize { width: 344, height: 194 }, 
                            position: DisplayDesktopPosition { x: 0, y: 1080 }, 
                            resolution: DisplayResolution { width: 1920, height: 1080 }, 
                            refresh_rate: 6003, 
                            primary: primary_first, 
                            supported: capacities});


        // Second display
        let mut capacities : Vec<DisplaySupportedResolution> = Vec::new();
        capacities.push(create_supported(1920,1080, vec![12000,6000,5000,5994,5993]));
        capacities.push(create_supported(1920,1200, vec![5995]));
        capacities.push(create_supported(1680,1050, vec![5995,5988]));  
        capacities.push(create_supported(1400,1050, vec![7476,5998]));  
        capacities.push(create_supported(1600,900, vec![5995,5982]));  
        capacities.push(create_supported(1280,1024, vec![7502,6002]));  
        capacities.push(create_supported(1440,900, vec![5990]));  
        capacities.push(create_supported(1400,900, vec![5996,5988]));  
        capacities.push(create_supported(1280,960, vec![6000]));  
        capacities.push(create_supported(1440,810, vec![5997]));  
        capacities.push(create_supported(1368,768, vec![5988,5985]));  
        capacities.push(create_supported(1280,800, vec![5997,5981,5991]));  
        capacities.push(create_supported(1152,864, vec![7500]));  
        capacities.push(create_supported(1280,720, vec![6000,5999,5986,6000,5000,5994,5974]));  
        capacities.push(create_supported(1024,768, vec![6004,7503,7007,6000]));  
        capacities.push(create_supported(960,720, vec![7500,6000]));  
        capacities.push(create_supported(928,696, vec![7500,6005]));  
        capacities.push(create_supported(896,672, vec![7505,6001]));  
        capacities.push(create_supported(1024,576, vec![5995,5996,5990,5982]));  
        capacities.push(create_supported(960,600, vec![5993,6000]));  
        capacities.push(create_supported(832,624, vec![7455]));  
        capacities.push(create_supported(960,540, vec![5996,5999,5963,5982]));  
        capacities.push(create_supported(800,600, vec![7500,7000,6500,6000,7219,7500,6032,5625]));  
        capacities.push(create_supported(840,525, vec![6001,5988]));  
        capacities.push(create_supported(864,486, vec![5992,5957]));  
        capacities.push(create_supported(720,576, vec![5000]));  
        capacities.push(create_supported(700,525, vec![7476,5998]));  
        capacities.push(create_supported(800,450, vec![5995,5982]));  
        capacities.push(create_supported(720,480, vec![6000,5994]));  
        capacities.push(create_supported(640,512, vec![7502,6002]));  
        capacities.push(create_supported(700,450, vec![5996,5988]));  
        capacities.push(create_supported(640,480, vec![6000,7500,7281,7500,6000,5994]));
        capacities.push(create_supported(720,405, vec![5951,5899]));  
        capacities.push(create_supported(720,400, vec![7008]));  
        capacities.push(create_supported(684,384, vec![5988,5985]));  
        capacities.push(create_supported(640,400, vec![5988,5998]));  
        capacities.push(create_supported(576,432, vec![7500]));  
        capacities.push(create_supported(640,360, vec![5986,5983,5984,5932]));  
        capacities.push(create_supported(512,384, vec![7503,7007,6000]));  
        capacities.push(create_supported(512,288, vec![6000,5992]));  
        capacities.push(create_supported(416,312, vec![7466]));  
        capacities.push(create_supported(480,270, vec![5963,5982]));  
        capacities.push(create_supported(400,300, vec![7219,7512,6032,5634]));
        capacities.push(create_supported(432,243, vec![5992,5957]));  
        capacities.push(create_supported(320,240, vec![7281,7500,6005]));  
        capacities.push(create_supported(360,202, vec![5951,5913]));  
        capacities.push(create_supported(320,180, vec![5984,5932]));   
        list.push(Display { identifier: "HDMI-1".to_owned(), 
                            size: DisplaySize { width: 510, height: 290 }, 
                            position: DisplayDesktopPosition { x: 0, y: 0 }, 
                            resolution: DisplayResolution { width: 1920, height: 1080 }, 
                            refresh_rate: 12000, 
                            primary: !primary_first, 
                            supported: capacities});

        Displays::create(list, Some(create_desktop()))

    }

    /// Shortcut to create a supported resolution
    fn create_supported(width : usize, height : usize, refreshs : Vec<DisplayRefreshRate>) -> DisplaySupportedResolution {
        DisplaySupportedResolution { 
            resolution: DisplayResolution { width, height }, 
            interlaced : false,
            refresh_rates: refreshs }
    }

    /// Create desktop from a known source.
    fn create_desktop() -> Desktop {

        Desktop {   min: super::DisplayResolution { width: 320, height: 200 }, 
                    max: super::DisplayResolution { width: 16384, height: 16384 }, 
                    current: super::DisplayResolution { width: 1920, height: 2160 } }
    }


    /// Unit tests empty Displays
    ///
    /// # Verification(s)
    /// V1 | Displays::primary() returns None
    /// V2 | displays.list.len() equal 0.
    /// V3 | displays.desktop is None.
    #[test]
    fn ut_empty_displays() {
        let displays = create_empty_displays();

        // V1 | Displays::primary() returns None"
        match displays.primary() {
            Some(_) => assert!(false, "Empty displays should not have a primary display!"),
            None => {},
        }

        // V2 | displays.list.len() equal 0
        assert!(displays.list.len() == 0);

        // V3 | displays.desktop is None.
        match displays.desktop {
            Some(_) => assert!(false, "Empty displays should not have a desktop!"),
            None => {},
        }

    }

    /// Unit tests Displays with datas
    ///
    /// # Verification(s)
    /// V1 | Displays created with datas succesfully.
    /// V2 | Displays::list::len() == 2
    /// V3 | Display::primary() match
    /// V4 | Display desktop min is 320, 200, max is 16384, 16384 and current is 1920, 2160.
    /// V5 | For each display, verify identifier, size, position, resolution, refresh_rate, primary, supported
    /// V6 | Test primary display set to second display.
    #[test]
    fn ut_displays() {

        // V1 | Displays created with datas succesfully.
        let displays = create_displays(true);


        // V2 | Displays::list::len() == 2
        assert!(displays.list.len() == 2);

        // V3 | Display::primary() match"
        match displays.primary() {
            Some(display) => assert!(display.identifier == "eDP-1"),
            None => assert!(false),
        }


        // V4 | Display desktop min is 320, 200, max is 16384, 16384 and current is 1920, 2160.
        match displays.desktop {
            Some(desktop) => {
                assert_eq!(desktop.min, DisplayResolution { width: 320, height: 200 });
                assert_eq!(desktop.max, DisplayResolution { width: 16384, height: 16384 });
                assert_eq!(desktop.current, DisplayResolution { width: 1920, height: 2160 });
            },
            None => assert!(false),
        }


        // V5 | For each display, verify identifier, size, position, resolution, refresh_rate, primary, supported
        displays.list.iter().for_each( |display|
            match display.identifier.as_str() {
                "eDP-1" => {
                    assert_eq!(display.size, DisplaySize { width: 344, height: 194 });
                    assert_eq!(display.position, DisplayDesktopPosition { x: 0, y: 1080 });
                    assert_eq!(display.resolution, DisplayResolution { width: 1920, height: 1080 });
                    assert_eq!(display.refresh_rate, 6003);
                    assert_eq!(display.primary, true);
                    assert_eq!(display.supported.len(), 39);
                },
                "HDMI-1" => {
                    assert_eq!(display.size, DisplaySize { width: 510, height: 290 });
                    assert_eq!(display.position, DisplayDesktopPosition { x: 0, y: 0 });
                    assert_eq!(display.resolution, DisplayResolution { width: 1920, height: 1080 });
                    assert_eq!(display.refresh_rate, 12000);
                    assert_eq!(display.primary, false);
                    assert_eq!(display.supported.len(), 47);
                }
                _ => assert!(false, "Unknown display identifier!"),
            }
            

        );
        
        // V6 | Test primary display set to second display.
        let displays = create_displays(false);
        assert_eq!(displays.primary().unwrap().identifier.as_str(), "HDMI-1");
    
    }

}