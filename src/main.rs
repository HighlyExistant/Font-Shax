mod bitmap;
mod rasterizer;
mod font;
use drowsed_math::{USizeVec2, ULVec2, FVec2, smoothing::{lerp, remap}};
use font::atlas::FontAtlas;
use image::{ImageBuffer, RgbImage, RgbaImage};
use num_traits::One;
use crate::{bitmap::{Bitmap, pixels::{RGBChannel, Antialiased, RGBAChannel}, projection::{CartesianPlane, BitmapProjection}}, rasterizer::vectors::contained_sdf, font::FontShape};
fn main() {
    // let rgb = RGBAChannel(255, 255, 255, 255);
    // let data = std::fs::read("arial.ttf").unwrap();
    // let atlas = FontAtlas::new(vec!['a'], &data, 0.03);

    // let face = ttf_parser::Face::parse(&data, 0).unwrap();
    // let mut builder = FontShape {
    //     contours: vec![],
    //     scale: 0.03
    // };
    // let id = face.glyph_index('y').unwrap();
    // let bbox = face.outline_glyph(id, &mut builder).unwrap();
    // println!("{:?}", bbox);
    // let mut bmp = Bitmap::new(90, 91);
    // bmp.fill(&RGBAChannel(0, 0, 0, 0));
    // let t = lerp(90.0, 91.0, 0.5);
    // let origin = FVec2::new((bmp.width()/2)as f32, (bmp.height()/2)as f32);
    // let mut project = CartesianPlane::new(&mut bmp, origin);
    // let mut edgevec = vec![];
    // println!("{}", builder.contours.len());
    // for c in &builder.contours {
    //     for e in &c.edges {
    //         edgevec.push(e.as_ref());
    //     }
    // }
    // project.remapped_graph(-44..45, -44..45, |x, y|{
    //     if x == 13 && y == 39 {
    //         println!("debug break");
    //     }
    //     let (intesect, antialias) = contained_sdf::<RGBAChannel>(FVec2::new(x as f32, y as f32), &edgevec);
    //     (intesect, rgb.antialiased(antialias))
    // });

    // let mut img: RgbaImage = ImageBuffer::new(bmp.width() as u32, bmp.height() as u32);
    // img.copy_from_slice(bmp.as_byte_slice());
    // img.save("a.png").unwrap();

    
    let rgb = RGBAChannel(255, 255, 255, 255);
    let data = std::fs::read("arial.ttf").unwrap();
    let vector: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let atlas = FontAtlas::new(vector, &data, 0.03);

    let mut img: RgbaImage = ImageBuffer::new(atlas.bmp.width() as u32, atlas.bmp.height() as u32);
    img.copy_from_slice(atlas.bmp.as_byte_slice());
    img.save("b.png").unwrap();
    for (character, entry) in atlas.offsets {
        let xmin = remap(0.0, atlas.bmp.width() as f32, 0.0, 1.0 as f32, entry.bbox.0.start);
        let xmax = remap(0.0, atlas.bmp.width() as f32, 0.0, 1.0 as f32, entry.bbox.0.end);
        let ymin = remap(0.0, atlas.bmp.height() as f32, 0.0, 1.0 as f32, entry.bbox.1.start);
        let ymax = remap(0.0, atlas.bmp.height() as f32, 0.0, 1.0 as f32, entry.bbox.1.end);
        println!("{}:\txmin: {}, xmax: {}, ymin: {}, ymax: {}", character, xmin, xmax, ymin, ymax);
    }
    // let mut bmp = Bitmap::new(93, 95);
    // bmp.fill(&RGBAChannel(0, 0, 0, 0));
    // let origin = FVec2::new((bmp.width()/2) as f32, (bmp.height()/2) as f32);
    // let mut project = CartesianPlane::new(&mut bmp, origin);
    // project.debug_print_remap(&FVec2::new(-45.0, 45.0));
}
