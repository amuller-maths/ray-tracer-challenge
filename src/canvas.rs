use std::ops::{Add, Mul, Sub};

use image::{ImageBuffer, Rgb, RgbImage};

use crate::macros::AlmostEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub f64, pub f64, pub f64);

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        let r = ((value.0 * 255.).round() as u8).clamp(0, 255);
        let g = ((value.1 * 255.).round() as u8).clamp(0, 255);
        let b = ((value.2 * 255.).round() as u8).clamp(0, 255);
        Rgb([r, g, b])
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        let [r, g, b] = value.0;
        let r = r as f64 / 255.;
        let g = g as f64 / 255.;
        let b = b as f64 / 255.;
        Color(r, g, b)
    }
}

impl Color {
    pub fn white() -> Self {
        Self(1., 1., 1.)
    }
    pub fn red() -> Self {
        Self(1., 0., 0.)
    }
    pub fn green() -> Self {
        Self(0., 1., 0.)
    }
    pub fn blue() -> Self {
        Self(0., 0., 1.)
    }
    pub fn black() -> Self {
        Self(0., 0., 0.)
    }
}

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: RgbImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32, color: Option<Color>) -> Self {
        let mut pixels = ImageBuffer::new(width, height);
        match color {
            None => {}
            Some(color) => {
                for j in 0..height {
                    for i in 0..width {
                        pixels.put_pixel(i, j, Rgb::from(color));
                    }
                }
            }
        }
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels.put_pixel(x, y, Rgb::from(color));
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        Color::from(self.pixels[(x, y)])
    }

    pub fn save(&self, path: &str) -> image::ImageResult<()> {
        self.pixels.save(path)
    }
}

impl AlmostEq for Color {
    fn almost_eq(self, other: Self, eps: f64) -> bool {
        (self.0 - other.0).abs() < eps
            && (self.1 - other.1).abs() < eps
            && (self.2 - other.2).abs() < eps
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_almost_eq;

    use super::*;
    #[test]
    fn adding_colors() {
        let a = Color(0.9, 0.6, 0.75);
        let b = Color(0.7, 0.1, 0.25);
        assert_almost_eq!(a + b, Color(1.6, 0.7, 1.));
    }

    #[test]
    fn substracting_colors() {
        let a = Color(0.9, 0.6, 0.75);
        let b = Color(0.7, 0.1, 0.25);
        assert_almost_eq!(a - b, Color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let a = Color(0.9, 0.6, 0.75);
        assert_almost_eq!(a * 2., Color(1.8, 1.2, 1.5));
    }

    #[test]
    fn multiplying_colors() {
        let a = Color(1., 0.2, 0.4);
        let b = Color(0.9, 1., 0.1);
        assert_almost_eq!(a * b, Color(0.9, 0.2, 0.04));
    }

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20, None);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for j in 0..c.height {
            for i in 0..c.width {
                assert_eq!(c.pixel_at(i, j), Color(0., 0., 0.))
            }
        }
    }

    #[test]
    fn creating_a_red_canvas() {
        let c = Canvas::new(10, 20, Some(Color(1., 0., 0.)));
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for j in 0..c.height {
            for i in 0..c.width {
                assert_eq!(c.pixel_at(i, j), Color(1., 0., 0.))
            }
        }
    }
    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20, None);
        let red = Color(1., 0., 0.);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn saving_a_canvas() {
        let c = Canvas::new(100, 100, Some(Color(1., 0., 0.)));
        c.save("img.png").unwrap();
    }
}
