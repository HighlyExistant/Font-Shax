use std::collections::HashMap;

use drowsed_math::{USizeVec2, FVec2};

use crate::{bitmap::{pixels::{RGBAChannel, Antialiased}, Bitmap, projection::{CartesianPlane, BitmapProjection}}, rasterizer::vectors::contained_sdf};

use super::FontShape;

pub struct FontAtlas {
    pub offsets: HashMap<char, usize>,
    pub bmp: Bitmap<RGBAChannel>,
}

impl FontAtlas {
    pub fn new(letters: Vec<char>, data: &Vec<u8>, scale: f32) -> Self {
        let mut offsets: HashMap<char, usize> = HashMap::new();
        let rgb = RGBAChannel(255, 255, 255, 255);
        let face = ttf_parser::Face::parse(data, 0).unwrap();
        let mut total_offset = 0;
        let ascender =  (face.ascender() as f32 * scale) as i16;
        let descender = (face.descender() as f32 * scale) as i16; 
        let linegap =   (face.line_gap() as f32 * scale) as i16;
        let bmpsize = letters.iter().fold(USizeVec2::new(0,0), |a, b| {
            let id = face.glyph_index(*b).unwrap();
            let mut bbox = face.glyph_bounding_box(id).unwrap();
            
            if *b == 'y' {
                println!("debug break");
            }
            bbox.x_max = (bbox.x_max as f32 * scale) as i16;
            bbox.x_min = (bbox.x_min as f32 * scale) as i16;
            bbox.y_max = (bbox.y_max as f32 * scale) as i16;
            bbox.y_min = (bbox.y_min as f32 * scale) as i16;
            println!("ascender {}, descender {}, linegap {}", ascender, descender, linegap);
            let ysize = (ascender - descender) as usize;
            
            let advance = (face.glyph_hor_advance(id).unwrap() as f32 * scale) as i16;
            let y_dist = (bbox.y_max - bbox.y_min).abs();
            let size = USizeVec2::new(advance.abs() as usize,
            y_dist as usize);
            if size.y > a.y {
                USizeVec2::new(a.x + size.x,ysize)
                // USizeVec2::new(a.x + size.x,size.y)
            } else {
                USizeVec2::new(a.x + size.x,ysize)
                // USizeVec2::new(a.x + size.x,a.y)
            }
        });
        
        // let bmpsize = USizeVec2::new(bmpsize.x, bmpsize.y);
        let mut bmp = Bitmap::<RGBAChannel>::new(bmpsize.x as usize, bmpsize.y);
        bmp.fill(&RGBAChannel(0, 0, 0, 0));
        for letter in letters {
            
            let mut builder = FontShape {
                contours: vec![],
                scale
            };
            let id = face.glyph_index(letter).unwrap();
            let mut bbox = face.outline_glyph(id, &mut builder).unwrap();
            bbox.x_max = (bbox.x_max as f32 * scale) as i16;
            bbox.x_min = (bbox.x_min as f32 * scale) as i16;
            bbox.y_max = (bbox.y_max as f32 * scale) as i16;
            bbox.y_min = (bbox.y_min as f32 * scale) as i16;

            offsets.insert(letter, total_offset);

            let bbox_x_dist = (bbox.x_max - bbox.x_min).abs() as usize;
            let bbox_y_dist = (bbox.y_max - bbox.y_min).abs() as usize;
            // total_offset += bmp.width()/2;
            // let y_offset = bmp.height()/2;
            let y_offset = bmpsize.y - descender.abs() as usize;
            let origin = FVec2::new(
                (total_offset) as f32, 
                (y_offset) as f32
            );
            total_offset += bbox_x_dist;
            // let origin = FVec2::new(
            //     32.0,
            //     32.0,
            // );
            // let origin = FVec2::new(
            //     (bbox_x_dist) as f32, 
            //     (bbox_y_dist) as f32
            // );
            let mut project = CartesianPlane::new(&mut bmp, origin);
            
            
            let mut edgevec = vec![];
            println!("{}", builder.contours.len());
            for c in &builder.contours {
                for e in &c.edges {
                    edgevec.push(e.as_ref());
                }
            }
            // project.remapped_graph(-100..100, -100..100, |x, y|{
            //     if x.is_negative() && y.is_negative() {
            //         (true, RGBAChannel(255, 0, 0, 255).antialiased(<RGBAChannel as Antialiased>::create_binary_tvalue(1.0)))
            //     } else if x.is_negative() && y.is_positive() {
            //         (true, RGBAChannel(0, 255, 0, 255).antialiased(<RGBAChannel as Antialiased>::create_binary_tvalue(1.0)))
            //     } else if x.is_positive() && y.is_negative() {
            //         (true, RGBAChannel(0, 0, 255, 255).antialiased(<RGBAChannel as Antialiased>::create_binary_tvalue(1.0)))
            //     }
            //     else {
            //         (true, RGBAChannel(255, 255, 0, 255).antialiased(<RGBAChannel as Antialiased>::create_binary_tvalue(1.0)))
            //     }
            // });
            project.remapped_graph(
                bbox.x_min as isize..bbox.x_max as isize, -bbox.y_max as isize..bbox.y_min.abs() as isize, |x, y|{
                let (intesect, antialias) = contained_sdf::<RGBAChannel>(FVec2::new(x as f32, -y as f32), &edgevec);
                
                (intesect, rgb.antialiased(antialias))
            });
            total_offset += 3;
        }
        Self { offsets, bmp }
    }
}