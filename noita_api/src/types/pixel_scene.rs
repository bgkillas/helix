#[repr(C)]
#[derive(Debug)]
pub struct PixelScene {
    unknown1: i32,
    pub x: i32,
    pub y: i32,
    unknown2: [u8; 124],
    pub width: i32,
    pub height: i32,
}
