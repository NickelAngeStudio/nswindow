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

/// Assert all subwindow default values.
fn assert_sub_default(sub : &super::SubWindow) {
    let anchor = super::SubWindowAnchor::new();

    assert!(sub.anchors == anchor);
    assert!(sub.block_parent_until_closed == super::BLOCK_PARENT);
    assert!(sub.minimize_with_parent == super::MIN_WPARENT);
    assert!(sub.restore_with_parent == super::RES_WPARENT);
    assert!(sub.show_with_parent == super::SHOW_WPARENT);
    assert!(sub.hide_with_parent == super::HIDE_WPARENT);
    assert!(sub.close_with_parent == super::CLOSE_WPARENT);
}

/// Assert all subwindow anchors default values
fn assert_subanchors_default(anchors : &super::SubWindowAnchor) {
    assert!(anchors.bottom_left == super::ANCHOR);
    assert!(anchors.bottom_right == super::ANCHOR);
    assert!(anchors.top_left == super::ANCHOR);
    assert!(anchors.top_right == super::ANCHOR);
}


/// Unit tests [super::SubWindow] default values.
#[test]
fn ut_subwindow_default() {
    assert_sub_default(&super::SubWindow::new());
}

/// Unit tests [super::SubWindow] reset.
#[test]
fn ut_subwindow_reset() {
    let mut sub = super::SubWindow::new();

    // Flip all value
    sub.block_parent_until_closed = !super::BLOCK_PARENT;
    sub.minimize_with_parent = !super::MIN_WPARENT;
    sub.restore_with_parent = !super::RES_WPARENT;
    sub.show_with_parent = !super::SHOW_WPARENT;
    sub.hide_with_parent = !super::HIDE_WPARENT;
    sub.close_with_parent = !super::CLOSE_WPARENT;

    // Reset
    sub.reset();

    // Retest defaults
    assert_sub_default(&sub);
}

/// Unit tests [super::SubWindowAnchor] default values.
#[test]
fn ut_subanchors_default() {
    assert_subanchors_default(&super::SubWindowAnchor::new());
}

/// Unit tests [super::SubWindowAnchor] value update.
#[test]
fn ut_subanchors_update() {
    let mut anchor = super::SubWindowAnchor::new();

    let bl = Some(crate::WindowPosition::new(1,1));
    let br = Some(crate::WindowPosition::new(2,2));
    let tl = Some(crate::WindowPosition::new(3,3));
    let tr = Some(crate::WindowPosition::new(4,4));

    // Update value
    anchor.set(bl, br, tl, tr);

    assert!(anchor.bottom_left == bl);
    assert!(anchor.bottom_right == br);
    assert!(anchor.top_left == tl);
    assert!(anchor.top_right == tr);

    // Clear anchors
    anchor.clear();

    // Retest
    assert_subanchors_default(&anchor);
}