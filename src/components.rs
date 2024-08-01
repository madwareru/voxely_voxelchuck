use edict::prelude::Component;
use glam::Vec3A;

use crate::voxel_model::VoxelModel;

#[derive(Clone, Copy, Component)]
pub struct Position{ pub value: Vec3A }

#[derive(Clone, Component)]
pub struct Voxel{ pub data: VoxelModel }

#[derive(Clone, Copy, Component)]
pub struct ViewAngle{ pub value: f32 }

#[derive(Clone, Copy, Component)]
pub struct PlayerTag;
