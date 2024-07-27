use edict::prelude::Component;
use glam::Vec3A;

#[derive(Clone, Copy, Component)]
pub struct Position{ pub value: Vec3A }

#[derive(Clone, Copy, Component)]
pub struct Voxel{ pub size: Vec3A }

#[derive(Clone, Copy, Component)]
pub struct Color{ pub idx: u8 }
