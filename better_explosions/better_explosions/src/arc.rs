use crate::line::LineIter;
#[derive(Debug, Clone)]
pub struct ArcIter {
    low_line: LineIter,
    high_line: LineIter,
    range_x: isize,
    range_y_start: isize,
    range_y_end: isize,
    x0: isize,
    y0: isize,
    hx: isize,
    r2: isize,
    steep: bool,
}
impl Iterator for ArcIter {
    type Item = (usize, usize);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.range_y_end >= self.range_y_start {
            Some(self.next_range())
        } else if let Some((_, hy)) = self.high_line.next() {
            let (lx, ly) = self.low_line.next().unwrap();
            self.range_x = lx.cast_signed();
            if self.high_line.sy == -1 {
                self.range_y_end = ly.cast_signed();
                self.range_y_start = hy.cast_signed();
            } else {
                self.range_y_start = ly.cast_signed();
                self.range_y_end = hy.cast_signed();
            }
            Some(self.next_range())
        } else if let Some((lx, ly)) = self.low_line.next() {
            self.hx += self.low_line.sx;
            let yy = self.r2 - (self.hx - self.x0).pow(2);
            self.range_x = lx.cast_signed();
            if self.high_line.sy == -1 {
                self.range_y_end = ly.cast_signed();
                self.range_y_start = self.y0 - yy.isqrt();
            } else {
                self.range_y_start = ly.cast_signed();
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
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        r2: usize,
    ) -> Self {
        Self::newi(
            x0.cast_signed(),
            y0.cast_signed(),
            x1.cast_signed(),
            y1.cast_signed(),
            x2.cast_signed(),
            y2.cast_signed(),
            r2.cast_signed(),
        )
    }
    #[inline]
    #[must_use]
    pub fn newi(
        x0: isize,
        y0: isize,
        mut x1: isize,
        mut y1: isize,
        mut x2: isize,
        mut y2: isize,
        r2: isize,
    ) -> Self {
        if (x1 < x0 || x2 < x0) ^ (y1 < y0 || y2 < y0) {
            (x1, y1, x2, y2) = (x2, y2, x1, y1);
        }
        println!("{x0} {y0}");
        println!("{x1} {y1}");
        println!("{x2} {y2}");
        Self {
            low_line: LineIter::newi(x0, y0, x1, y1),
            high_line: LineIter::newi(x0, y0, x2, y2),
            range_x: 0,
            range_y_start: 1,
            range_y_end: 0,
            x0,
            y0,
            hx: x2,
            r2,
            steep: (y2 - y0).abs() > (x2 - x0).abs(),
        }
    }
    fn next_range(&mut self) -> (usize, usize) {
        if self.range_x.cast_unsigned().is_multiple_of(2) {
            let y = self.range_y_start;
            self.range_y_start += 1;
            (self.range_x.cast_unsigned(), y.cast_unsigned())
        } else {
            let y = self.range_y_end;
            self.range_y_end -= 1;
            (self.range_x.cast_unsigned(), y.cast_unsigned())
        }
    }
}
#[cfg(feature = "test")]
#[test]
fn test1() {
    use crate::round_f32;
    use image::RgbImage;
    use std::f32::consts::TAU;
    let mut image = RgbImage::new(65, 65);
    let rays: u16 = 32;
    let delta_theta = TAU / f32::from(rays);
    for ray in 0..rays / 8 {
        let r = 64 - ray * 16;
        let r2: usize = usize::from(r).pow(2);
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
    use crate::round_f32;
    use image::RgbImage;
    use std::f32::consts::TAU;
    let mut image = RgbImage::new(65, 65);
    let rays: u16 = 32;
    let delta_theta = TAU / f32::from(rays);
    for r in (0..=64).rev() {
        for ray in 0..rays / 8 {
            let r2: usize = usize::from(r).pow(2);
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
#[cfg(feature = "test")]
#[test]
fn test3() {
    use crate::round_f32;
    use image::RgbImage;
    use std::f32::consts::TAU;
    let mut image = RgbImage::new(256, 256);
    let rays: u16 = 32;
    let delta_theta = TAU / f32::from(rays);
    let ix0: usize = 128;
    let iy0: usize = 128;
    for r in (0..=64).rev() {
        for ray in 3 * rays / 8..4 * rays / 8 {
            let r2: usize = usize::from(r).pow(2);
            let rf = f32::from(r);
            let theta = f32::from(ray) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix3 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy3 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
            let theta = f32::from(ray + 1) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix4 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy4 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
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
    image.save("../../test3.png").unwrap();
}
#[cfg(feature = "test")]
#[test]
fn test4() {
    use crate::round_f32;
    use image::RgbImage;
    use std::f32::consts::TAU;
    let mut image = RgbImage::new(256, 256);
    let rays: u16 = 32;
    let delta_theta = TAU / f32::from(rays);
    let ix0: usize = 128;
    let iy0: usize = 128;
    for r in (0..=64).rev() {
        for ray in 4 * rays / 8..5 * rays / 8 {
            let r2: usize = usize::from(r).pow(2);
            let rf = f32::from(r);
            let theta = f32::from(ray) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix3 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy3 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
            let theta = f32::from(ray + 1) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix4 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy4 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
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
    image.save("../../test4.png").unwrap();
}
#[cfg(feature = "test")]
#[test]
fn test5() {
    use crate::round_f32;
    use image::RgbImage;
    use std::f32::consts::TAU;
    let mut image = RgbImage::new(256, 256);
    let rays: u16 = 32;
    let delta_theta = TAU / f32::from(rays);
    let ix0: usize = 128;
    let iy0: usize = 128;
    for r in (0..=64).rev() {
        for ray in 7 * rays / 8..rays {
            let r2: usize = usize::from(r).pow(2);
            let rf = f32::from(r);
            let theta = f32::from(ray) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix3 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy3 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
            let theta = f32::from(ray + 1) * delta_theta;
            let (sin, cos) = theta.sin_cos();
            let ix4 = (ix0.cast_signed() + round_f32(cos * rf)).cast_unsigned();
            let iy4 = (iy0.cast_signed() + round_f32(sin * rf)).cast_unsigned();
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
    image.save("../../test5.png").unwrap();
}
