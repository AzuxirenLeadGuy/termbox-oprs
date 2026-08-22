use crate::binding::{EventCode, TbEvent};

mod binding;

pub struct Runtime;

pub trait Application {
    fn run(&mut self, runtime: &Runtime) -> Result<(), i32>;
}

pub enum CursorPosition {
    Hidden,
    Visible { pos_x: i32, pos_y: i32 },
}

pub enum Event {
    None,
    Key {
        modifier: u8,
        keycode: u16,
        character: u32,
    },
    Resize {
        new_width: i32,
        new_height: i32,
    },
    Mouse {
        keycode: u16,
        pos_x: i32,
        pos_y: i32,
    },
}

impl Runtime {
    fn setup() -> Result<Self, i32> {
        let ret_val = unsafe { binding::tb_init() };
        return if ret_val == 0 {
            Ok(Runtime)
        } else {
            Err(ret_val)
        };
    }

    fn shutdown() {
        unsafe {
            binding::tb_shutdown();
        }
    }

    /// Clear the terminal screen
    pub fn clear(&self) {
        unsafe {
            binding::tb_clear();
        }
    }

    /// Clear the terminal screen with given settings
    pub fn clear_with(&self, fg_attr: u16, bg_attr: u16) {
        unsafe {
            binding::tb_set_clear_attributes(fg_attr, bg_attr);
        }
    }

    pub fn set_cursor(&self, position: CursorPosition) {
        unsafe {
            match position {
                CursorPosition::Hidden => binding::tb_hide_cursor(),
                CursorPosition::Visible { pos_x, pos_y } => binding::tb_set_cursor(pos_x, pos_y),
            }
        }
    }

    /// Get the size of the terminal screen
    pub fn size(&self) -> Option<(i32, i32)> {
        let height = unsafe { binding::tb_height() };
        let width = unsafe { binding::tb_width() };
        return if height <= 0 || width <= 0 {
            Option::None
        } else {
            Option::Some((width, height))
        };
    }

    /// Show the draw buffer to screen
    pub fn present(&self) {
        unsafe {
            binding::tb_present();
        }
    }

    pub fn set_cell(&self, pos_x: i32, pos_y: i32, char: char, fg_attr: u16, bg_attr: u16) {
        unsafe {
            binding::tb_change_cell(pos_x, pos_y, char.into(), fg_attr, bg_attr);
        }
    }

    pub fn write_cells(
        &self,
        pos_x: i32,
        pos_y: i32,
        string_val: &str,
        fg_attr: u16,
        bg_attr: u16,
    ) {
        let mut pos_x = pos_x;
        for char in string_val.chars() {
            self.set_cell(pos_x, pos_y, char, fg_attr, bg_attr);
            pos_x += 1;
        }
    }

    pub fn poll_event(&self) -> Event {
        let mut event = TbEvent::default();
        unsafe {
            binding::tb_poll_event(&mut event);
        }
        if event.event_type == EventCode::Key as u8 {
            return Event::Key {
                modifier: event.modifier,
                keycode: event.key,
                character: event.char_utf32,
            };
        } else if event.event_type == EventCode::Resize as u8 {
            return Event::Resize {
                new_width: event.width,
                new_height: event.height,
            };
        } else if event.event_type == EventCode::Mouse as u8 {
            return Event::Mouse {
                keycode: event.key,
                pos_x: event.pos_x,
                pos_y: event.pos_y,
            };
        } else {
            return Event::None;
        }
    }

    /// Run a termbox application
    pub fn launch<App: Application>(app: App) -> Result<(), i32> // TODO: Change err type to be more specific
    {
        let mut app: App = app;
        let runtime = match Self::setup() {
            Err(code) => return Err(code),
            Ok(runtime) => runtime,
        };
        let result = app.run(&runtime);
        Self::shutdown();
        return result;
    }
}

pub enum KeyType {
    F1 = 65535,
    F2 = 65534,
    F3 = 65533,
    F4 = 65532,
    F5 = 65531,
    F6 = 65530,
    F7 = 65529,
    F8 = 65528,
    F9 = 65527,
    F10 = 65526,
    F11 = 65525,
    F12 = 65524,
    Insert = 65523,
    Delete = 65522,
    Home = 65521,
    End = 65520,
    PageUp = 65519,
    PageDown = 65518,
    UpArrow = 65517,
    DownArrow = 65516,
    LeftArrow = 65515,
    RightArrow = 65514,
    MouseLeftButton = 65513,
    MouseRightButton = 65512,
    MouseMiddleButton = 65511,
    MouseRelease = 65510,
    MouseWheelUp = 65509,
    MouseWheelDown = 65508,
    CtrlTilde = 0,
    CtrlA = 1,
    CtrlB = 2,
    CtrlC = 3,
    CtrlD = 4,
    CtrlE = 5,
    CtrlF = 6,
    CtrlG = 7,
    Backspace = 8,
    Tab = 9,
    CtrlJ = 10,
    CtrlK = 11,
    CtrlL = 12,
    Enter = 13,
    CtrlN = 14,
    CtrlO = 15,
    CtrlP = 16,
    CtrlQ = 17,
    CtrlR = 18,
    CtrlS = 19,
    CtrlT = 20,
    CtrlU = 21,
    CtrlV = 22,
    CtrlW = 23,
    CtrlX = 24,
    CtrlY = 25,
    CtrlZ = 26,
    Escape = 27,
    Ctrl4 = 28,
    Ctrl5 = 29,
    Ctrl6 = 30,
    Ctrl7 = 31,
    Space = 32,
    Backspace2 = 127,
}
