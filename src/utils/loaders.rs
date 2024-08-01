use std::io::{Cursor, Read, Seek, SeekFrom};

use retro_blit::rendering::blittable::{BufferProvider, SizedSurface};

use crate::voxel_model::{VoxelData, VoxelModel};

pub fn create_voxel_model_from_2d_tile(
    tiles_2d: &retro_blit::rendering::BlittableSurface,
    x: usize,
    y: usize
) -> VoxelModel {
    let data = VoxelData::make_2x2x2(|iiiii, jjjjj, kkkkk|{
        if jjjjj > 0 { return VoxelData::make_leaf(0); }
        VoxelData::make_2x2x2(|iiii, jjjj, kkkk|{
            if jjjj > 0 { return VoxelData::make_leaf(0); }
            VoxelData::make_2x2x2(|iii, _, kkk|{
                VoxelData::make_2x2x2(|ii, _, kk|{
                    VoxelData::make_2x2x2(|i, _, k|{
                        let i = (iiiii * 16 + iiii * 8 + iii * 4 + ii * 2 + i) as usize;
                        let k = (kkkkk * 16 + kkkk * 8 + kkk * 4 + kk * 2 + k) as usize;
                        let ix = tiles_2d.get_width() * (y + 31 - k) + i + x;
                        let color_id = tiles_2d.get_buffer()[ix];
                        VoxelData::make_leaf(color_id)
                    })
                })
            })
        })
    }).compact();
    VoxelModel { size: [32; 3], data }
}

pub fn print_xraw(bytes: &[u8]) {
    let mut cursor = Cursor::new(bytes);
    let mut buf = [0u8; 4];
    cursor.read(&mut buf).unwrap();
    assert_eq!([b'X', b'R', b'A', b'W'], buf);

    // skip some bytes of uknown nature
    cursor.seek(SeekFrom::Current(4)).unwrap();

    cursor.read(&mut buf).unwrap();
    let width =
        (buf[3] as u32) * 0x1000000 +
        (buf[2] as u32) * 0x10000 +
        (buf[1] as u32) * 0x100 +
        buf[0] as u32;

    cursor.read(&mut buf).unwrap();
    let height =
        (buf[3] as u32) * 0x1000000 +
        (buf[2] as u32) * 0x10000 +
        (buf[1] as u32) * 0x100 +
        buf[0] as u32;

    cursor.read(&mut buf).unwrap();
    let depth =
        (buf[3] as u32) * 0x1000000 +
        (buf[2] as u32) * 0x10000 +
        (buf[1] as u32) * 0x100 +
        buf[0] as u32;

    cursor.read(&mut buf).unwrap();
    let palette_size =
        (buf[3] as u32) * 0x1000000 +
        (buf[2] as u32) * 0x10000 +
        (buf[1] as u32) * 0x100 +
        buf[0] as u32;

    let mut data = Vec::with_capacity(depth as usize);
    for _ in 0..depth {
        let mut data_1 = Vec::with_capacity(height as usize);
        for _ in 0..height {
            let mut data_2 = vec![0u8; width as usize];
            cursor.read(&mut data_2).unwrap();
            data_1.push(data_2);
        }
        data.push(data_1);
    }

    println!("widht: {}, height: {}, depth: {}, pal_size: {}", width, depth, height, palette_size);

    print_hex_rs::print_hex(bytes)
}

pub fn load_xraw(bytes: &[u8]) -> VoxelModel {
    let mut cursor = Cursor::new(bytes);
    let mut buf = [0u8; 4];
    cursor.read(&mut buf).unwrap();
    assert_eq!([b'X', b'R', b'A', b'W'], buf);

    // skip some bytes of uknown nature
    cursor.seek(SeekFrom::Current(4)).unwrap();

    cursor.read(&mut buf).unwrap();
    let width =
        (buf[3] as u32) * 0x1000000 +
        (buf[2] as u32) * 0x10000 +
        (buf[1] as u32) * 0x100 +
        buf[0] as u32;

    cursor.read(&mut buf).unwrap();
    let height =
        (buf[3] as u32) * 0x1000000 +
        (buf[2] as u32) * 0x10000 +
        (buf[1] as u32) * 0x100 +
        buf[0] as u32;

    cursor.read(&mut buf).unwrap();
    let depth =
        (buf[3] as u32) * 0x1000000 +
        (buf[2] as u32) * 0x10000 +
        (buf[1] as u32) * 0x100 +
        buf[0] as u32;

    // skip palette data since we know it from other source
    cursor.seek(SeekFrom::Current(4)).unwrap();

    let mut data = Vec::with_capacity(height as usize);
    for _ in 0..height {
        let mut data_1 = Vec::with_capacity(width as usize);
        for _ in 0..width {
            let mut data_2 = vec![0u8; depth as usize];
            cursor.read(&mut data_2).unwrap();
            data_1.push(data_2);
        }
        data.push(data_1);
    }

    let data = VoxelData::make_32x32x32(|i, j, k| {
        let clr = data[j][i][k];
        VoxelData::make_leaf(if clr > 0 { clr - 1 } else { clr })
    }).compact();
    VoxelModel { size: [width as usize, height as usize, depth as usize], data }
}
