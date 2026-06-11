use crate::line::LineIter;
#[derive(Debug, Clone)]
pub struct ArcIter {
    low_line: LineIter,
    high_line: LineIter,
    range_x: usize,
    range_y_start: usize,
    range_y_end: usize,
    x0: usize,
    y0: usize,
    hx: usize,
    r2: usize,
    steep: bool,
}
impl Iterator for ArcIter {
    type Item = (usize, usize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.range_y_end >= self.range_y_start {
            Some(self.next_range())
        } else if let Some((_, _, hy)) = self.high_line.next() {
            let (_, lx, ly) = self.low_line.next().unwrap();
            self.range_x = lx;
            if self.high_line.dy_neg {
                self.range_y_end = ly;
                self.range_y_start = hy;
            } else {
                self.range_y_start = ly;
                self.range_y_end = hy;
            }
            Some(self.next_range())
        } else if let Some((_, lx, ly)) = self.low_line.next() {
            if self.low_line.dx_neg {
                self.hx -= 1;
            } else {
                self.hx += 1;
            }
            let yy = self.r2 - self.hx.abs_diff(self.x0).pow(2);
            self.range_x = lx;
            if self.high_line.dy_neg {
                self.range_y_end = ly;
                self.range_y_start = self.y0 - yy.isqrt();
            } else {
                self.range_y_start = ly;
                self.range_y_end = self.y0 + yy.isqrt();
            }
            Some(self.next_range())
        } else {
            None
        }
    }
}
impl ArcIter {
    #[inline]
    #[must_use]
    pub fn new(
        mut x0: usize,
        mut y0: usize,
        mut x1: usize,
        mut y1: usize,
        mut x2: usize,
        mut y2: usize,
        r2: usize,
    ) -> Self {
        let steep = y2.abs_diff(y0) > x2.abs_diff(x0) || y1.abs_diff(y0) > x1.abs_diff(x0);
        if steep {
            (x0, y0) = (y0, x0);
            (x1, y1, x2, y2) = (y2, x2, y1, x1);
        }
        if (x1 < x0 || x2 < x0) ^ (y1 < y0 || y2 < y0) {
            (x1, y1, x2, y2) = (x2, y2, x1, y1);
        }
        Self {
            low_line: LineIter::new(x0, y0, x1, y1),
            high_line: LineIter::new(x0, y0, x2, y2),
            range_x: 0,
            range_y_start: 1,
            range_y_end: 0,
            x0,
            y0,
            hx: x2,
            r2,
            steep,
        }
    }
    fn next_range(&mut self) -> (usize, usize) {
        let y = self.range_y_start;
        self.range_y_start += 1;
        let x = self.range_x;
        if self.steep { (y, x) } else { (x, y) }
    }
}
#[cfg(feature = "test")]
#[test]
fn test() {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::as_conversions)]
    fn round_f32(f: f32) -> isize {
        f.round() as isize
    }
    use image::RgbImage;
    use std::f32::consts::TAU;
    let r: u16 = 256;
    let mut image = RgbImage::new(r.strict_cast::<u32>() + 8, r.strict_cast::<u32>() + 8);
    let rays: u16 = 64;
    let delta_theta = TAU / f32::from(rays);
    for ray in 0..rays / 4 {
        let r = r - ray * 16;
        let r2: usize = r.strict_cast::<usize>().pow(2);
        let rf = f32::from(r);
        let ix0: usize = 0;
        let iy0: usize = 0;
        let theta = f32::from(ray) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix3 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
        let iy3 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
        let theta = f32::from(ray + 1) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix4 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
        let iy4 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
        for (i, (x, y)) in ArcIter::new(ix0, iy0, ix3, iy3, ix4, iy4, r2).enumerate() {
            let p = &mut image
                .get_pixel_mut(x.try_into().unwrap(), y.try_into().unwrap())
                .0;
            p[0] = 128 + (i % 128).strict_cast::<u8>();
            p[1] = 128 + (i % 128).strict_cast::<u8>();
            p[2] = 128 + (i % 128).strict_cast::<u8>();
        }
    }
    image.save("../../test.png").unwrap();
}
