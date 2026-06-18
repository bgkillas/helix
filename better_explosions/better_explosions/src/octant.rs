pub enum Octant {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}
#[inline]
pub fn octant(x0: usize, y0: usize, dx: usize, dy: usize, mut f: impl FnMut(Octant, usize, usize)) {
    f(Octant::Zero, x0 + dx, y0 + dy);
    f(Octant::One, x0 + dy, y0 + dx);
    f(Octant::Two, x0 - dy, y0 + dx);
    f(Octant::Three, x0 - dx, y0 + dy);
    f(Octant::Four, x0 - dx, y0 - dy);
    f(Octant::Five, x0 - dy, y0 - dx);
    f(Octant::Six, x0 + dy, y0 - dx);
    f(Octant::Seven, x0 + dx, y0 - dy);
}
#[test]
fn test_octant() {
    octant(10, 20, 1, 2, |i, x, y| {
        assert_eq!(
            (x, y),
            match i {
                Octant::Zero => (11, 22),
                Octant::One => (12, 21),
                Octant::Two => (8, 21),
                Octant::Three => (9, 22),
                Octant::Four => (9, 18),
                Octant::Five => (8, 19),
                Octant::Six => (12, 19),
                Octant::Seven => (11, 18),
            }
        );
    });
}
