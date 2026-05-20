#[derive(Debug, Clone)]
pub struct LineIter {
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    dx: isize,
    dy: isize,
    sx: isize,
    sy: isize,
    error: isize,
}
impl Iterator for LineIter {
    type Item = (isize, isize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ret = (self.x0, self.y0);
        let error = self.error;
        if error >= self.dy {
            if self.x0 == self.x1 {
                return None;
            }
            self.error += 2 * self.dy;
            self.x0 += self.sx;
        }
        if error <= self.dx {
            if self.y0 == self.y1 {
                return None;
            }
            self.error += 2 * self.dx;
            self.y0 += self.sy;
        }
        Some(ret)
    }
}
impl LineIter {
    #[inline]
    #[must_use]
    pub fn new(x0: isize, y0: isize, x1: isize, y1: isize) -> Self {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        Self {
            x0,
            y0,
            x1,
            y1,
            dx,
            dy,
            sx: if x0 < x1 { 1 } else { -1 },
            sy: if y0 < y1 { 1 } else { -1 },
            error: 2 * (dx + dy),
        }
    }
    #[inline]
    #[must_use]
    pub fn is_next_x_same(&self) -> bool {
        self.error < self.dy
    }
    #[inline]
    #[must_use]
    pub fn is_next_y_same(&self) -> bool {
        self.error > self.dx
    }
    #[inline]
    #[must_use]
    pub fn is_high(&self) -> bool {
        self.dx < -self.dy
    }
}
#[test]
fn test_line() {
    let arr = [(3, 2), (4, 2), (5, 3), (6, 3), (7, 4), (8, 4)];
    let mut iter = arr.iter().copied();
    for (x, y) in LineIter::new(3, 2, 8, 4) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let mut iter = arr.iter().copied().rev();
    for (x, y) in LineIter::new(8, 4, 3, 2) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let arr = [(2, 3), (2, 4), (3, 5), (3, 6), (4, 7), (4, 8)];
    let mut iter = arr.iter().copied();
    for (x, y) in LineIter::new(2, 3, 4, 8) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let mut iter = arr.iter().copied().rev();
    for (x, y) in LineIter::new(4, 8, 2, 3) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
}
