use std::{mem::swap, ops::Range};

use drowsed_math::{self, FVec2, IVec2};

use super::Bitmap;

pub trait Antialiased {
    fn antialiased(&self, t: Self::TValue) -> Self;
    fn create_binary_tvalue(t: f32) -> Self::TValue;
    fn operate_mul(t: &mut Self::TValue, val: f32, range: Range<usize>);
    fn max_value() -> Self::TValue;
    fn min_value() -> Self::TValue;
    fn alpha(t: &mut Self::TValue, val: f32);
    type TValue;
}
#[derive(Clone, Copy, Default)]
pub struct RGBChannel(pub u8, pub u8, pub u8);
impl Antialiased for RGBChannel {
    fn antialiased(&self, t: Self::TValue) -> Self {
        let x =
        RGBChannel((self.0 as f32 * t.0) as u8,
        (self.1 as f32 * t.1) as u8,
        (self.2 as f32 * t.2) as u8,);
        // println!("{:?}", x);
        x
    }
    fn create_binary_tvalue(t: f32) -> Self::TValue {
        (t, t, t)
    }
    fn max_value() -> Self::TValue {
        (1.0, 1.0, 1.0)
    }
    fn min_value() -> Self::TValue {
        (0.0, 0.0, 0.0)
    }
    fn operate_mul(t: &mut Self::TValue, val: f32, range: Range<usize>) {
        let array = [&mut t.0, &mut t.1, &mut t.2];
        for i in range {
            *array[i] *= val;
        }
    }
    fn alpha(t: &mut Self::TValue, val: f32) {

    }
    type TValue = (f32, f32, f32);
}

#[derive(Clone, Copy, Default)]
pub struct RGBAChannel(pub u8, pub u8, pub u8, pub u8);
impl Antialiased for RGBAChannel {
    fn antialiased(&self, t: Self::TValue) -> Self {
        let x =
        RGBAChannel((self.0 as f32 * t.0) as u8,
        (self.1 as f32 * t.1) as u8,
        (self.2 as f32 * t.2) as u8,
        (self.3 as f32 * t.3) as u8);
        // println!("{:?}", x);
        x
    }
    fn create_binary_tvalue(t: f32) -> Self::TValue {
        (t, t, t, t)
    }
    fn max_value() -> Self::TValue {
        (1.0, 1.0, 1.0, 1.0)
    }
    fn min_value() -> Self::TValue {
        (0.0, 0.0, 0.0, 0.0)
    }
    fn operate_mul(t: &mut Self::TValue, val: f32, range: Range<usize>) {
        let array = [&mut t.0, &mut t.1, &mut t.2, &mut t.3];
        for i in range {
            *array[i] *= val;
        }
    }
    fn alpha(t: &mut Self::TValue, val: f32) {
        t.3 = val;
    }
    type TValue = (f32, f32, f32, f32);
}