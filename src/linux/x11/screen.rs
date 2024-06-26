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


use std::ptr::null_mut;

use crate::{error::NSWindowError, display::{DisplaySupportedResolution, Displays, Display}};

//use super::Display;

/// Get screen list in Screens
pub(crate) fn x11Screens() -> Result<Displays, NSWindowError> {

    todo!()
    //match xrandr_fetch() {
    //    Ok(screens) => Ok(Displays::create( screens)),
    //    Err(err) => Err(err),
    //}
}
/// Ugly function that fetch x11 display server screens.
fn xrandr_fetch() -> Result<Vec<Display>, NSWindowError>{
    use std::process::Command;

    // xrandr command list monitors with current resolution, available resolution and refresh rates.
    let cmd = Command::new("sh")
                .arg("-c")
                .arg("xrandr")
                .output();
            
    match cmd {
        Ok(output) => {
            let out_str = String::from_utf8(output.stdout);

            match out_str {
                Ok(out_str) => {
                    let mut screens : Vec<Display> = Vec::new();

                    for line in out_str.split("\n") {

                    }

                    Ok(screens)
                    
                    /*
                    let mut screens : Vec<Screen> = Vec::new();

                    // Used to get screen details
                    let mut sd = ScreenDetail::new();


                    for line in out_str.split("\n") {
                        // Remove all double spaces
                        let line_trimmed =  &line.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed = line_trimmed.trim();
                        
                        screen_section_end(line_trimmed, &mut screens, &mut sd);

                        match scan_connected_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_current_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_screen_supported_resolutions(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }
                    }
                    if sd.is_resolution_detail  {
                        // Section ended, add screen to list
                        screens.push(Screen::new( sd.identifier.clone(), (sd.pos_x, sd.pos_y),(sd.width, sd.height), sd.refresh_rate, 
                        sd.primary, sd.resolution.clone() ));
                    }

                    Ok((sd.desktop_width,sd.desktop_height,screens))
                    */
                },
                Err(_) => Err(NSWindowError::DisplayInformationError),
            }
        },
        Err(_) => Err(NSWindowError::DisplayInformationError),
    }
}

/*************
* UNIT TESTS * 
*************/

/// X11 Screen fetch test
#[cfg(test)]
mod tests {



    #[test]
    fn x11screens_fetch() {

        match super::x11Screens() {
            Ok(screens) => {
                println!("Screens : {:?}", screens);
            },
            Err(_) => assert!(false, "Error!"),
        }
    }

}

/*
/// Private struct used to fetch screen details
pub(crate) struct ScreenDetail {

    // Flag to know if we are in resolution details or not.
    pub is_resolution_detail : bool,

    // Desktop screen width and height
    pub desktop_width : u32,
    pub desktop_height : u32,

    // Screen details
    pub identifier : String,
    pub pos_x : i32,
    pub pos_y : i32,
    pub height : u32,
    pub width : u32,
    pub refresh_rate : u32,
    pub primary : bool,
    pub resolution : Vec<MonitorCapacity>,

    // Screen resolution details
    pub res_width : u32,
    pub res_height : u32,
}
/// Ugly function that fetch x11 display server screens.
pub(crate) fn fetch_screens_from_xrandr() -> Result<(u32,u32,Vec<Screen>), NSWindowError>{
    use std::process::Command;

    // xrandr command list monitors with current resolution, available resolution and refresh rates.
    let cmd = Command::new("sh")
                .arg("-c")
                .arg("xrandr")
                .output();
            
    match cmd {
        Ok(output) => {
            let out_str = String::from_utf8(output.stdout);

            match out_str {
                Ok(out_str) => {
                    let mut screens : Vec<Screen> = Vec::new();

                    // Used to get screen details
                    let mut sd = ScreenDetail::new();


                    for line in out_str.split("\n") {
                        // Remove all double spaces
                        let line_trimmed =  &line.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed = line_trimmed.trim();
                        
                        screen_section_end(line_trimmed, &mut screens, &mut sd);

                        match scan_connected_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_current_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_screen_supported_resolutions(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }
                    }
                    if sd.is_resolution_detail  {
                        // Section ended, add screen to list
                        screens.push(Screen::new( sd.identifier.clone(), (sd.pos_x, sd.pos_y),(sd.width, sd.height), sd.refresh_rate, 
                        sd.primary, sd.resolution.clone() ));
                    }

                    Ok((sd.desktop_width,sd.desktop_height,screens))
                },
                Err(_) => Err(NSWindowError::ScreenInformationError),
            }
        },
        Err(_) => Err(NSWindowError::ScreenInformationError),
    }
}

/// Ugly function that fetch x11 display server screens.
pub(crate) fn fetch_screens_from_xrandr() -> Result<(u32,u32,Vec<Screen>), NSWindowError>{
    use std::process::Command;

    // xrandr command list monitors with current resolution, available resolution and refresh rates.
    let cmd = Command::new("sh")
                .arg("-c")
                .arg("xrandr")
                .output();
            
    match cmd {
        Ok(output) => {
            let out_str = String::from_utf8(output.stdout);

            match out_str {
                Ok(out_str) => {
                    let mut screens : Vec<Screen> = Vec::new();

                    // Used to get screen details
                    let mut sd = ScreenDetail::new();


                    for line in out_str.split("\n") {
                        // Remove all double spaces
                        let line_trimmed =  &line.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed =  &line_trimmed.replace("  ", " ");
                        let line_trimmed = line_trimmed.trim();
                        
                        screen_section_end(line_trimmed, &mut screens, &mut sd);

                        match scan_connected_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_current_screen(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }

                        match scan_screen_supported_resolutions(line_trimmed, &mut sd){
                            Ok(_) => {},
                            Err(err) => return Err(err),
                        }
                    }
                    if sd.is_resolution_detail  {
                        // Section ended, add screen to list
                        screens.push(Screen::new( sd.identifier.clone(), (sd.pos_x, sd.pos_y),(sd.width, sd.height), sd.refresh_rate, 
                        sd.primary, sd.resolution.clone() ));
                    }

                    Ok((sd.desktop_width,sd.desktop_height,screens))
                },
                Err(_) => Err(NSWindowError::ScreenInformationError),
            }
        },
        Err(_) => Err(NSWindowError::ScreenInformationError),
    }
}

/// End of screen section
fn screen_section_end(line_trimmed:&str, screens : &mut Vec<Screen>, sd: &mut ScreenDetail){
    if sd.is_resolution_detail  {
        // Scan for section end
        if line_trimmed.contains("connected") {
            // Section ended, add screen to list
            screens.push(Screen::new( sd.identifier.clone(), (sd.pos_x, sd.pos_y),(sd.width, sd.height), sd.refresh_rate, 
                sd.primary, sd.resolution.clone() ));

            sd.is_resolution_detail = false;
        }
    }
}

/// Scan for connected screens
/// 
/// This kind of line : HDMI-0 connected 1920x1080+0+0 (normal left inverted right x axis y axis) 510mm x 290mm
fn scan_connected_screen(line_trimmed:&str, sd: &mut ScreenDetail) -> Result<u8, NSWindowError>{
    // Scan for connected monitor
    if line_trimmed.contains("connected") && !line_trimmed.contains("disconnected") {
        let mut line_split = line_trimmed.split(" ");
        
        // Get identifier
        match line_split.next(){
            Some(id) => sd.identifier = String::from(id),
            // If no id, return error
            None => return Err(NSWindowError::ScreenInformationError),
        }

        // Skip connected keyword
        line_split.next();

        // Get if primary screen
        if line_trimmed.contains("primary") {
            sd.primary = true;
            // Skip primary keyword
            line_split.next();
        } else {
            sd.primary = false;
        }

        // Get width and height
        match line_split.next() {
            Some(res) => {
                let mut res = res.split("x");

                // Get width
                match res.next() {
                    Some(w) => match w.parse::<u32>() {
                        Ok(w) => sd.width = w,
                        // If width conversion error, return error
                        Err(_) => return Err(NSWindowError::ScreenInformationError),
                    },
                    // If no width, return error
                    None => return Err(NSWindowError::ScreenInformationError),
                }

                // Get height
                match res.next() {
                    Some(h) => {
                        let mut h = h.split("+");

                         // Get height
                        match h.next() {
                            Some(h) => match h.parse::<u32>() {
                                Ok(h) => sd.height = h,
                                // If height conversion error, return error
                                Err(_) => return Err(NSWindowError::ScreenInformationError),
                            },
                            None => return Err(NSWindowError::ScreenInformationError),
                        }

                        // Get position x
                        match h.next() {
                            Some(x) => match x.parse::<i32>() {
                                Ok(x) => sd.pos_x = x,
                                // If x conversion error, return error
                                Err(_) => return Err(NSWindowError::ScreenInformationError),
                            },
                            None => return Err(NSWindowError::ScreenInformationError),
                        }

                        // Get position y
                        match h.next() {
                            Some(y) => match y.parse::<i32>() {
                                Ok(y) => sd.pos_y = y,
                                // If y conversion error, return error
                                Err(_) => return Err(NSWindowError::ScreenInformationError),
                            },
                            None => return Err(NSWindowError::ScreenInformationError),
                        }
                    },
                    None => return Err(NSWindowError::ScreenInformationError),
                }

                
            },
            // If no resolution, return error
            None => return Err(NSWindowError::ScreenInformationError),
        }

        sd.refresh_rate = 0;
        sd.resolution = Vec::new();
        sd.is_resolution_detail = true;
    }

    Ok(0)
}

/// Scan current screen to get desktop resolution
fn scan_current_screen(line_trimmed:&str, sd: &mut ScreenDetail) -> Result<u8, NSWindowError>{
    if line_trimmed.contains("current") {   // desktop screen details
        let mut line_split = line_trimmed.split(" ");

        loop {
            match line_split.next() {
                Some(part) => if part.contains("curr") {
                    break;  // We reached current, exit loop
                },
                None => return Err(NSWindowError::ScreenInformationError),
            }
        }

        // Get width
        match line_split.next() {
            Some(w) => match w.parse::<u32>() {
                Ok(w) => sd.desktop_width = w,
                // If height conversion error, return error
                Err(_) => return Err(NSWindowError::ScreenInformationError),
            },
            None => return Err(NSWindowError::ScreenInformationError),
        }

        // Skip x
        line_split.next();

        // Get height
        match line_split.next() {
            Some(h) => match h.replace(",", "").parse::<u32>() {
                Ok(h) => sd.desktop_height = h,
                // If height conversion error, return error
                Err(_) => return Err(NSWindowError::ScreenInformationError),
            },
            None => return Err(NSWindowError::ScreenInformationError),
        }
    }

    Ok(0)
}

/// Scan screen supported resolutions
fn scan_screen_supported_resolutions(line_trimmed:&str, sd: &mut ScreenDetail) -> Result<u8, NSWindowError>{

    if !line_trimmed.contains("connected") && sd.is_resolution_detail { // Resolution details
        let mut line_split = line_trimmed.split(" ");


        // Get width and height
        match line_split.next() {
            Some(res) => {
                let mut res = res.split("x");

                // Get width
                match res.next() {
                    Some(w) => match w.parse::<u32>() {
                        Ok(w) => sd.res_width = w,
                        // If width conversion error, return error
                        Err(_) => return Err(NSWindowError::ScreenInformationError),
                    },
                    None => return Err(NSWindowError::ScreenInformationError),
                }

                // Get height
                match res.next() {
                    Some(h) => match h.parse::<u32>() {
                        Ok(h) => sd.res_height = h,
                        // If width conversion error, return error
                        Err(_) => return Err(NSWindowError::ScreenInformationError),
                    },
                    None => return Err(NSWindowError::ScreenInformationError),
                }
            },
            None => return Err(NSWindowError::ScreenInformationError),
        }

        let mut res_res = MonitorCapacity::new(sd.res_width, sd.res_height);

        // To make sure we really get refresh counts.
        let mut rfcount = 0;

        // Get refresh rates
        loop {
            match line_split.next() {
                Some(rfp) => {
                    rfcount += 1;

                    // Remove extra characters
                    let rf = rfp.replace(".", "").replace("+", "").replace("*", "");

                    match rf.parse::<u32>() {
                        Ok(rf) => {
                            // Is it monitor refresh rate?
                            if rfp.contains("*"){
                                sd.refresh_rate = rf;
                            }

                            res_res.add_refresh_rate(rf);

                        },
                        // If width conversion error, return error
                        Err(_) => return Err(NSWindowError::ScreenInformationError),
                    };                                                
                },
                None => if rfcount > 0 {
                    break;
                } else {
                    // No refresh rate added, return error
                    return Err(NSWindowError::ScreenInformationError);
                },
            }
        }
        // Add to screen resolution
        sd.resolution.push(res_res.clone());

    }    

    Ok(0)
}
*/