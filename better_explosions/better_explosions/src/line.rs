#[derive(Debug, Clone)]
pub struct LineIter {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
    pub dx_neg: bool,
    pub dy_neg: bool,
    pub dx_abs: isize,
    pub dy_abs: isize,
    pub dx_abs_div2: isize,
    pub dy_abs_div2: isize,
    pub error: isize,
    pub first: bool,
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
            return Some((StepCase::Start, self.x0, self.y0));
        }
        match (
            self.error >= self.dy_abs_div2,
            self.error <= self.dx_abs_div2,
        ) {
            (true, true) => {
                if self.x0 == self.x1 || self.y0 == self.y1 {
                    return None;
                }
                self.error += self.dx_abs + self.dy_abs;
                self.sx();
                self.sy();
                Some((StepCase::Both, self.x0, self.y0))
            }
            (true, false) => {
                if self.x0 == self.x1 {
                    return None;
                }
                self.error += self.dy_abs;
                self.sx();
                Some((StepCase::Dx, self.x0, self.y0))
            }
            (false, true) => {
                if self.y0 == self.y1 {
                    return None;
                }
                self.error += self.dx_abs;
                self.sy();
                Some((StepCase::Dy, self.x0, self.y0))
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
                self.nsx();
                self.error -= self.dy_abs;
            }
            StepCase::Dy => {
                self.nsy();
                self.error -= self.dx_abs;
            }
            StepCase::Both => {
                self.nsx();
                self.nsy();
                self.error -= self.dx_abs + self.dy_abs;
            }
        }
    }
    fn sy(&mut self) {
        if self.dy_neg {
            self.y0 -= 1;
        } else {
            self.y0 += 1;
        }
    }
    fn sx(&mut self) {
        if self.dx_neg {
            self.x0 -= 1;
        } else {
            self.x0 += 1;
        }
    }
    fn nsy(&mut self) {
        if self.dy_neg {
            self.y0 += 1;
        } else {
            self.y0 -= 1;
        }
    }
    fn nsx(&mut self) {
        if self.dx_neg {
            self.x0 += 1;
        } else {
            self.x0 -= 1;
        }
    }
    #[inline]
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        let dx = x1.cast_signed() - x0.cast_signed();
        let dy = y1.cast_signed() - y0.cast_signed();
        let dx_abs = dx.abs();
        let dy_abs = -dy.abs();
        Self {
            x0,
            y0,
            x1,
            y1,
            dx_neg: dx.is_negative(),
            dy_neg: dy.is_negative(),
            dx_abs,
            dy_abs,
            dx_abs_div2: dx_abs / 2,
            dy_abs_div2: dy_abs / 2,
            error: dx_abs + dy_abs,
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
