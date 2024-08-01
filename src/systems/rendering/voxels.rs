use edict::world::World;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use retro_blit::{
    rendering::{blittable::{BufferProviderMut, SizedSurface}, fonts::{font_align::{HorizontalAlignment, VerticalAlignment}, tri_spaced::{Font, TextDrawer}}},
    utility::StopWatch,
    window::RetroBlitContext
};
use crate::{
    components::{PlayerTag, Position, ViewAngle, Voxel},
    systems::BaseSystem,
    utils::{ray_queries::VoxelIntersector, rendering::gen_frustum_planes}
};

pub struct VoxelRenderingSystem {
    font: Font
}

impl VoxelRenderingSystem {
    pub fn new() -> Self {
        Self {
            font: Font::default_font_small().unwrap()
        }
    }
}

impl BaseSystem for VoxelRenderingSystem {
    fn run(&mut self, ctx: &mut RetroBlitContext, world: &mut World, _dt: f32) {
        let _sw = StopWatch::named("voxels");
        let (sw, sh) = (ctx.get_width(), ctx.get_height());

        let aspect_ratio = sw as f32 / sh as f32;

        let Some((_, pos, angle)) = world.view::<(&PlayerTag, &Position, &ViewAngle)>()
            .into_iter()
            .next() else { return; };

        let [near_plane, far_plane] = gen_frustum_planes(
            pos.value.x, pos.value.y, pos.value.z,
            angle.value,
            1.125,
            aspect_ratio
        );

        let buffer = ctx.get_buffer_mut();
        let buffer_slice = &mut buffer[0..96*160];

        fn split_range(range: std::ops::Range<i32>, at: i32) -> (std::ops::Range<i32>, std::ops::Range<i32>) {
            (range.start..range.start+at, range.start+at..range.end)
        }

        let ((s0, s1), (r0, r1)) = (buffer_slice.split_at_mut(48*160), split_range(0..96, 48));

        let ((s00, s01), (r00, r01)) = (s0.split_at_mut(24*160), split_range(r0, 24));
        let ((s02, s03), (r02, r03)) = (s1.split_at_mut(24*160), split_range(r1, 24));

        let ((s000, s001), (r000, r001)) = (s00.split_at_mut(12*160), split_range(r00, 12));
        let ((s002, s003), (r002, r003)) = (s01.split_at_mut(12*160), split_range(r01, 12));
        let ((s004, s005), (r004, r005)) = (s02.split_at_mut(12*160), split_range(r02, 12));
        let ((s006, s007), (r006, r007)) = (s03.split_at_mut(12*160), split_range(r03, 12));

        [(r000, s000), (r001, s001), (r002, s002), (r003, s003),
            (r004, s004), (r005, s005), (r006, s006), (r007, s007)]
            .into_par_iter()
            .for_each(|(j_range, slice)| {
                let mut stride = 0;
                for j in j_range {
                    let v = j as f32 / 95.0;

                    let near_left = near_plane.top_left.lerp(near_plane.bottom_left, v);
                    let near_right = near_plane.top_right.lerp(near_plane.bottom_right, v);

                    let far_left = far_plane.top_left.lerp(far_plane.bottom_left, v);
                    let far_right = far_plane.top_right.lerp(far_plane.bottom_right, v);

                    let slice_mut = &mut slice[stride..stride+sw];

                    for (i, clr) in slice_mut.iter_mut().enumerate() {
                        let u = i as f32 / (sw - 1) as f32;
                        let ray_origin = near_left.lerp(near_right, u);
                        let far = far_left.lerp(far_right, u);
                        let ray_dir = (far - ray_origin).normalize();

                        let mut min = None;
                        for (pos, vox) in world.view::<(&Position, &Voxel)>() {
                            let mut intersector = VoxelIntersector {
                                ray_origin,
                                ray_dir,
                                pos: pos.value,
                                min: &mut min
                            };
                            vox.data.traverse(&mut intersector);
                        }

                        let Some((_t, color_id)) = min else { continue; };
                        *clr = color_id;
                    }
                    stride += sw;
                }
            });

        // self.font.draw_text_in_box(
        //     ctx,
        //     0, 0,
        //     160, 20,
        //     HorizontalAlignment::Center,
        //     VerticalAlignment::Center,
        //     &format!("{}", pos.value.z),
        //     Some(21)
        // );
    }
}
