use std::ops::Range;

use drowsed_math::{USVec2, USizeVec2, FVec2, smoothing::remap, Vector, DVec2};

use super::Bitmap;

pub trait BitmapProjection<T: Copy + Default> {
    fn put_remapped_pixel(&mut self, projection: &Self::Projection, pixel: &T);
    fn remapped_graph<F>(&mut self, range_x: Range<isize>, range_y: Range<isize>, f: F)
        where F: Fn(isize, isize) -> (bool, T);
    type Projection: Vector;
}

pub struct CartesianPlane<'a, T: Copy + Default> {
    bmp: &'a mut Bitmap<T>,
    // this point decides when a number is positive or negative and the distance of said pixel
    origin: FVec2
}

impl<'a, T: Copy + Default> CartesianPlane<'a, T> {
    pub fn new(bmp: &'a mut Bitmap<T>, origin: FVec2) -> Self {
        Self { bmp, origin }
    }
    pub fn debug_print_remap(&self, projection: &FVec2) {
        let valuex = remap(0.0 , self.bmp.width as f32, (self.bmp.width as f32 - self.origin.x) as f32, self.bmp.width as f32 + self.origin.x, projection.x);
        let valuey = remap(0.0 , self.bmp.height as f32, (self.bmp.height as f32 - self.origin.y)as f32, self.bmp.height as f32 + self.origin.y, projection.y);
        println!("( x: {}, y: {} )", valuex, valuey);
    }
}
impl<'a, T: Copy + Default> BitmapProjection<T> for CartesianPlane<'a, T> {
    fn put_remapped_pixel(&mut self, projection: &Self::Projection, pixel: &T) {
        let valuex = self.origin.x + projection.x; // remap(0.0 , self.bmp.width as f32, (self.bmp.width as f32 - self.origin.x) as f32, self.bmp.width as f32 + self.origin.x, projection.x);
        let valuey = self.origin.y + projection.y; // remap(0.0 , self.bmp.height as f32, (self.bmp.height as f32 - self.origin.y)as f32, self.bmp.height as f32 + self.origin.y, projection.y);
        self.bmp.put_pixel(valuex as usize, valuey as usize, pixel);
    }
    fn remapped_graph<F>(&mut self, range_x: Range<isize>, range_y: Range<isize>, f: F)
        where F: Fn(isize, isize) -> (bool, T) {
        for y in range_y {
            for x in range_x.clone() {
                let (intersect, pixel) = f(x, y);
                if intersect {
                    self.put_remapped_pixel(&FVec2::new(x as f32, y as f32), &pixel);
                }
            }
        }
    }
    type Projection = FVec2;
}