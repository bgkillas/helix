#[derive(Debug, Clone)]
pub struct LineIter {
    x0: isize,
    y0: isize,
    x1: isize,
    dx: isize,
    dy: isize,
    error: isize,
}
impl Iterator for LineIter {
    type Item = (isize, isize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x0 == self.x1 + 1 {
            return None;
        }
        let (x, y) = (self.x0, self.y0);
        self.x0 += 1;
        if self.error > 0 {
            self.y0 += 1;
            self.error += 2 * (self.dy - self.dx);
        } else {
            self.error += 2 * self.dy;
        }
        Some((x, y))
    }
}
impl LineIter {
    #[inline]
    #[must_use]
    pub fn new(x0: isize, y0: isize, x1: isize, y1: isize) -> Self {
        let dx = x1 - x0;
        let dy = y1 - y0;
        Self {
            x0,
            y0,
            x1,
            dx,
            dy,
            error: 2 * dy - dx,
        }
    }
}
#[test]
fn test() {
    for b in 0..=16 {
        for i in 0..=b {
            let mut iter = LineIter::new(0, 0, b, i);
            assert_eq!(iter.next(), Some((0, 0)));
            if b != 0 {
                assert_eq!(iter.last(), Some((b, i)));
            }
        }
    }
}
