use components::{PlayerTag, Position, ViewAngle, Voxel};
use edict::world::World;
use glam::vec3a;
use retro_blit::window::{RetroBlitContext, ContextHandler, WindowMode};
use systems::logic::player_systems::RotateOnPlaceSystem;
use systems::rendering::voxels::VoxelRenderingSystem;
use systems::rendering::ClearScreenSystem;
use systems::{BaseSystem, SystemGroup};
use utils::loaders::{create_voxel_model_from_2d_tile, load_xraw};
use voxel_model::VoxelModel;

pub mod systems;
pub mod components;
pub mod utils;
pub mod voxel_model;

const TILES_2D_BYTES: &[u8] = include_bytes!("assets/tiles2d.im256");
//const GRASS_XRAW: &[u8] = include_bytes!("assets/grass.vox.xraw");
const GRASS_DIRT_CORNER_XRAW: &[u8] = include_bytes!("assets/grass_dirt_corner.vox.xraw");

struct App {
    world: World,
    root_system_group: SystemGroup,
    palette: Vec<[u8; 3]>,
    tiles_2d: retro_blit::rendering::BlittableSurface
}

impl App {
    fn create_logic_systems() -> Box<dyn BaseSystem> {
        Box::new(SystemGroup {
            systems: vec![
                //Box::new(MoveForwardSystem),
                Box::new(RotateOnPlaceSystem)
            ]
        })
    }

    fn create_rendering_systems() -> Box<dyn BaseSystem> {
        Box::new(SystemGroup {
            systems: vec![
                Box::new(ClearScreenSystem(1)),
                Box::new(VoxelRenderingSystem::new())
            ]
        })
    }

    pub fn new() -> Self {
        let (palette, tiles_2d) = retro_blit::format_loaders::im_256::Image
            ::load_from(TILES_2D_BYTES)
                .unwrap();

        let root_system_group = SystemGroup {
            systems: vec![
                Self::create_logic_systems(),
                Self::create_rendering_systems()
            ]
        };

        let world = World::new();

        Self { world, root_system_group, palette, tiles_2d }
    }
}

impl ContextHandler for App {
    fn get_window_title(&self) -> &'static str {
        "Вокселий Воксельчук в пещерах тёмных и глубоких"
    }

    fn get_window_mode(&self) -> WindowMode {
        WindowMode::Mode160x120
    }

    fn init(&mut self, ctx: &mut RetroBlitContext) {
        let grass_tile = load_xraw(GRASS_DIRT_CORNER_XRAW);
        let lava_tile = create_voxel_model_from_2d_tile(&self.tiles_2d, 64, 32);
        let water_tile = create_voxel_model_from_2d_tile(&self.tiles_2d, 64, 64);
        let sphere = VoxelModel::make_sphere32x32x32(0, 5);

        for (i, [red, green, blue]) in self.palette.iter().enumerate() {
            ctx.set_palette(i as u8, [*red, *green, *blue])
        }

        let world = &mut self.world;

        world.spawn(
            (
                PlayerTag,
                Position { value: vec3a(0.0, -16.0, 80.0) },
                ViewAngle { value: (0.0f32).to_radians() }
            )
        );

        world.spawn(
            (
                Position { value: vec3a(-16.0, -48.0, 96.0) },
                Voxel { data: lava_tile.clone() }
            )
        );
        world.spawn(
            (
                Position { value: vec3a(-16.0, -48.0, 64.0) },
                Voxel {
                    data: water_tile.clone()
                }
            )
        );
        world.spawn(
            (
                Position { value: vec3a(-16.0, -48.0, 32.0) },
                Voxel {
                    data: lava_tile.clone()
                }
            )
        );
        world.spawn(
            (
                Position { value: vec3a(16.0, -48.0, 64.0) },
                Voxel {
                    data: grass_tile.clone()
                }
            )
        );
        world.spawn(
            (
                Position { value: vec3a(-32.0, 0.0, 164.0) },
                Voxel { data: sphere }
            )
        );
    }

    fn update(&mut self, ctx: &mut RetroBlitContext, dt: f32) {
        //let _sw = StopWatch::named("update");
        let world = &mut self.world;
        self.root_system_group.run(ctx, world, dt);
    }
}

fn main() {
    retro_blit::window::start(App::new())
}
