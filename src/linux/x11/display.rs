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

use crate::{display::{self, Desktop, Display, DisplayDesktopPosition, DisplayRefreshRate, DisplayResolution, DisplaySize, DisplaySupportedResolution, Displays}, error::WindowError};

//use super::Display;

/// Get x11 Displays from xrandr
pub(crate) fn x11Displays() -> Result<Displays, WindowError> {

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
                    let mut list : Vec<Display> = Vec::new();
                    let mut desktop : Option<Desktop> = None;

                    for line in out_str.split("\n") {
                        if line.contains("Screen") { // Desktop entry
                            desktop = Some(fetch_desktop(line));
                        } else if line.contains("connected") && !line.contains("disconnected")  {  // Display entry
                            list.push(fetch_display(line));
                        } else if !line.contains("disconnected") && line.len() > 2 {  // Supported entry
                            let last = list.len() - 1;
                            fetch_supported(&mut list[last], line);
                        }

                    }

                    Ok(Displays::create(list, desktop))
             
                },
                Err(_) => Err(WindowError::DisplayInformationError),
            }
        },
        Err(_) => Err(WindowError::DisplayInformationError),
    }
}


/// Fetch desktop from line
fn fetch_desktop(line : &str) -> Desktop {

    let line = line.replace(",", "");
    let mut infos = line.split(" ");

    // Minimum
    let min  = DisplayResolution {  width : infos.nth(3).unwrap().parse::<usize>().unwrap(), 
                                                        height : infos.nth(1).unwrap().parse::<usize>().unwrap() };

    // Current
    let current  = DisplayResolution {  width : infos.nth(1).unwrap().parse::<usize>().unwrap(), 
        height : infos.nth(1).unwrap().parse::<usize>().unwrap() };

    // Maximum
    let max  = DisplayResolution {  width : infos.nth(1).unwrap().parse::<usize>().unwrap(), 
        height : infos.nth(1).unwrap().parse::<usize>().unwrap() };

    Desktop { min, max, current }

}

/// Fetch display from line
fn fetch_display(line : &str) -> Display {

    // Is display primary?
    let primary = line.contains("primary");

    let mut infos = line.split(" ");
    let identifier = infos.next().unwrap().to_string();

    // Position and resolution are on the same block WxH+X+Y
    let pos = if primary {
        2
    } else {
        1
    };
    let posres =   infos.nth(pos).unwrap();
    let posres = posres.replace("+", "x");  // Convert plus (+) to x
    let mut posres = posres.split("x");
    let resolution = DisplayResolution { width: posres.next().unwrap().parse::<usize>().unwrap() , height : posres.next().unwrap().parse::<usize>().unwrap() };
    let position = DisplayDesktopPosition { x: posres.next().unwrap().parse::<i32>().unwrap() , y : posres.next().unwrap().parse::<i32>().unwrap() };

    // Size is the last 2 information.
    while !infos.next().unwrap().contains(")") {};

    let size = DisplaySize { width: infos.next().unwrap().replace("mm", "").parse::<usize>().unwrap(),
        height: infos.nth(1).unwrap().replace("mm", "").parse::<usize>().unwrap() };

    Display { 
        identifier, 
        size, 
        position, 
        resolution, 
        refresh_rate: 0, 
        primary, 
        supported: Vec::new() 
    }
}

/// Fetch supported resolution from line
fn fetch_supported(display : &mut Display, line : &str) {

    
    let mut cleaned = line.trim().replace("  ", " ").replace(".", "");

    /// Remove all double spaces
    while cleaned.contains("  ") {
        cleaned = cleaned.replace("  ", " ");
    }

    let mut infos = cleaned.split(" ");

    // Resolution
    let res = infos.next().unwrap();

    // Interlaced video indicator
    let interlaced = res.contains("i");
    let res = res.replace("i", ""); // Remove interlaced indicator

    let mut res = res.split("x");

    let resolution = DisplayResolution { width : res.next().unwrap().parse::<usize>().unwrap() ,
         height : res.next().unwrap().parse::<usize>().unwrap() };

    let mut supported = DisplaySupportedResolution::new(resolution, interlaced);

    // Refresh rates
    loop {
            let rate = infos.next();

            match rate {
                Some(rate) => {
                     // Is it current refresh rate?
                    let is_display_rate = rate.contains("*");
    
                    // Remove symbols
                    let rate = rate.replace("*", "");
                    let rate = rate.replace("+", "");
            
                    let rate = rate.parse::<DisplayRefreshRate>().unwrap();
    
                    if is_display_rate {
                        display.refresh_rate = rate;
                    }
    
                    supported.add_refresh_rate(rate);
                },
                None => break,
            }
    }

    display.supported.push(supported);

}

/*************
* UNIT TESTS * 
*************/

/// X11 Screen fetch test
#[cfg(test)]
mod tests {



    #[test]
    fn x11screens_fetch() {

        // Only if x11 is supported
        if super::super::x11_supported() {
            let displays = super::x11Displays();

            println!("Display={:?}", displays);
        }

    }

}