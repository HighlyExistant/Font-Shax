use drowsed_math::{FVec2, QuadraticSegment, LinearSegment, Matrix4, FMat4};
use ttf_parser::OutlineBuilder;

use crate::rasterizer::windings::WindingOrder;
pub mod atlas;
pub struct Contour {
    pub edges: Vec<Box<dyn WindingOrder<VectorType = FVec2>>>,
    pub new: Option<FVec2>,
    pub start: Option<FVec2>,
}
pub struct FontShape {
    pub contours: Vec<Contour>,
    pub scale: f32,
}

impl FontShape {
    pub fn edge_count(&self) -> usize {
        let x = self.contours.iter().fold(0, |a, b|{ a + b.edges.len() });
        println!("{}", x);
        x
    }
    // pub fn scale(&mut self, scale: f32) {
    //     for contour in self.contours.iter_mut() {
    //         for edge in contour.edges.iter_mut() {
    //             edge.mul_scalar(scale);
    //         }
    //     }
    // }
}


// const SCALE: f32 = 0.03;
impl OutlineBuilder for FontShape {
    fn move_to(&mut self, x: f32, y: f32) {
        self.contours.push(Contour { edges: vec![], new: None, start: None });
        let contour = self.contours.last_mut().unwrap();
        contour.new = Some(FVec2::new(x, y) * self.scale);
        contour.start = Some(FVec2::new(x, y) * self.scale);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let contour = self.contours.last_mut().unwrap();
        if let Some(point) = contour.new {
            contour.edges.push(Box::new(LinearSegment::new(point, FVec2::new(x, y)* self.scale)));
            contour.new = Some(FVec2::new(x, y)* self.scale);
        } else {
            contour.new = Some(FVec2::new(x, y)* self.scale);
        }
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let contour = self.contours.last_mut().unwrap();
        if let Some(point) = contour.new {
            contour.edges.push(Box::new(QuadraticSegment::new(point, FVec2::new(x1, y1)* self.scale, FVec2::new(x, y)* self.scale)));
            contour.new = Some(FVec2::new(x, y)* self.scale);
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // write!(&mut self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap();
    }

    fn close(&mut self) {
        let contour = self.contours.last_mut().unwrap();
        let start = contour.start.unwrap();
        let new = contour.new.unwrap();
        contour.edges.push(Box::new(LinearSegment::new(new, start)));
    }
}

// impl Contour {
//     pub fn transform(&mut self, mat4x4: &FMat4) {
//         for edge in &mut self.edges {
//             edge.iter_mut().map(|p|{
//                 p = *p * *mat4x4;
//             });
//         }
//     }
// }