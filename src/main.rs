use components::{Color, Position, Voxel};
use edict::world::World;
use glam::vec3a;
use retro_blit::window::{RetroBlitContext, ContextHandler, WindowMode};
use systems::rendering::voxels::VoxelRenderingSystem;
use systems::rendering::ClearScreenSystem;
use systems::{BaseSystem, SystemGroup};

pub mod systems;
pub mod components;

struct App {
    world: World,
    root_system_group: SystemGroup
}

impl App {
    fn create_logic_systems() -> Box<dyn BaseSystem> {
        Box::new(SystemGroup {
            systems: vec![
                // todo
            ]
        })
    }

    fn create_rendering_systems() -> Box<dyn BaseSystem> {
        Box::new(SystemGroup {
            systems: vec![
                Box::new(ClearScreenSystem(255)),
                Box::new(VoxelRenderingSystem::default())
            ]
        })
    }

    pub fn new() -> Self {

        let root_system_group = SystemGroup {
            systems: vec![
                Self::create_logic_systems(),
                Self::create_rendering_systems()
            ]
        };

        let mut world = World::new();

        world.spawn(
            (
                Position { value: vec3a(-0.1, -0.1, -0.1) },
                Voxel { size: vec3a(0.2, 0.2, 0.2) },
                Color { idx: 25 }
            )
        );

        Self { world, root_system_group }
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
        let mut idx = 0;
        for j in 0..16 {
            for i in 0..16 {
                let red = 255.0 * (i as f32) / 15.0;
                let green = 255.0 * (j as f32) / 15.0;
                ctx.set_palette(idx, [red as _, green as _, 0]);
                if idx < 255 {
                    idx += 1;
                }
            }
        }
    }

    fn update(&mut self, ctx: &mut RetroBlitContext, dt: f32) {
        let world = &mut self.world;
        self.root_system_group.run(ctx, world, dt);
    }
}

fn main() {
    retro_blit::window::start(App::new())
}
