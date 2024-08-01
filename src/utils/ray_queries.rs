use core::f32;

use glam::{vec3a, Vec3A};

use crate::voxel_model::{VoxelData, VoxelDataVisitor};

#[inline(always)]
pub fn cast_ray_to_box(
    ray_origin : Vec3A,
    ray_dir : Vec3A,
    p0 : Vec3A,
    size : Vec3A
) -> Option<f32> {
    let p1 = p0 + size;

    if (p0.x ..= p1.x).contains(&ray_origin.x) {
        if (p0.y ..= p1.y).contains(&ray_origin.y) {
            if (p0.z ..= p1.z).contains(&ray_origin.z) {
                return Some(0.0);
            }
        }
    }

    let t012 = (p0 - ray_origin) / ray_dir;
    let t345 = (p1 - ray_origin) / ray_dir;

    #[inline(always)]
    fn range(x: f32, y: f32) -> std::ops::RangeInclusive<f32> {
        if x <= y { x..=y } else { y..=x }
    }

    let (x_range, y_range, z_range) = (
        &range(t012.x, t345.x),
        &range(t012.y, t345.y),
        &range(t012.z, t345.z)
    );

    #[inline(always)]
    fn get_min_t(
        checked_r: &std::ops::RangeInclusive<f32>,
        r0: &std::ops::RangeInclusive<f32>,
        r1: &std::ops::RangeInclusive<f32>,
        min_t: &mut Option<f32>
    ) {
        let t = *checked_r.start();
        if t >= 0.0 && r0.contains(&t) && r1.contains(&t) {
            *min_t = Some(min_t.map(|old_t| old_t.min(t)).unwrap_or(t));
        }
        let t = *checked_r.end();
        if t >= 0.0 && r0.contains(&t) && r1.contains(&t) {
            *min_t = Some(min_t.map(|old_t| old_t.min(t)).unwrap_or(t));
        }
    }

    let min_t = &mut None;
    if t012.x.is_finite() { get_min_t(x_range, y_range, z_range, min_t); }
    if t012.y.is_finite() { get_min_t(y_range, x_range, z_range, min_t); }
    if t012.z.is_finite() { get_min_t(z_range, x_range, y_range, min_t); }
    *min_t
}

pub struct VoxelIntersector<'a> {
    pub ray_origin: Vec3A,
    pub ray_dir: Vec3A,
    pub pos: Vec3A,
    pub min: &'a mut Option<(f32, u8)>
}
impl<'a> VoxelDataVisitor for VoxelIntersector<'a> {
    fn visit(
        &mut self,
        min_c: &[usize],
        max_c: &[usize],
        data: &VoxelData
    ) -> bool {
        let p0 = vec3a(min_c[0] as f32, min_c[1] as f32, min_c[2] as f32);
        let size = vec3a(max_c[0] as f32, max_c[1] as f32, max_c[2] as f32) - p0;

        let intersection = cast_ray_to_box(
            self.ray_origin,
            self.ray_dir,
            self.pos + p0,
            size
        );

        match data {
            VoxelData::Node2x2x2 { .. } => match (*self.min, intersection) {
                (Some((old_t, _)), Some(t)) if t < old_t => true,
                (None, Some(_)) => true,
                _ => false
            },
            VoxelData::Leaf { color_id } if *color_id == 0 => false,
            &VoxelData::Leaf { color_id } => {
                match (*self.min, intersection) {
                    (Some((old_t, _)), Some(t)) if t < old_t => {
                        assert!(!old_t.is_nan());
                        assert!(!t.is_nan());
                        *self.min = Some((t, color_id))
                    },
                    (None, Some(t)) => {
                        assert!(!t.is_nan());
                        *self.min = Some((t, color_id))
                    },
                    _ => ()
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod test {
    use glam::vec3a;

    use super::cast_ray_to_box;

    #[test]
    fn test_cast_ray_to_box() {
        let p0 = vec3a(-16.0, -48.0, 96.0);
        let size_0 = vec3a(32.0, 32.0, 32.0);

        let p1 = vec3a(-16.0, -48.0, 64.0);
        let size_1 = vec3a(32.0, 32.0, 32.0);

        let ray_origin = vec3a(0.0, -24.0, 70.0);
        let ray_dir = vec3a(0.0052223853, -0.55721146, 0.8303543);

        let t0 = cast_ray_to_box(ray_origin, ray_dir, p0, size_0);
        let t1 = cast_ray_to_box(ray_origin, ray_dir, p1, size_1);

        //assert!(t0.is_some());
        //assert!(t1.is_some());

        println!("{:?}, {:?}", t0, t1);

        let ray_origin = vec3a(0.0, -24.0, 72.0);

        let t0 = cast_ray_to_box(ray_origin, ray_dir, p0, size_0);
        let t1 = cast_ray_to_box(ray_origin, ray_dir, p1, size_1);

        //assert!(t0.is_some());
        //assert!(t1.is_some());

        println!("{:?}, {:?}", t0, t1);

        let ray_origin = vec3a(0.0, -24.0, 80.0);

        let t0 = cast_ray_to_box(ray_origin, ray_dir, p0, size_0);
        let t1 = cast_ray_to_box(ray_origin, ray_dir, p1, size_1);

        //assert!(t0.is_some());
        //assert!(t1.is_some());

        println!("{:?}, {:?}", t0, t1);
    }
}
