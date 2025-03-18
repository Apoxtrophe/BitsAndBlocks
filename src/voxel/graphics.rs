use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::prelude::*;

pub fn create_voxel_mesh(tile_row: usize) -> Mesh {
    // === Texture Atlas Setup ===
    let num_tiles_x = 6.0;
    let num_rows_f = NUM_VOXELS as f32;
    let v_top = tile_row as f32 / num_rows_f;
    let v_bottom = (tile_row as f32 + 1.0) / num_rows_f;

    // Each face uses the four default UV corners:
    // 0: [u_min, v_bottom]
    // 1: [u_min, v_top]
    // 2: [u_max, v_top]
    // 3: [u_max, v_bottom]
    // We then reorder these corners for each face.
    let uv_orders: [[usize; 4]; 6] = [
        [0, 1, 2, 3], // Top face (+y)
        [0, 1, 2, 3], // Bottom face (-y)
        [0, 3, 2, 1], // Right face (+x)
        [3, 0, 1, 2], // Left face (-x)
        [0, 1, 2, 3], // Back face (+z)
        [3, 2, 1, 0], // Forward face (-z)
    ];

    let mut uvs = Vec::with_capacity(24);
    for (i, order) in uv_orders.iter().enumerate() {
        let u_min = i as f32 / num_tiles_x;
        let u_max = (i as f32 + 1.0) / num_tiles_x;
        let default_uvs = [
            [u_min, v_bottom],
            [u_min, v_top],
            [u_max, v_top],
            [u_max, v_bottom],
        ];
        for &idx in order.iter() {
            uvs.push(default_uvs[idx]);
        }
    }

    // === Cube Geometry ===
    // Define the positions for 6 faces (24 vertices).
    let positions = vec![
        // Top face (+y)
        [-0.5,  0.5, -0.5],
        [ 0.5,  0.5, -0.5],
        [ 0.5,  0.5,  0.5],
        [-0.5,  0.5,  0.5],
        // Bottom face (-y)
        [-0.5, -0.5, -0.5],
        [ 0.5, -0.5, -0.5],
        [ 0.5, -0.5,  0.5],
        [-0.5, -0.5,  0.5],
        // Right face (+x)
        [ 0.5, -0.5, -0.5],
        [ 0.5, -0.5,  0.5],
        [ 0.5,  0.5,  0.5],
        [ 0.5,  0.5, -0.5],
        // Left face (-x)
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5,  0.5],
        [-0.5,  0.5,  0.5],
        [-0.5,  0.5, -0.5],
        // Back face (+z)
        [-0.5, -0.5,  0.5],
        [-0.5,  0.5,  0.5],
        [ 0.5,  0.5,  0.5],
        [ 0.5, -0.5,  0.5],
        // Forward face (-z)
        [-0.5, -0.5, -0.5],
        [-0.5,  0.5, -0.5],
        [ 0.5,  0.5, -0.5],
        [ 0.5, -0.5, -0.5],
    ];
    let rotated_positions: Vec<[f32; 3]> = positions
        .into_iter()
        .map(|p| {
            // In this example, we’re simply converting to Vec3 and back.
            // This is where you might apply an actual rotation.
            let pos = Vec3::from(p);
            [pos.x, pos.y, pos.z]
        })
        .collect();

    // Normals for each vertex.
    let normals = vec![
        // Top face (+y)
        [ 0.0,  1.0,  0.0],
        [ 0.0,  1.0,  0.0],
        [ 0.0,  1.0,  0.0],
        [ 0.0,  1.0,  0.0],
        // Bottom face (-y)
        [ 0.0, -1.0,  0.0],
        [ 0.0, -1.0,  0.0],
        [ 0.0, -1.0,  0.0],
        [ 0.0, -1.0,  0.0],
        // Right face (+x)
        [ 1.0,  0.0,  0.0],
        [ 1.0,  0.0,  0.0],
        [ 1.0,  0.0,  0.0],
        [ 1.0,  0.0,  0.0],
        // Left face (-x)
        [-1.0,  0.0,  0.0],
        [-1.0,  0.0,  0.0],
        [-1.0,  0.0,  0.0],
        [-1.0,  0.0,  0.0],
        // Back face (+z)
        [ 0.0,  0.0,  1.0],
        [ 0.0,  0.0,  1.0],
        [ 0.0,  0.0,  1.0],
        [ 0.0,  0.0,  1.0],
        // Forward face (-z)
        [ 0.0,  0.0, -1.0],
        [ 0.0,  0.0, -1.0],
        [ 0.0,  0.0, -1.0],
        [ 0.0,  0.0, -1.0],
    ];
    let rotated_normals: Vec<[f32; 3]> = normals
        .into_iter()
        .map(|n| {
            let norm = Vec3::from(n);
            [norm.x, norm.y, norm.z]
        })
        .collect();

    // Indices for the cube.
    let indices = Indices::U32(vec![
         0,  3,  1,  1,  3,  2,  // Top face
         4,  5,  7,  5,  6,  7,  // Bottom face
         8, 11,  9,  9, 11, 10,  // Right face
        12, 13, 15, 13, 14, 15,  // Left face
        16, 19, 17, 17, 19, 18,  // Back face
        20, 21, 23, 21, 22, 23,  // Forward face
    ]);

    // Build and return the mesh.
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, rotated_positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, rotated_normals)
    .with_inserted_indices(indices)
}


pub fn create_cable_mesh(tile_row: usize, connections: [bool; 6]) -> Mesh {
    // Texture atlas setup.
    let num_tiles_x = 6.0;
    let num_rows_f = NUM_VOXELS as f32;
    let v_top = tile_row as f32 / num_rows_f;
    let v_bottom = (tile_row as f32 + 1.0) / num_rows_f;

    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0u32;

    // Helper to compute UVs for a face.
    fn compute_face_uvs(
        tile_index: usize,
        v_top: f32,
        v_bottom: f32,
        order: [usize; 4],
        num_tiles_x: f32,
    ) -> [[f32; 2]; 4] {
        let u_min = tile_index as f32 / num_tiles_x;
        let u_max = (tile_index as f32 + 1.0) / num_tiles_x;
        let default_uvs = [
            [u_min, v_bottom],
            [u_min, v_top],
            [u_max, v_top],
            [u_max, v_bottom],
        ];
        [
            default_uvs[order[0]],
            default_uvs[order[1]],
            default_uvs[order[2]],
            default_uvs[order[3]],
        ]
    }

    // Helper to add a cuboid with per-face UVs based on tile indices.
    fn add_cuboid(
        positions: &mut Vec<[f32; 3]>,
        uvs: &mut Vec<[f32; 2]>,
        normals: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        index_offset: &mut u32,
        dims: [f32; 3],
        offset: [f32; 3],
        v_top: f32,
        v_bottom: f32,
        num_tiles_x: f32,
    ) {
        let hx = dims[0] * 0.5;
        let hy = dims[1] * 0.5;
        let hz = dims[2] * 0.5;

        // Closure to push a face.
        let mut push_face = |face_positions: &[[f32; 3]; 4],
                               tile_index: usize,
                               uv_order: [usize; 4],
                               normal: [f32; 3]| {
            let start = *index_offset;
            positions.extend_from_slice(face_positions);
            let face_uvs = compute_face_uvs(tile_index, v_top, v_bottom, uv_order, num_tiles_x);
            uvs.extend_from_slice(&face_uvs);
            for _ in 0..4 {
                normals.push(normal);
            }
            indices.extend_from_slice(&[start, start + 1, start + 2, start, start + 2, start + 3]);
            *index_offset += 4;
        };

        // For each face, we now assign a tile index (0–5) so that the U coordinates are a subrange
        // of [0,1] rather than the full range. You can adjust these indices and UV orders as needed.
        push_face(
            // Top face (+Y)
            &[
                [offset[0] - hx, offset[1] + hy, offset[2] - hz],
                [offset[0] - hx, offset[1] + hy, offset[2] + hz],
                [offset[0] + hx, offset[1] + hy, offset[2] + hz],
                [offset[0] + hx, offset[1] + hy, offset[2] - hz],
            ],
            0, // tile index for top face
            [0, 1, 2, 3],
            [0.0, 1.0, 0.0],
        );

        push_face(
            // Bottom face (-Y)
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] - hz],
                [offset[0] + hx, offset[1] - hy, offset[2] - hz],
                [offset[0] + hx, offset[1] - hy, offset[2] + hz],
                [offset[0] - hx, offset[1] - hy, offset[2] + hz],
            ],
            1, // tile index for bottom face
            [0, 3, 2, 1],
            [0.0, -1.0, 0.0],
        );

        push_face(
            // Right face (+X)
            &[
                [offset[0] + hx, offset[1] - hy, offset[2] - hz],
                [offset[0] + hx, offset[1] + hy, offset[2] - hz],
                [offset[0] + hx, offset[1] + hy, offset[2] + hz],
                [offset[0] + hx, offset[1] - hy, offset[2] + hz],
            ],
            2, // tile index for right face
            [0, 1, 2, 3],
            [1.0, 0.0, 0.0],
        );

        push_face(
            // Left face (-X)
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] + hz],
                [offset[0] - hx, offset[1] + hy, offset[2] + hz],
                [offset[0] - hx, offset[1] + hy, offset[2] - hz],
                [offset[0] - hx, offset[1] - hy, offset[2] - hz],
            ],
            3, // tile index for left face
            [3, 0, 1, 2],
            [-1.0, 0.0, 0.0],
        );

        push_face(
            // Front face (-Z)
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] - hz],
                [offset[0] - hx, offset[1] + hy, offset[2] - hz],
                [offset[0] + hx, offset[1] + hy, offset[2] - hz],
                [offset[0] + hx, offset[1] - hy, offset[2] - hz],
            ],
            4, // tile index for front face
            [0, 1, 2, 3],
            [0.0, 0.0, -1.0],
        );

        push_face(
            // Back face (+Z)
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] + hz],
                [offset[0] + hx, offset[1] - hy, offset[2] + hz],
                [offset[0] + hx, offset[1] + hy, offset[2] + hz],
                [offset[0] - hx, offset[1] + hy, offset[2] + hz],
            ],
            5, // tile index for back face
            [0, 3, 2, 1],
            [0.0, 0.0, 1.0],
        );
    }
    let mut bool_count = 0;
    
    let mut core_dims = [0.3, 0.3, 0.3];
    for i in 0..connections.len() {
        if connections[i] == true {
            bool_count += 1;
        }
    }
    
    if bool_count <= 1 {
        core_dims = [0.5, 0.5, 0.5];
    }
    

    // 1. Add the cable core.

    let core_offset = [0.0, 0.0, 0.0];
    add_cuboid(
        &mut positions,
        &mut uvs,
        &mut normals,
        &mut indices,
        &mut index_offset,
        core_dims,
        core_offset,
        v_top,
        v_bottom,
        num_tiles_x,
    );

    // 2. Add extensions based on connectivity.
    if connections[0] {
        let ext_width = 0.5 - core_dims[0] / 2.0;
        let ext_dims = [ext_width, 0.3, 0.3];
        let ext_offset = [-(core_dims[0] / 2.0 + ext_width / 2.0), 0.0, 0.0];
        add_cuboid(
            &mut positions,
            &mut uvs,
            &mut normals,
            &mut indices,
            &mut index_offset,
            ext_dims,
            ext_offset,
            v_top,
            v_bottom,
            num_tiles_x,
        );
    }
    if connections[1] {
        let ext_width = 0.5 - core_dims[0] / 2.0;
        let ext_dims = [ext_width, 0.3, 0.3];
        let ext_offset = [core_dims[0] / 2.0 + ext_width / 2.0, 0.0, 0.0];
        add_cuboid(
            &mut positions,
            &mut uvs,
            &mut normals,
            &mut indices,
            &mut index_offset,
            ext_dims,
            ext_offset,
            v_top,
            v_bottom,
            num_tiles_x,
        );
    }
    if connections[2] {
        let ext_height = 0.5 - core_dims[1] / 2.0;
        let ext_dims = [0.3, ext_height, 0.3];
        let ext_offset = [0.0, core_dims[1] / 2.0 + ext_height / 2.0, 0.0];
        add_cuboid(
            &mut positions,
            &mut uvs,
            &mut normals,
            &mut indices,
            &mut index_offset,
            ext_dims,
            ext_offset,
            v_top,
            v_bottom,
            num_tiles_x,
        );
    }
    if connections[3] {
        let ext_height = 0.5 - core_dims[1] / 2.0;
        let ext_dims = [0.3, ext_height, 0.3];
        let ext_offset = [0.0, -(core_dims[1] / 2.0 + ext_height / 2.0), 0.0];
        add_cuboid(
            &mut positions,
            &mut uvs,
            &mut normals,
            &mut indices,
            &mut index_offset,
            ext_dims,
            ext_offset,
            v_top,
            v_bottom,
            num_tiles_x,
        );
    }
    if connections[4] {
        let ext_depth = 0.5 - core_dims[2] / 2.0;
        let ext_dims = [0.3, 0.3, ext_depth];
        let ext_offset = [0.0, 0.0, -(core_dims[2] / 2.0 + ext_depth / 2.0)];
        add_cuboid(
            &mut positions,
            &mut uvs,
            &mut normals,
            &mut indices,
            &mut index_offset,
            ext_dims,
            ext_offset,
            v_top,
            v_bottom,
            num_tiles_x,
        );
    }
    if connections[5] {
        let ext_depth = 0.5 - core_dims[2] / 2.0;
        let ext_dims = [0.3, 0.3, ext_depth];
        let ext_offset = [0.0, 0.0, core_dims[2] / 2.0 + ext_depth / 2.0];
        add_cuboid(
            &mut positions,
            &mut uvs,
            &mut normals,
            &mut indices,
            &mut index_offset,
            ext_dims,
            ext_offset,
            v_top,
            v_bottom,
            num_tiles_x,
        );
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}