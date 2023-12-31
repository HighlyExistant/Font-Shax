use std::{collections::HashMap, ops::Range};

use drowsed_math::{USizeVec2, FVec2};

use crate::{bitmap::{pixels::{RGBAChannel, Antialiased}, Bitmap, projection::{CartesianPlane, BitmapProjection}}, rasterizer::vectors::contained_sdf};

use super::FontShape;
#[derive(Clone, Debug)]
pub struct FontAtlasEntry {
    pub bbox: (Range<isize>, Range<isize>)
}
pub struct FontAtlas {
    pub offsets: HashMap<char, FontAtlasEntry>,
    pub bmp: Bitmap<RGBAChannel>,
}

impl FontAtlas {
    pub fn new(letters: Vec<char>, data: &Vec<u8>, scale: f32) -> Self {
        let mut offsets: HashMap<char, FontAtlasEntry> = HashMap::new();
        let rgb = RGBAChannel(255, 255, 255, 255);
        let face = ttf_parser::Face::parse(data, 0).unwrap();
        let mut total_offset = 0isize;
        let ascender =  (face.ascender() as f32 * scale) as i16;
        let descender = (face.descender() as f32 * scale) as i16; 
        let linegap =   (face.line_gap() as f32 * scale) as i16;
        let bmpsize = letters.iter().fold(USizeVec2::new(0,0), |a, b| {
            let id = face.glyph_index(*b).unwrap();
            let mut bbox = face.glyph_bounding_box(id).unwrap();
            
            bbox.x_max = (bbox.x_max as f32 * scale) as i16;
            bbox.x_min = (bbox.x_min as f32 * scale) as i16;
            bbox.y_max = (bbox.y_max as f32 * scale) as i16;
            bbox.y_min = (bbox.y_min as f32 * scale) as i16;
            let ysize = (ascender - descender) as usize;
            
            let advance = (face.glyph_hor_advance(id).unwrap() as f32 * scale) as i16;
            let y_dist = (bbox.y_max - bbox.y_min).abs();
            let size = USizeVec2::new(advance.abs() as usize,
            y_dist as usize);
            if size.y > a.y {
                USizeVec2::new(a.x + size.x,ysize)
            } else {
                USizeVec2::new(a.x + size.x,ysize)
            }
        });
        
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

            

            let bbox_x_dist = (bbox.x_max - bbox.x_min).abs() as usize;
            let y_offset = bmpsize.y - descender.abs() as usize;
            let origin = FVec2::new(
                (total_offset) as f32, 
                (y_offset) as f32
            );
            let mut project = CartesianPlane::new(&mut bmp, origin);
            
            
            let mut edgevec = vec![];
            for c in &builder.contours {
                for e in &c.edges {
                    edgevec.push(e.as_ref());
                }
            }
            let range_x = bbox.x_min as isize..bbox.x_max as isize;
            let range_y = -bbox.y_max as isize..bbox.y_min.abs() as isize;
            let range_x_offseted = (bbox.x_min + total_offset as i16) as isize..(bbox.x_max + total_offset as i16) as isize;
            let entry = FontAtlasEntry {
                bbox: (range_x_offseted.clone(), range_y.clone())
            };
            offsets.insert(letter, entry);
            project.remapped_graph(
                range_x, range_y, |x, y|{
                let (intesect, antialias) = contained_sdf::<RGBAChannel>(FVec2::new(x as f32, -y as f32), &edgevec);
                
                (intesect, rgb.antialiased(antialias))
            });
            total_offset += bbox_x_dist as isize;
            total_offset += 3;
        }
        Self { offsets, bmp }
    }
}