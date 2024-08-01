use glam::{vec3a, Vec3A};

pub const PIXELS_PER_METER: f32 = 512.0;
pub const VIEW_RANGE: f32 = 14.0;

pub const NEAR: f32 = 0.005 * PIXELS_PER_METER;
pub const FAR: f32 = PIXELS_PER_METER * VIEW_RANGE;

#[derive(Clone, Copy, Debug)]
pub struct FrustumPlane {
    pub top_left: Vec3A,
    pub top_right: Vec3A,
    pub bottom_left: Vec3A,
    pub bottom_right: Vec3A,
}

#[inline(always)]
fn rotate(p: (f32, f32), angle: f32) -> (f32, f32) {
    let sin_cos = (angle.sin(), angle.cos());
    (
        p.0 * sin_cos.1 + p.1 * sin_cos.0,
        -p.0 * sin_cos.0 + p.1 * sin_cos.1
    )
}

#[inline(always)]
pub fn gen_trapezoid_coords(x: f32, y: f32, angle: f32, fov_slope: f32) -> [(f32, f32); 4] {
    [
        rotate((-fov_slope * NEAR, NEAR), angle),
        rotate((fov_slope * NEAR, NEAR), angle),
        rotate((-fov_slope * FAR, FAR), angle),
        rotate((fov_slope * FAR, FAR), angle)
    ].map(|p| (p.0 + x as f32, y as f32 + p.1))
}

pub fn gen_frustum_planes(x: f32, y: f32, z: f32, angle: f32, fov_slope: f32, aspect_ratio: f32) -> [FrustumPlane; 2] {
    let [near_left, near_right, far_left, far_right] = gen_trapezoid_coords(x, z, angle, fov_slope);
    [
        FrustumPlane {
            top_left: vec3a(near_left.0, y + fov_slope * NEAR / aspect_ratio, near_left.1),
            top_right: vec3a(near_right.0, y + fov_slope * NEAR / aspect_ratio, near_right.1),
            bottom_left: vec3a(near_left.0, y - fov_slope * NEAR / aspect_ratio, near_left.1),
            bottom_right: vec3a(near_right.0, y - fov_slope * NEAR / aspect_ratio, near_right.1)
        },
        FrustumPlane {
            top_left: vec3a(far_left.0, y + fov_slope * FAR / aspect_ratio, far_left.1),
            top_right: vec3a(far_right.0, y + fov_slope * FAR / aspect_ratio, far_right.1),
            bottom_left: vec3a(far_left.0, y - fov_slope * FAR / aspect_ratio, far_left.1),
            bottom_right: vec3a(far_right.0, y - fov_slope * FAR / aspect_ratio, far_right.1)
        }
    ]
}
