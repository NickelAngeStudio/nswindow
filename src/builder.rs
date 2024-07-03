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

use crate::{Window, WindowError, WindowFullScreenMode, WindowHandle, WindowKeyboard, WindowManager, WindowPointer, WindowRelativePosition, WindowSize};

/// Default title of [Window].
const WB_DEFAULT_TITLE : &str = "New window";

/// Default size of [Window].
const WB_DEFAULT_SIZE : WindowSize = WindowSize { width: 640, height: 480 };

/// Default position of [Window].
const WB_DEFAULT_POSITION : WindowRelativePosition = WindowRelativePosition::PrimaryCenter;

/// Default parent of [Window].
const WB_DEFAULT_PARENT : Option<WindowHandle> = None;

/// Default fullscreen mode of [Window].
const WB_DEFAULT_FSMODE : Option<WindowFullScreenMode> = None;

/// Default decoration toggle of [Window].
const WB_DEFAULT_DECORATION : bool = true;

/// Default minimized toggle of [Window].
const WB_DEFAULT_MINIMIZED : bool = false;

/// Default maximized toggle of [Window].
const WB_DEFAULT_MAXIMIZED : bool = false;

/// Default visibility toggle of [Window].
const WB_DEFAULT_VISIBLE : bool = true;


/// [WindowBuilder] used to create new [Window] or recreate an actual one.
pub struct WindowBuilder {

    /// Title of the [Window]
    title : String,

    /// Size of the [Window]
    size : WindowSize,

    /// Relative position of the [Window].
    position : WindowRelativePosition,

    /// Parent of the [Window].
    parent : Option<WindowHandle>,

    /// Keyboard properties for [Window].
    keyboard : WindowKeyboard,

    /// Pointer properties for [Window]
    pointer : WindowPointer,

    /// Fullscreen mode
    fsmode : Option<WindowFullScreenMode>,

    /// Show desktop frame around [Window].
    decoration : bool,

    /// Window minimized
    minimized : bool,

    /// Window maximized
    maximized : bool,

    /// Window will be showed on desktop when created.
    visible : bool,

}

impl WindowBuilder {

    /// Create a new [WindowBuilder] instance with default parameters. See 
    pub fn new() -> WindowBuilder {
        WindowBuilder { 
            title: WB_DEFAULT_TITLE.to_string(), 
            size: WB_DEFAULT_SIZE, 
            position: WB_DEFAULT_POSITION, 
            parent: WB_DEFAULT_PARENT, 
            keyboard: WindowKeyboard::new(), 
            pointer: WindowPointer::new(), 
            decoration: WB_DEFAULT_DECORATION,
            fsmode: WB_DEFAULT_FSMODE,
            minimized: WB_DEFAULT_MINIMIZED,
            maximized: WB_DEFAULT_MAXIMIZED, 
            visible : WB_DEFAULT_VISIBLE,
        }
    }

    /// Set the [Window] title of the new [Window].
    pub fn title(&mut self, title : &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    /// Set the [WindowSize] of the new window.
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowSizeOOB`] if `size` isn't between [Desktop::min](crate::display::Desktop::min) and [Desktop::max](crate::display::Desktop::max).
    pub fn size(&mut self, size : WindowSize) -> &mut Self {
        self.size = size;
        self
    }

    /// Set the [WindowRelativePosition] of the window to be build. 
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::WindowRelativePositionOOB`] if position would be out of desktop bound.
    pub fn position(&mut self, position : WindowRelativePosition) -> &mut Self {
        self.position = position;
        self
    }

    /// Override the default [WindowPointer].
    pub fn pointer(&mut self, pointer : WindowPointer) -> &mut Self {
        self.pointer = pointer;
        self
    }

    /// Override the default [WindowKeyboard].
    pub fn keyboard(&mut self, keyboard : WindowKeyboard) -> &mut Self {
        self.keyboard = keyboard;
        self
    }

    /// [WindowHandle] of the parent of the new [Window].
    /// 
    /// # Errors
    /// [WindowBuilder::build] returns [`WindowError::InvalidWindowHandle`] if [WindowHandle] doesn't refer to any [Window].
    /// [WindowBuilder::rebuild] returns [`WindowError::WindowParentSelf`] if [WindowHandle] if the same as the [Window] itself.
    /// [WindowBuilder::build] returns [`WindowError::WindowParentLoop`] if a parent loop would occur upon creation.
    pub fn parent(&mut self, parent : Option<WindowHandle>) -> &mut Self {
        self.parent = parent;
        self
    }
   
    /// Set the [Window] fullscreen mode. This mode is applied when [Window] is showed.
    pub fn fullscreen(&mut self, fsmode : Option<WindowFullScreenMode>) -> &mut Self {
        self.fsmode = fsmode;
        self
    }

    /// [Window] will be showed as minimized.
    pub fn minimized(&mut self) -> &mut Self {
        self.minimized = true;
        self
    }

    /// [Window] will be showed as maximized.
    pub fn maximized(&mut self) -> &mut Self {
        self.maximized = true;
        self
    }

    /// [Window] built will not be showed on desktop upon creation.
    /// 
    /// By default, [Window] created are visible.
    pub fn hide(&mut self) -> &mut Self {
        self.visible = false;
        self
    }

    /// Remove [Window] decoration on creation (Title bar, min and max buttons, etc...).
    pub fn no_decoration(&mut self) -> &mut Self {
        self.decoration = false;
        self
    }

    /// Reset the [WindowBuilder] with default values. 
    pub fn reset(&mut self) -> &mut Self {
        self.title = WB_DEFAULT_TITLE.to_string();
        self.size = WB_DEFAULT_SIZE;
        self.position = WB_DEFAULT_POSITION; 
        self.parent = WB_DEFAULT_PARENT;
        self.keyboard = WindowKeyboard::new(); 
        self.pointer = WindowPointer::new();
        self.decoration = WB_DEFAULT_DECORATION;
        self.fsmode = WB_DEFAULT_FSMODE;
        self.minimized = WB_DEFAULT_MINIMIZED;
        self.maximized = WB_DEFAULT_MAXIMIZED;
        self.visible = WB_DEFAULT_VISIBLE;
        self
    }

    /// Build a new [Window] within the given [WindowManager].
    pub fn build(&mut self, wm : &mut WindowManager) -> Result<WindowHandle, WindowError> {
        wm.build(self)
    }

    /// Rebuild a [Window] from the [WindowBuilder] parameters.
    pub fn rebuild(&mut self, w : &mut Window) -> Result<WindowHandle, WindowError> {
        w.rebuild(self)
    }

}



/*************
* UNIT TESTS * 
*************/
#[cfg(test)]
mod tests{
    use crate::{builder::{WB_DEFAULT_DECORATION, WB_DEFAULT_FSMODE, WB_DEFAULT_MAXIMIZED, WB_DEFAULT_MINIMIZED, WB_DEFAULT_PARENT, WB_DEFAULT_POSITION, WB_DEFAULT_SIZE, WB_DEFAULT_TITLE, WB_DEFAULT_VISIBLE}, WindowBuilder, WindowCursor, WindowFullScreenMode, WindowHandle, WindowKeyboard, WindowKeyboardMode, WindowPointer, WindowPointerMode, WindowPosition, WindowRelativePosition, WindowSize};

    /// Test default values of WindowBuilder
    fn test_defaults(wb : &WindowBuilder) {
        let wkb = WindowKeyboard::new();
        let wp = WindowPointer::new();

        assert!(wb.title == WB_DEFAULT_TITLE.to_string());
        assert!(wb.size == WB_DEFAULT_SIZE);
        assert!(wb.position == WB_DEFAULT_POSITION);
        assert!(wb.parent == WB_DEFAULT_PARENT);
        assert!(wb.fsmode == WB_DEFAULT_FSMODE);
        assert!(wb.decoration == WB_DEFAULT_DECORATION);
        assert!(wb.minimized == WB_DEFAULT_MINIMIZED);
        assert!(wb.maximized == WB_DEFAULT_MAXIMIZED);
        assert!(wb.visible == WB_DEFAULT_VISIBLE);
        assert!(wb.keyboard == wkb);
        assert!(wb.pointer == wp);

    }

    /// Unit tests [super::WindowBuilder] default values.
    ///
    /// # Verification(s)
    /// V1 | Test each value on creation vs default values. (except pointer and keyboard that are tested on their own)
    /// V2 | Test each pointer values.
    /// V3 | Test each keyboard values.
    #[test]
    fn ut_window_builder_default() {
        let wb = WindowBuilder::new();

        // V1 | Test each value on creation vs default values. (except pointer and keyboard that are tested on their own)
        test_defaults(&wb);

    }

    /// Unit tests [super::WindowBuilder] modification of values.
    ///
    /// # Verification(s)
    /// V1 | Allocate value to each parameter.
    /// V2 | Compare values VS Allocated values.
    /// V3 | Reset and test default values.
    #[test]
    fn ut_window_builder_modifications() {
        const TITLE : &str = "TEST TITLE";
        const SIZE : WindowSize = WindowSize { width: 123, height: 456 };
        const POSITION : WindowRelativePosition = WindowRelativePosition::Primary(WindowPosition { x: 987, y: 345 });

        const PARENTHANDLE : usize = 1;
        const PARENT : Option<WindowHandle> = Some(&PARENTHANDLE);
        const FSMODE : Option<WindowFullScreenMode> = Some(WindowFullScreenMode::Primary);

        const WKB_MODE : WindowKeyboardMode = WindowKeyboardMode::Text;
        const WKB_REPEAT : bool = true;

        const WP_MODE : WindowPointerMode = WindowPointerMode::Acceleration;
        const WP_CONFINED : bool = true;
        const WP_VISIBLE : bool = false;
        const WP_CURSOR : WindowCursor = WindowCursor::Move;

        let mut wkb = WindowKeyboard::new();
        wkb.mode = WKB_MODE;
        wkb.auto_repeat = WKB_REPEAT;

        let mut wp = WindowPointer::new();
        wp.mode = WP_MODE;
        wp.confined = WP_CONFINED;
        wp.visible = WP_VISIBLE;
        wp.cursor = WP_CURSOR;

        // V1 | Allocate value to each parameter.
        let mut wb = WindowBuilder::new();
                wb.title(TITLE)
                .size(SIZE)
                .position(POSITION)
                .pointer(wp)
                .keyboard(wkb)
                .parent(PARENT)
                .fullscreen(FSMODE)
                .minimized()
                .maximized()
                .hide()
                .no_decoration();

        
        // V2 | Compare values VS Allocated values.
        assert!(wb.title == TITLE.to_string());
        assert!(wb.size == SIZE);
        assert!(wb.position == POSITION);
        assert!(wb.parent == PARENT);
        assert!(wb.fsmode == FSMODE);
        assert!(wb.decoration == false);
        assert!(wb.minimized == true);
        assert!(wb.maximized == true);
        assert!(wb.visible == false);
        assert!(wb.keyboard == wkb);
        assert!(wb.pointer == wp);

        // V3 | Reset and test default values.
        wb.reset();
        test_defaults(&wb);

    }

}