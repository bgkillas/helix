fn line_low(mut f: impl FnMut(isize, isize) -> bool, x0: isize, y0: isize, x1: isize, y1: isize) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut y = y0;
    if dx < 0 {
        let mut d = (2 * dy) + dx;
        for x in (x1..=x0).rev() {
            if f(x, y) {
                break;
            }
            if d > 0 {
                y += yi;
                d += 2 * (dy + dx);
            } else {
                d += 2 * dy;
            }
        }
    } else {
        let mut d = (2 * dy) - dx;
        for x in x0..=x1 {
            if f(x, y) {
                break;
            }
            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }
}
fn line_high(mut f: impl FnMut(isize, isize) -> bool, x0: isize, y0: isize, x1: isize, y1: isize) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut x = x0;
    if dy < 0 {
        let mut d = (2 * dx) + dy;
        for y in (y1..=y0).rev() {
            if f(x, y) {
                break;
            }
            if d > 0 {
                x += xi;
                d += 2 * (dx + dy);
            } else {
                d += 2 * dx;
            }
        }
    } else {
        let mut d = (2 * dx) - dy;
        for y in y0..=y1 {
            if f(x, y) {
                break;
            }
            if d > 0 {
                x += xi;
                d += 2 * (dx - dy);
            } else {
                d += 2 * dx;
            }
        }
    }
}
pub fn line(f: impl FnMut(isize, isize) -> bool, x0: isize, y0: isize, x1: isize, y1: isize) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    if dy.abs() < dx.abs() {
        line_low(f, x0, y0, x1, y1);
    } else {
        line_high(f, x0, y0, x1, y1);
    }
}
#[test]
fn test_line() {
    let arr = [(3, 2), (4, 2), (5, 3), (6, 3), (7, 4), (8, 4)];
    let mut iter = arr.iter().copied();
    line(
        |x, y| {
            let (nx, ny) = iter.next().unwrap();
            assert_eq!(x, nx, "{x} {y} {nx} {ny}");
            assert_eq!(y, ny, "{x} {y} {nx} {ny}");
            false
        },
        3,
        2,
        8,
        4,
    );
    let mut iter = arr.iter().copied().rev();
    line(
        |x, y| {
            let (nx, ny) = iter.next().unwrap();
            assert_eq!(x, nx, "{x} {y} {nx} {ny}");
            assert_eq!(y, ny, "{x} {y} {nx} {ny}");
            false
        },
        8,
        4,
        3,
        2,
    );
    let arr = [(2, 3), (2, 4), (3, 5), (3, 6), (4, 7), (4, 8)];
    let mut iter = arr.iter().copied();
    line(
        |x, y| {
            let (nx, ny) = iter.next().unwrap();
            assert_eq!(x, nx, "{x} {y} {nx} {ny}");
            assert_eq!(y, ny, "{x} {y} {nx} {ny}");
            false
        },
        2,
        3,
        4,
        8,
    );
    let mut iter = arr.iter().copied().rev();
    line(
        |x, y| {
            let (nx, ny) = iter.next().unwrap();
            assert_eq!(x, nx, "{x} {y} {nx} {ny}");
            assert_eq!(y, ny, "{x} {y} {nx} {ny}");
            false
        },
        4,
        8,
        2,
        3,
    );
}
