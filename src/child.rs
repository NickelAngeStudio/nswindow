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

use crate::WindowPosition;


/// Anchors used to dock to parent [Window].
/// 
/// To dock without resize, specify the top_left anchor only.
pub struct WindowChildAnchor {
    pub bottom_left : Option<WindowPosition>,
    pub bottom_right : Option<WindowPosition>,
    pub top_left : Option<WindowPosition>,
    pub top_right : Option<WindowPosition>,
}

/// [Window] behaviour as child of parent.
pub struct WindowChildBehaviour {
    /// Anchors used to dock to parent [Window].
    anchors : WindowChildAnchor,

    /// Visible child block parent until hidden or closed. Also known as [modal window](https://en.wikipedia.org/wiki/Modal_window).
    /// 
    /// Parent [WindowManagerEvent](crate::event::WindowManagerEvent) will be blocked.
    block_parent_until_closed : bool,

    /// Minimize [Window] if parent is minimized. 
    /// 
    /// Overriden to true if any anchor is set.
    minimize_with_parent : bool,

    /// Restore [Window] if parent is retored. 
    /// 
    /// Overriden to true if any anchor is set.
    restore_with_parent : bool,

    /// Show [Window] if parent is shown. 
    show_with_parent : bool,

    /// Hide [Window] if parent is hidden. 
    hide_with_parent : bool,

    /// Close [Window] if parent is closed.
    /// 
    /// If set to false, [Window] parent will be removed if parent is closed.
    /// 
    /// Overriden to true if any anchor is set.
    close_with_parent : bool,

}

impl WindowChildBehaviour {
   
}