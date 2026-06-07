use noita_api_macros::assert_size_with;
#[repr(C)]
#[assert_size_with(0x10, f32)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AABB<T> {
    pub bottom_right: Vec2<T>,
    pub top_left: Vec2<T>,
}
#[repr(C)]
#[assert_size_with(0x8, isize)]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
#[repr(C)]
#[assert_size_with(0x8, f32)]
#[derive(Debug, Default, Clone)]
pub struct ValueRange<T> {
    pub min: T,
    pub max: T,
}
#[derive(Debug, Default)]
#[repr(C)]
#[assert_size_with(0x8, bool)]
#[assert_size_with(0xc, f32)]
pub struct LensValue<T> {
    pub value: T,
    pub valueb: T,
    pub frame: isize,
}
impl<T: PartialOrd> Vec2<T> {
    fn intersects(self, other: &Vec2<T>) -> bool {
        (self.x <= other.x && other.x < self.y) || (self.x <= other.y && other.y < self.y)
    }
    fn contains(self, other: &Vec2<T>) -> bool {
        self.x <= other.x && other.x < self.y && self.x <= other.y && other.y < self.y
    }
}
impl<T: PartialOrd> AABB<T> {
    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        let self_x_range = Vec2 {
            x: self.top_left.x,
            y: self.bottom_right.x,
        };
        let self_y_range = Vec2 {
            x: self.top_left.y,
            y: self.bottom_right.y,
        };
        let other_x_range = Vec2 {
            x: other.top_left.x,
            y: other.bottom_right.x,
        };
        let other_y_range = Vec2 {
            x: other.top_left.y,
            y: other.bottom_right.y,
        };
        self_x_range.intersects(&other_x_range) && self_y_range.intersects(&other_y_range)
    }
    #[inline]
    pub fn contains(self, other: Self) -> bool {
        let self_x_range = Vec2 {
            x: self.top_left.x,
            y: self.bottom_right.x,
        };
        let self_y_range = Vec2 {
            x: self.top_left.y,
            y: self.bottom_right.y,
        };
        let other_x_range = Vec2 {
            x: other.top_left.x,
            y: other.bottom_right.x,
        };
        let other_y_range = Vec2 {
            x: other.top_left.y,
            y: other.bottom_right.y,
        };
        self_x_range.contains(&other_x_range) && self_y_range.contains(&other_y_range)
    }
}
