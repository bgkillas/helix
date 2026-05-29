pub struct Circumference {
    x: isize,
    y: isize,
    error: isize,
}
impl Iterator for Circumference {
    type Item = (usize, usize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.y {
            None
        } else {
            let (x, y) = (self.x, self.y);
            self.y += 1;
            self.error += self.y;
            let e = self.error - x;
            if e.is_positive() {
                self.error = e;
                self.x -= 1;
            }
            Some((x.cast_unsigned(), y.cast_unsigned()))
        }
    }
}
impl Circumference {
    #[inline]
    #[must_use]
    pub fn new(r: usize) -> Self {
        Self {
            x: r.cast_signed(),
            y: 0,
            error: r.cast_signed() / 16,
        }
    }
}
