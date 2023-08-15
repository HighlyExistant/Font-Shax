use std::ops::Range;
pub mod projection;
pub mod pixels;
pub struct Bitmap<T: Copy + Default> {
    width: usize,
    height: usize,
    pixels: Vec<T>,
}

impl<T: Copy + Default> Bitmap<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![T::default(); width * height * std::mem::size_of::<T>()];
        Self { width, height, pixels }
    }
    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: &T) {
        if x > self.width { return; } else if y > self.height { return; }
        self.pixels[y * self.width + x] = *pixel;
    }
    pub fn fill(&mut self, pixel: &T) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, &pixel);
            }
        }
    }
    pub fn graph<F>(&mut self, range_x: Range<usize>, range_y: Range<usize>, f: F)
        where F: Fn(usize, usize) -> (bool, T) {
        for y in range_y {
            for x in range_x.clone() {
                let (intersect, pixel) = f(x, y);
                if intersect {
                    self.put_pixel(x, y, &pixel);
                }
            }
        }
    }
    pub fn as_slice(&self) -> &[T] { &self.pixels }
    pub fn as_byte_slice(&self) -> &[u8] { unsafe { std::mem::transmute::<&[T], &[u8]>(&self.pixels) } }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
}