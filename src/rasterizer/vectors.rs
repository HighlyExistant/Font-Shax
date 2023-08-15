use drowsed_math::{FVec2, equations::QuadraticFormula, Vector};

use crate::bitmap::pixels::Antialiased;

use super::windings::WindingOrder;


pub fn contained_sdf<T: Antialiased>(p: FVec2, edges: &Vec<&dyn WindingOrder<VectorType = FVec2>>) -> (bool, T::TValue) {
    let mut winding = 0;
    let mut signed_distance = 1.0;
    let mut min_distance = f32::MAX;
    let mut closest_edge: Option<&dyn WindingOrder<VectorType = FVec2>> = None;
    for (i, e) in edges.iter().enumerate() {
        let distance = e.signed_distance(p).abs();
        if min_distance > distance {
            min_distance = distance;
            signed_distance = e.signed_distance(p);
            closest_edge = Some(*e);
        }
        winding += e.winding(&p);
    };
    let inside = winding > 0;
    if inside && signed_distance.is_sign_negative() {
        signed_distance *= -1.0;
    }
    
    let mut antialias = T::create_binary_tvalue(signed_distance);
    if let Some(e) = closest_edge {
        let normalized = e.direction(0.5).normalize();
        if normalized.x < -0.3 {
            T::operate_mul(&mut antialias, 0.78, 1..3);
            // antialias.1 *= 0.6;
            // antialias.2 *= 0.6;
        }
        if normalized.x > 0.3 {
            T::operate_mul(&mut antialias, 0.78, 0..2);
            // antialias.0 *= 0.6;
            // antialias.1 *= 0.6;
        }
        if normalized.y > 0.85 || normalized.y < -0.85 {
            T::operate_mul(&mut antialias, 0.68, 0..3);
            // antialias.0 = signed_distance - 0.5;
            // antialias.1 = signed_distance - 0.5;
            // antialias.2 = signed_distance - 0.5;
        }
        T::alpha(&mut antialias, 1.0);
    }
    (inside, antialias)
}