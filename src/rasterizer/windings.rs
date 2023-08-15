use drowsed_math::{FloatingPoint, Segment, QuadraticSegment, Vector2, LinearSegment, FlatSegment, barycentric_coordinates, quadratic_bezier_curve_sdf};


pub trait WindingOrder: FlatSegment {
    fn winding(&self, p: &<Self as Segment>::VectorType) -> i32;
}
fn linear_winding<T: FloatingPoint>(start: Vector2<T>, end: Vector2<T>, p: Vector2<T>) -> i32 {
    if start.y <= p.y && end.y > p.y {
        let direction = drowsed_math::line_sdf(start, end, p);
        if direction > T::zero() { // On Right
            return -1;
        }
    } else if start.y > p.y && end.y <= p.y {
        let direction = drowsed_math::line_sdf(start, end, p);
        if direction < T::zero() { // On Left
            return 1;
        }
    }
    0
}
impl<T: FloatingPoint> WindingOrder for LinearSegment<Vector2<T>>
    where f32: num_traits::cast::AsPrimitive<T>,
    f64: num_traits::cast::AsPrimitive<T> {
    #[inline]
    fn winding(&self, p: &<Self as Segment>::VectorType) -> i32 {
        let start = *self.start();
        let end = *self.end();
        linear_winding::<T>(start, end, *p)
    }
}
impl<T: FloatingPoint> WindingOrder for QuadraticSegment<Vector2<T>>
    where f32: num_traits::cast::AsPrimitive<T>,
    f64: num_traits::cast::AsPrimitive<T> {
    fn winding(&self, p: &<Self as Segment>::VectorType) -> i32 {
        let start = *self.start();
        let end = *self.end();
        let winding = linear_winding(start, end, *p);
        let control = *self.get_point(1); // we're going to use point 2 at index 1 as control point in quadratic bezier curve
        // check if the point is in the triangle using barycentric coordinates
        let bcoords = barycentric_coordinates(start, end, control, *p);
        if bcoords.x >= T::zero() && bcoords.x <= T::one() && bcoords.y >= T::zero() && bcoords.y <= T::one() && bcoords.z >= T::zero() && bcoords.z <= T::one() {
            let uv_value = quadratic_bezier_curve_sdf(start, end, control, bcoords);
            if uv_value <= T::zero() {
                return winding + 1;
            } else {
                return winding - 1;
            }
        }
        winding
    }
}