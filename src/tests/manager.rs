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

/// WindowManager unit tests

use std::{i32, u32};

use crate::{ WindowBuilder, WindowError, WindowHandle, WindowManager};

/// WindowManager::build() unit test
#[test]
fn window_manager_ut_build() {
    match WindowManager::new() {
        Ok(mut wm) => {
            match WindowBuilder::new().build(&mut wm){
                Ok(_) => { },
                Err(err) => panic!("{:?}", err),
            }
        },
        Err(err) => assert!(false, "{:?}", err),
    }
}

/// WindowManager::event() unit test
#[test]
fn window_manager_ut_event() {
    match WindowManager::new() {
        Ok(mut wm) => {
            wm.event();
        },
        Err(err) => assert!(false, "{:?}", err),
    }
}

/// WindowManager::window() and WindowManager::window_mut() unit test
#[test]
fn window_manager_ut_window() {
    match WindowManager::new() {
        Ok(mut wm) => {
            match WindowBuilder::new().build(&mut wm){
                Ok(wh) => {
                    match wm.window(wh){
                        Ok(_) => {},
                        Err(err) => panic!("{:?}", err),
                    }
                    
                    match wm.window_mut(wh){
                        Ok(_) => {},
                        Err(err) => panic!("{:?}", err),
                    }

                    },
                Err(err) => panic!("{:?}", err),
            }
        },
        Err(err) => assert!(false, "{:?}", err),
    }
}

/// WindowManager::displays() unit test
#[test]
fn window_manager_ut_displays() {
    match WindowManager::new() {
        Ok(wm) => {
            wm.displays();
        },
        Err(err) => assert!(false, "{:?}", err),
    }
}

/// WindowManager::build() error unit tests
#[test]
fn window_manager_ut_build_error() {
    match WindowManager::new() {
        Ok(mut wm) => {
            let invalid_handle : WindowHandle = &0;

            // WindowRelativePositionOOB
            match WindowBuilder::new().hide()
                .position(crate::WindowRelativePosition::Desktop(crate::WindowPosition { x: i32::MAX, y: i32::MAX }))
                .build(&mut wm) {
                Ok(_) => panic!("WindowRelativePositionOOB expected"),
                Err(err) => assert!(err == WindowError::WindowRelativePositionOOB),
            } 

            // WindowSizeOOB
            match WindowBuilder::new().hide()
                .size(crate::WindowSize { width: u32::MAX, height: u32::MAX })
                .build(&mut wm) {
                Ok(_) => panic!("WindowSizeOOB expected"),
                Err(err) => assert!(err == WindowError::WindowSizeOOB),
            } 

            // WindowMinSizeBiggerThanMax
            match WindowBuilder::new().hide()
                .size_min(crate::WindowSize { width: u32::MAX, height: u32::MAX })
                .build(&mut wm) {
                Ok(_) => panic!("WindowMinSizeBiggerThanMax expected"),
                Err(err) => assert!(err == WindowError::WindowMinSizeBiggerThanMax),
            } 

            nscfg::match_cfg! {
                !single_opt:ft => {
                    // InvalidWindowHandle for parent
                    match WindowBuilder::new().hide()
                    .parent(Some(invalid_handle))
                    .build(&mut wm) {
                        Ok(_) => panic!("WindowMinSizeBiggerThanMax expected"),
                        Err(err) => assert!(err == WindowError::WindowMinSizeBiggerThanMax),
                        }
                },
                _ => {}
            }
            
            
        },
        Err(err) => assert!(false, "{:?}", err),
    }

}

/// WindowManager::window() and window_mut() error unit tests
#[test]
fn window_manager_ut_window_error() {
    match WindowManager::new() {
        Ok(mut wm) => {
            let invalid_handle : WindowHandle = &0;
            assert!(wm.window(invalid_handle).expect_err("InvalidWindowHandle expected!") == WindowError::InvalidWindowHandle);
            assert!(wm.window_mut(invalid_handle).expect_err("InvalidWindowHandle expected!") == WindowError::InvalidWindowHandle);
        },
        Err(err) => assert!(false, "{:?}", err),
    }
}
