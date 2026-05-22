pub struct Octant {
    index: usize,
    x: isize,
    y: isize,
}
impl Iterator for Octant {
    type Item = (usize, isize, isize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 8 {
            None
        } else {
            let i = self.index;
            let x = self.x;
            let y = self.y;
            if i == 3 {
                (self.x, self.y) = (self.y, self.x);
            } else if i.is_multiple_of(2) {
                self.y = -self.y;
            } else {
                self.x = -self.x;
            }
            self.index += 1;
            Some((i, x, y))
        }
    }
}
impl Octant {
    #[inline]
    #[must_use]
    pub fn new(x: isize, y: isize) -> Self {
        Self { index: 0, x, y }
    }
}
#[test]
fn test() {
    for ox in 0..32 {
        for oy in 0..32 {
            for (i, x, y) in Octant::new(ox, oy) {
                assert_eq!(
                    (x, y),
                    match i {
                        0 => (ox, oy),
                        1 => (ox, -oy),
                        2 => (-ox, -oy),
                        3 => (-ox, oy),
                        4 => (oy, -ox),
                        5 => (oy, ox),
                        6 => (-oy, ox),
                        7 => (-oy, -ox),
                        _ => unreachable!(),
                    }
                )
            }
        }
    }
}
