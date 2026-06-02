#[derive(Debug, Clone)]
pub struct LineIter {
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    dx: isize,
    dy: isize,
    pub sx: isize,
    pub sy: isize,
    error: isize,
    first: bool,
}
#[derive(Debug)]
pub enum StepCase {
    Start,
    Dx,
    Dy,
    Both,
}
impl Iterator for LineIter {
    type Item = (StepCase, usize, usize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some((
                StepCase::Start,
                self.x0.cast_unsigned(),
                self.y0.cast_unsigned(),
            ));
        }
        match (self.error >= self.dy, self.error <= self.dx) {
            (true, true) => {
                if self.x0 == self.x1 || self.y0 == self.y1 {
                    return None;
                }
                self.error += 2 * (self.dx + self.dy);
                self.x0 += self.sx;
                self.y0 += self.sy;
                Some((
                    StepCase::Both,
                    self.x0.cast_unsigned(),
                    self.y0.cast_unsigned(),
                ))
            }
            (true, false) => {
                if self.x0 == self.x1 {
                    return None;
                }
                self.error += 2 * self.dy;
                self.x0 += self.sx;
                Some((
                    StepCase::Dx,
                    self.x0.cast_unsigned(),
                    self.y0.cast_unsigned(),
                ))
            }
            (false, true) => {
                if self.y0 == self.y1 {
                    return None;
                }
                self.error += 2 * self.dx;
                self.y0 += self.sy;
                Some((
                    StepCase::Dy,
                    self.x0.cast_unsigned(),
                    self.y0.cast_unsigned(),
                ))
            }
            (false, false) => unreachable!(),
        }
    }
}
impl LineIter {
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn back(&mut self, case: StepCase) {
        match case {
            StepCase::Start => {
                self.first = true;
            }
            StepCase::Dx => {
                self.x0 -= self.sx;
                self.error -= 2 * self.dy;
            }
            StepCase::Dy => {
                self.y0 -= self.sy;
                self.error -= 2 * self.dx;
            }
            StepCase::Both => {
                self.x0 -= self.sx;
                self.y0 -= self.sy;
                self.error -= 2 * (self.dx + self.dy);
            }
        }
    }
    #[inline]
    #[must_use]
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        Self::newi(
            x0.cast_signed(),
            y0.cast_signed(),
            x1.cast_signed(),
            y1.cast_signed(),
        )
    }
    #[inline]
    #[must_use]
    pub fn newi(x0: isize, y0: isize, x1: isize, y1: isize) -> Self {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        Self {
            x0,
            y0,
            x1,
            y1,
            dx,
            dy,
            sx: if x0 <= x1 { 1 } else { -1 },
            sy: if y0 <= y1 { 1 } else { -1 },
            error: 2 * (dx + dy),
            first: true,
        }
    }
}
#[test]
fn test_line() {
    let arr = [(3, 2), (4, 2), (5, 3), (6, 3), (7, 4), (8, 4)];
    let mut iter = arr.iter().copied();
    for (_, x, y) in LineIter::new(3, 2, 8, 4) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let mut iter = arr.iter().copied().rev();
    for (_, x, y) in LineIter::new(8, 4, 3, 2) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let arr = [(2, 3), (2, 4), (3, 5), (3, 6), (4, 7), (4, 8)];
    let mut iter = arr.iter().copied();
    for (_, x, y) in LineIter::new(2, 3, 4, 8) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let mut iter = arr.iter().copied().rev();
    for (_, x, y) in LineIter::new(4, 8, 2, 3) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                if i == k || (j == 0 && k == 8) {
                    continue;
                }
                let mut iter_a = LineIter::new(i, 0, j, 8).map(|(_, x, y)| (x, y));
                let mut iter_b = LineIter::new(j, 8, 0, k).map(|(_, x, y)| (x, y));
                let mut iter_c = LineIter::new(0, k, i, 0).map(|(_, x, y)| (x, y));
                let start_a = iter_a.next();
                let start_b = iter_b.next();
                let start_c = iter_c.next();
                assert_eq!(start_a, Some((i, 0)), "{i} {j} {k}");
                assert_eq!(start_b, Some((j, 8)), "{i} {j} {k}");
                assert_eq!(start_c, Some((0, k)), "{i} {j} {k}");
                assert_eq!(iter_a.last(), start_b, "{i} {j} {k}");
                assert_eq!(iter_b.last(), start_c, "{i} {j} {k}");
                assert_eq!(iter_c.last(), start_a, "{i} {j} {k}");
            }
        }
    }
}
