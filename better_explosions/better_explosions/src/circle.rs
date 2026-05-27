use crate::circumference::Circumference;
use std::ops::Range;
pub struct Circle {
    x0: usize,
    y0: usize,
    octant: u8,
    dx: usize,
    dy: usize,
    y: usize,
    range: Range<usize>,
    circumference: Circumference,
}
impl Iterator for Circle {
    type Item = (usize, usize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.range.next() {
            Some((x, self.y))
        } else if self.octant < 3 {
            self.octant += 1;
            match self.octant {
                1 => {
                    self.range = self.x0 - self.dx..self.x0 + self.dx;
                    self.y = self.y0 - self.dy;
                    Some((self.x0 + self.dx, self.y))
                }
                2 => {
                    self.range = self.x0 - self.dy..self.x0 + self.dy;
                    self.y = self.y0 + self.dx;
                    Some((self.x0 + self.dy, self.y))
                }
                3 => {
                    self.range = self.x0 - self.dy..self.x0 + self.dy;
                    self.y = self.y0 - self.dx;
                    Some((self.x0 + self.dy, self.y))
                }
                _ => unreachable!(),
            }
        } else if let Some((dx, dy)) = self.circumference.next() {
            self.dx = dx;
            self.dy = dy;
            self.range = self.x0 - dx..self.x0 + dx;
            self.y = self.y0 + dy;
            self.octant = 0;
            Some((self.x0 + dx, self.y))
        } else {
            None
        }
    }
}
impl Circle {
    #[inline]
    #[must_use]
    pub fn new(x0: usize, y0: usize, r: usize) -> Self {
        Self {
            x0,
            y0,
            dx: 0,
            dy: 0,
            octant: u8::MAX,
            y: 0,
            #[allow(clippy::reversed_empty_ranges)]
            range: 1..0,
            circumference: Circumference::new(r),
        }
    }
}
