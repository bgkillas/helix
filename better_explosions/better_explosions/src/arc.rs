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
            Some(self.next_range())
        } else if let Some((_, hy)) = self.high_line.next() {
            let (lx, ly) = self.low_line.next().unwrap();
            self.range_x = lx;
            self.range_y_start = ly;
            self.range_y_end = hy;
            Some(self.next_range())
        } else if let Some((lx, ly)) = self.low_line.next() {
            self.hx += self.low_line.sx;
            let yy = self.r2 - self.hx * self.hx;
            self.hy = yy.max(0).isqrt();
            self.range_x = lx;
            self.range_y_start = ly;
            self.range_y_end = self.hy;
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
        x0: isize,
        y0: isize,
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
        r2: isize,
    ) -> Self {
        Self {
            low_line: LineIter::new(x0, y0, x1, y1),
            high_line: LineIter::new(x0, y0, x2, y2),
            range_x: 0,
            range_y_start: 1,
            range_y_end: 0,
            hx: x2,
            hy: y2,
            r2,
        }
    }
    fn next_range(&mut self) -> (isize, isize) {
        if self.range_x.cast_unsigned().is_multiple_of(2) {
            let y = self.range_y_start;
            self.range_y_start += 1;
            (self.range_x, y)
        } else {
            let y = self.range_y_end;
            self.range_y_end -= 1;
            (self.range_x, y)
        }
    }
}
#[cfg(feature = "test")]
#[test]
fn test1() {
    use crate::truncate_f32;
    use image::RgbImage;
    use std::f32::consts::TAU;
    let mut image = RgbImage::new(65, 65);
    let rays: u16 = 32;
    for ray in 0..rays / 8 {
        let r = 64 - ray * 16;
        let r2: isize = usize::from(r).cast_signed().pow(2);
        let rf = f32::from(r);
        let delta_theta = TAU / f32::from(rays);
        let ix0 = 0;
        let iy0 = 0;
        let theta = f32::from(ray) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix3 = ix0 + truncate_f32(cos * rf);
        let iy3 = iy0 + truncate_f32(sin * rf);
        let theta = f32::from(ray + 1) * delta_theta;
        let (sin, cos) = theta.sin_cos();
        let ix4 = ix0 + truncate_f32(cos * rf);
        let iy4 = iy0 + truncate_f32(sin * rf);
        for (i, (x, y)) in ArcIter::new(ix0, iy0, ix3, iy3, ix4, iy4, r2).enumerate() {
            let p = &mut image
                .get_pixel_mut(x.try_into().unwrap(), y.try_into().unwrap())
                .0;
            p[0] = u8::try_from(i & 0xff).unwrap();
            p[1] = 64 * u8::try_from((i >> 8) & 0xff).unwrap();
            p[2] = 64;
        }
    }
    image.save("../../test1.png").unwrap();
}
#[cfg(feature = "test")]
#[test]
fn test2() {
    use crate::truncate_f32;
    use image::RgbImage;
    use std::f32::consts::TAU;
    let mut image = RgbImage::new(65, 65);
    let rays: u16 = 32;
    for r in (0..=64).rev() {
        for ray in 0..rays / 8 {
            let r2: isize = usize::from(r).cast_signed().pow(2);
            let rf = f32::from(r);
            let delta_theta = TAU / f32::from(rays);
            let ix0 = 0;
            let iy0 = 0;
            let theta = f32::from(ray) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix3 = ix0 + truncate_f32(cos * rf);
            let iy3 = iy0 + truncate_f32(sin * rf);
            let theta = f32::from(ray + 1) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix4 = ix0 + truncate_f32(cos * rf);
            let iy4 = iy0 + truncate_f32(sin * rf);
            for (x, y) in ArcIter::new(ix0, iy0, ix3, iy3, ix4, iy4, r2) {
                let p = &mut image
                    .get_pixel_mut(x.try_into().unwrap(), y.try_into().unwrap())
                    .0;
                p[0] = 3 * r;
                p[1] = 3 * r;
                p[2] = 3 * r;
            }
        }
    }
    image.save("../../test2.png").unwrap();
}
