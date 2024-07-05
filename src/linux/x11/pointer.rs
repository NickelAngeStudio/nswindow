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

use crate::pointer::{WindowCursor, WindowPointerMode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct X11Pointer {

}

impl X11Pointer {
    
    #[inline(always)]
    pub fn set_mode(&mut self, mode : WindowPointerMode) {
        todo!()
    }

    #[inline(always)]
    pub fn show(&mut self) {
        todo!()
    }

    #[inline(always)]
    pub fn hide(&mut self) {
        todo!()
    }

    #[inline(always)]
    pub fn confine(&mut self) {
        todo!()
    }

    #[inline(always)]
    pub fn release(&mut self) {
        todo!()
    }

    #[inline(always)]
    pub fn set_cursor(&mut self, cursor : WindowCursor) {
        todo!()
    }

}
