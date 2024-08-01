use std::array::from_fn;

use glam::vec3a;

#[derive(Clone, Debug)]
pub enum VoxelData {
    Leaf { color_id: u8 },
    Node2x2x2 { children: Box<[[[VoxelData; 2]; 2]; 2]> }
}

impl Default for VoxelData {
    fn default() -> Self {
        Self::make_leaf(0)
    }
}

impl VoxelData {
    pub fn compact(&self) -> VoxelData {
        let VoxelData::Node2x2x2 { children } = self else { return self.clone(); };

        // first compact all children
        let new_result = VoxelData::make_2x2x2(|i, j, k| { children[k as usize][j as usize][i as usize].compact() });
        let VoxelData::Node2x2x2 { children } = &new_result else { return self.clone(); };

        // then check if all children is leafs of equal color
        let mut color = None;
        for cc in children.iter() {
            for c in cc.iter() {
                for data in c.iter() {
                    let VoxelData::Leaf { color_id } = data else { return new_result; };
                    match color {
                        None => { color = Some(*color_id); },
                        Some(clr_id) if clr_id.eq(color_id) => {},
                        _ => { return new_result; }
                    }
                }
            }
        }

        match color {
            Some(color_id) => VoxelData::make_leaf(color_id),
            None => new_result,
        }
    }

    pub fn make_leaf(color_id: u8) -> Self { Self::Leaf { color_id } }
    pub fn make_2x2x2(foo: impl Fn(usize, usize, usize) -> VoxelData) -> Self {
        let arr = from_fn(|k| from_fn(|j| from_fn(|i| foo(i, j, k))));
        Self::Node2x2x2 { children: Box::new(arr) }
    }
    pub fn make_4x4x4(foo: impl Fn(usize, usize, usize) -> VoxelData) -> Self {
        Self::make_2x2x2(|ii, jj, kk| Self::make_2x2x2(|i, j, k| foo(ii * 2 + i, jj * 2 + j, kk * 2 + k)))
    }
    pub fn make_8x8x8(foo: impl Fn(usize, usize, usize) -> VoxelData) -> Self {
        Self::make_2x2x2(|ii, jj, kk| Self::make_4x4x4(|i, j, k| foo(ii * 4 + i, jj * 4 + j, kk * 4 + k)))
    }
    pub fn make_16x16x16(foo: impl Fn(usize, usize, usize) -> VoxelData) -> Self {
        Self::make_2x2x2(|ii, jj, kk| Self::make_8x8x8(|i, j, k| foo(ii * 8 + i, jj * 8 + j, kk * 8 + k)))
    }
    pub fn make_32x32x32(foo: impl Fn(usize, usize, usize) -> VoxelData) -> Self {
        Self::make_2x2x2(|ii, jj, kk| Self::make_16x16x16(|i, j, k| foo(ii * 16 + i, jj * 16 + j, kk * 16 + k)))
    }
    pub fn traverse<T: VoxelDataVisitor>(
        &self,
        min: [usize; 3],
        max: [usize; 3],
        visitor: &mut T
    ) {
        if !visitor.visit(&min, &max, self) { return; }
        let VoxelData::Node2x2x2 { children } = self else { return; };

        let [min_pi, min_pj, min_pk] = min;
        let [max_pi, max_pj, max_pk] = max;

        let step_pi = (max_pi - min_pi) / 2;
        let step_pj = (max_pj - min_pj) / 2;
        let step_pk = (max_pk - min_pk) / 2;

        let pi_range = (min_pi..max_pi).step_by(step_pi);
        let pj_range = (min_pj..max_pj).step_by(step_pj);
        let pk_range = (min_pk..max_pk).step_by(step_pk);

        for (cc, pk) in children.as_ref().iter().zip(pk_range) {
            for (c, pj) in cc.iter().zip(pj_range.clone()) {
                for (data, pi) in c.iter().zip(pi_range.clone()) {
                    data.traverse([pi, pj, pk], [pi + step_pi, pj + step_pj, pk + step_pk], visitor);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct VoxelModel {
    pub size: [usize; 3],
    pub data: VoxelData
}

pub trait VoxelDataVisitor {
    fn visit(
        &mut self,
        min_p: &[usize],
        max_p: &[usize],
        data: &VoxelData
    ) -> bool;
}

impl VoxelModel {
    pub fn traverse<T: VoxelDataVisitor>(&self, visitor: &mut T) {
        self.data.traverse([0; 3], self.size, visitor)
    }
    pub fn make_sphere32x32x32(transparent_color: u8, opaque_color: u8) -> Self {
        let center_p = vec3a(15.5, 15.5, 15.5);
        let mag_sqr = 15.5 * 15.5;
        let data = VoxelData::make_32x32x32(|x, y, z| {
           let p = vec3a(x as _, y as _, z as _);
           let diff = p - center_p;
           let dot = diff.dot(diff);
           let color = if dot <= mag_sqr { opaque_color } else { transparent_color };
           VoxelData::make_leaf(color)
        }).compact();
        Self { size: [32; 3], data }
    }
}
