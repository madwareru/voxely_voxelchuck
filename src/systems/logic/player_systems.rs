use crate::{
    components::{PlayerTag, Position, ViewAngle},
    systems::BaseSystem
};

pub struct RotateOnPlaceSystem;

impl BaseSystem for RotateOnPlaceSystem {
    fn run(&mut self, _ctx: &mut retro_blit::window::RetroBlitContext, world: &mut edict::prelude::World, dt: f32) {
        for (_, view_angle) in world.view::<(&PlayerTag, &mut ViewAngle)>() {
            view_angle.value += dt * (45.0f32).to_radians();
        }
    }
}

pub struct MoveForwardSystem;

impl BaseSystem for MoveForwardSystem {
    fn run(&mut self, _ctx: &mut retro_blit::window::RetroBlitContext, world: &mut edict::prelude::World, dt: f32) {
        for (_, pos) in world.view::<(&PlayerTag, &mut Position)>() {
            pos.value.z += dt * (10.0f32);
        }
    }
}
