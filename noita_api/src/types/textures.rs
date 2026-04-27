#[repr(C)]
#[derive(Debug)]
pub struct Textures {}
#[repr(C)]
#[derive(Debug, Default)]
pub struct TextureInfo {
    width: i32,
    height: i32,
    unknown: i32,
    buffer: *const u8,
}
