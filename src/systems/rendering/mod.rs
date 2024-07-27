use edict::world::World;
use retro_blit::window::RetroBlitContext;
use super::BaseSystem;

pub mod voxels;

pub struct ClearScreenSystem(pub u8);

impl BaseSystem for ClearScreenSystem {
    fn run(&mut self, ctx: &mut RetroBlitContext, _world: &mut World, _dt: f32) {
        ctx.clear(self.0);
    }
}
