use edict::world::World;
use glam::vec3a;
use retro_blit::{rendering::blittable::{BufferProviderMut, SizedSurface}, window::RetroBlitContext};

use crate::{components::{Color, Position, Voxel}, systems::BaseSystem};

#[derive(Default)]
pub struct VoxelRenderingSystem {
    tvec : Vec<(f32, u8)>
}

impl BaseSystem for VoxelRenderingSystem {
    fn run(&mut self, ctx: &mut RetroBlitContext, world: &mut World, _dt: f32) {
        let (sw, sh) = (ctx.get_width(), ctx.get_height());

        let mut stride = 0;
        for j in 0..sh {
            for i in 0..sw {
                let ix = i + stride;
                let ray_origin = vec3a(
                    i as f32 * 2.0f32 / (sw as f32) - 1.0f32,
                    j as f32 * 2.0f32 / (sh as f32) - 1.0f32,
                    -10.0
                );
                let ray_dir = vec3a(0.0f32, 0.0f32, 1.0f32);

                self.tvec.clear();

                for (pos, vox, col) in world.view::<(&Position, &Voxel, &Color)>() {
                    let p0 = pos.value;
                    let p1 = p0 + vox.size;

                    let t012 = (p0 - ray_origin) / ray_dir;
                    let t345 = (p0 - ray_origin) / ray_dir;

                    for side_t in [t012.x, t345.x] {
                        //p.y ∈ [p0.y, p1.y] && p.z ∈ [p0.z, p1.z]
                        let p = ray_origin + ray_dir * side_t;
                        if (p0.y..=p1.y).contains(&p.y) && (p0.z..=p1.z).contains(&p.z) {
                            self.tvec.push((side_t, col.idx));
                        }
                    }

                    for side_t in [t012.y, t345.y] {
                        //p.x ∈ [p0.x, p1.x] && p.z ∈ [p0.z, p1.z]
                        let p = ray_origin + ray_dir * side_t;
                        if (p0.x..=p1.x).contains(&p.x) && (p0.z..=p1.z).contains(&p.z) {
                            self.tvec.push((side_t, col.idx));
                        }
                    }

                    for side_t in [t012.z, t345.z] {
                        //p.x ∈ [p0.x, p1.x] && p.y ∈ [p0.y, p1.y]
                        let p = ray_origin + ray_dir * side_t;
                        if (p0.x..=p1.x).contains(&p.x) && (p0.y..=p1.y).contains(&p.y) {
                            self.tvec.push((side_t, col.idx));
                        }
                    }
                }

                let mut min = None;
                for (t, color_id) in self.tvec.drain(..) {
                    match min {
                        Some((t_min, _)) if t_min > t => {
                            min = Some((t, color_id))
                        },
                        None => {
                            min = Some((t, color_id))
                        }
                        _ => {}
                    }
                }

                let Some((_, color_id)) = min else { continue; };

                ctx.get_buffer_mut()[ix] = color_id;
            }
            stride += ctx.get_width();
        }
    }
}
