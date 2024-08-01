use edict::world::World;
use retro_blit::window::RetroBlitContext;

pub mod rendering;
pub mod logic;

pub trait BaseSystem {
    fn run(&mut self, ctx: &mut RetroBlitContext, world: &mut World, dt: f32);
}

pub struct SystemGroup {
    pub systems: Vec<Box<dyn BaseSystem>>
}

impl BaseSystem for SystemGroup {
    fn run(&mut self, ctx: &mut RetroBlitContext, world: &mut World, dt: f32) {
        for system in self.systems.iter_mut() {
            system.run(ctx, world, dt);
        }
    }
}
