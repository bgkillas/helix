#[inline]
pub fn octant(x0: usize, y0: usize, dx: usize, dy: usize, mut f: impl FnMut(usize, usize, usize)) {
    f(0, x0 + dx, y0 + dy);
    f(1, x0 + dy, y0 + dx);
    f(2, x0 - dy, y0 + dx);
    f(3, x0 - dx, y0 + dy);
    f(4, x0 - dx, y0 - dy);
    f(5, x0 - dy, y0 - dx);
    f(6, x0 + dy, y0 - dx);
    f(7, x0 + dx, y0 - dy);
}
