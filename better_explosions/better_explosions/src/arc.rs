use crate::line::LineIter;
use std::ops::Range;
#[derive(Debug, Clone)]
pub struct ArcIter {
    low_line: LineIter,
    high_line: LineIter,
    range: Option<(Range<isize>, isize)>,
    is_high: bool,
}
impl Iterator for ArcIter {
    type Item = (isize, isize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((range, over)) = &mut self.range
            && let Some(next) = range.next()
        {
            Some(if self.is_high {
                (*over, next)
            } else {
                (next, *over)
            })
        } else if self.is_high {
            None
        } else {
            let mut next = self.low_line.next()?;
            while self.low_line.is_next_y_same() {
                next = self.low_line.next()?;
            }
            while self.high_line.is_next_y_same() {
                self.low_line.next()?;
            }
            let next_high = self.high_line.next()?;
            debug_assert_eq!(next.1, next_high.1);
            debug_assert!(next_high.0 <= next.0);
            self.range = Some((next_high.0..next.0, next.1));
            self.next()
        }
    }
}
impl ArcIter {
    pub fn new(x0: isize, y0: isize, x1: isize, y1: isize, x2: isize, y2: isize) -> Self {
        let low_line = LineIter::new(x0, y0, x1, y1);
        let is_high = low_line.is_high();
        Self {
            low_line,
            high_line: LineIter::new(x0, y0, x2, y2),
            range: None,
            is_high,
        }
    }
}
