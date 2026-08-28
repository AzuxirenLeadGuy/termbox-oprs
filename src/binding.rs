#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Attribute {
    Default = 0,
    Black = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Blue = 5,
    Magenta = 6,
    Cyan = 7,
    White = 8,
    Bold = 256,
    Underline = 512,
    Reverse = 1024,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EventCode {
    Key = 1,
    Resize = 2,
    Mouse = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Modifier {
    Alt = 1,
    Motion = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TbEvent {
    pub event_type: u8,
    pub modifier: u8,
    pub key: u16,
    pub char_utf32: u32,
    pub width: i32,
    pub height: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}

unsafe extern "C" {
    pub fn tb_init() -> i32;

    pub fn tb_shutdown();

    pub fn tb_width() -> i32;

    pub fn tb_height() -> i32;

    pub fn tb_clear();

    pub fn tb_set_clear_attributes(fg: u16, bg: u16);

    pub fn tb_present();

    pub fn tb_set_cursor(cx: i32, cy: i32);

    pub fn tb_hide_cursor();

    pub fn tb_change_cell(x: i32, y: i32, ch: u32, fg: u16, bg: u16);

    pub fn tb_poll_event(event: *mut TbEvent) -> i32;
}
