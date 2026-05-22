use crate::line::LineIter;
#[derive(Debug, Clone)]
pub struct ArcIter {
    low_line: LineIter,
    high_line: LineIter,
    range_x: isize,
    range_y_start: isize,
    range_y_end: isize,
    hx: isize,
    hy: isize,
    r2: isize,
}
impl Iterator for ArcIter {
    type Item = (isize, isize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.range_y_end >= self.range_y_start {
            return if self.range_x.cast_unsigned().is_multiple_of(2) {
                let y = self.range_y_start;
                self.range_y_start += 1;
                Some((self.range_x, y))
            } else {
                let y = self.range_y_end;
                self.range_y_end -= 1;
                Some((self.range_x, y))
            };
        }
        if let Some((_, hy)) = self.high_line.next() {
            let (lx, ly) = self.low_line.next().unwrap();
            self.range_x = lx;
            self.range_y_start = ly + 1;
            self.range_y_end = hy;
            Some((lx, ly))
        } else if let Some((lx, ly)) = self.low_line.next() {
            self.hx += 1;
            if self.hx * self.hx + self.hy * self.hy > self.r2 {
                self.hy -= 1;
            }
            self.range_x = lx;
            self.range_y_start = ly + 1;
            self.range_y_end = self.hy;
            Some((lx, ly))
        } else {
            None
        }
    }
}
impl ArcIter {
    #[inline]
    #[must_use]
    pub fn new(
        x0: isize,
        y0: isize,
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
        r2: isize,
    ) -> Self {
        let low_line = LineIter::new(x0, y0, x1, y1);
        Self {
            low_line,
            high_line: LineIter::new(x0, y0, x2, y2),
            range_x: 0,
            range_y_start: 0,
            range_y_end: -1,
            hx: x1,
            hy: x2,
            r2,
        }
    }
}
