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

/// SubWindow unit tests
#[cfg(test)]
pub(crate) mod tests {
    include!("tests/sub.rs");
}

use crate::WindowPosition;

// Defaults new SubWindow values
const BLOCK_PARENT : bool = false;
const MIN_WPARENT : bool = true;
const RES_WPARENT : bool = true;
const SHOW_WPARENT : bool = false;
const HIDE_WPARENT : bool = true;
const CLOSE_WPARENT : bool = true;
const ANCHOR : Option<WindowPosition> = None;

/// Anchors used to dock to parent [Window].
/// 
/// To dock without resize, specify the top_left anchor only.
#[derive(Debug, PartialEq, Clone)]
pub struct SubWindowAnchor {
    pub bottom_left : Option<WindowPosition>,
    pub bottom_right : Option<WindowPosition>,
    pub top_left : Option<WindowPosition>,
    pub top_right : Option<WindowPosition>,
}

impl SubWindowAnchor {
    pub(crate)  fn new() -> SubWindowAnchor {
        SubWindowAnchor { bottom_left: ANCHOR, bottom_right: ANCHOR, top_left: ANCHOR, top_right: ANCHOR }
    }

    /// Set all anchor in one call.
    pub fn set(&mut self, bottom_left : Option<WindowPosition>, bottom_right : Option<WindowPosition>, top_left : Option<WindowPosition>, top_right : Option<WindowPosition>) {
        self.bottom_left = bottom_left;
        self.bottom_right = bottom_right;
        self.top_left = top_left;
        self.top_right = top_right;
    }

    /// Clear all [SubWindow] anchors.
    pub fn clear(&mut self) {
        self.bottom_left = ANCHOR;
        self.bottom_right = ANCHOR;
        self.top_left = ANCHOR;
        self.top_right = ANCHOR;
    }
}

/// [SubWindow] properties as child of parent.
#[derive(Debug, PartialEq, Clone)]
pub struct SubWindow {
    /// Anchors used to dock to parent [Window].
    pub anchors : SubWindowAnchor,

    /// Shown sub window block parent until hidden or closed. Also known as [modal window](https://en.wikipedia.org/wiki/Modal_window).
    /// 
    /// Parent [WindowManagerEvent](crate::event::WindowManagerEvent) will be blocked.
    pub block_parent_until_closed : bool,

    /// Minimize [Window] if parent is minimized. 
    /// 
    /// Overriden to true if any anchor is set.
    pub minimize_with_parent : bool,

    /// Restore [Window] if parent is retored. 
    /// 
    /// Overriden to true if any anchor is set.
    pub restore_with_parent : bool,

    /// Show [Window] if parent is shown. 
    pub show_with_parent : bool,

    /// Hide [Window] if parent is hidden. 
    pub hide_with_parent : bool,

    /// Close [Window] if parent is closed.
    /// 
    /// If set to false, [Window] parent will be removed if parent is closed.
    /// 
    /// Overriden to true if any anchor is set.
    pub close_with_parent : bool,

}

impl SubWindow {
    /// Create a new [SubWindow] instance.
    pub fn new() -> SubWindow {
        SubWindow { 
            anchors: SubWindowAnchor::new(), 
            block_parent_until_closed: BLOCK_PARENT, 
            minimize_with_parent: MIN_WPARENT, 
            restore_with_parent: RES_WPARENT, 
            show_with_parent: SHOW_WPARENT, 
            hide_with_parent: HIDE_WPARENT, 
            close_with_parent: CLOSE_WPARENT }
    }

    /// Reset [SubWindow] to default parameters.
    pub fn reset(&mut self) {
        self.block_parent_until_closed = BLOCK_PARENT;
        self.minimize_with_parent = MIN_WPARENT;
        self.restore_with_parent = RES_WPARENT;
        self.show_with_parent = SHOW_WPARENT;
        self.hide_with_parent = HIDE_WPARENT;
        self.close_with_parent = CLOSE_WPARENT;
    }
}