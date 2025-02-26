use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::config::NUM_VOXELS;

pub fn create_voxel_mesh(tile_row: usize) -> Mesh {
    // === Texture Atlas Setup ===
    // The atlas is arranged in 6 columns and `num_rows` rows.
    let num_tiles_x = 6.0;
    let num_rows_f = NUM_VOXELS as f32;
    let v_top = tile_row as f32 / num_rows_f;
    let v_bottom = (tile_row as f32 + 1.0) / num_rows_f;

    // Build the UVs per face.
    // For each of the 6 faces we compute the horizontal UV bounds for its tile:
    // u_min = tile_index/6, u_max = (tile_index+1)/6.
    // Then we remap the order for some faces so that the texture appears upright.
    let mut uvs = Vec::with_capacity(24);
    for i in 0..6 {
        let u_min = i as f32 / num_tiles_x;
        let u_max = (i as f32 + 1.0) / num_tiles_x;
        match i {
            // Top face (tile 0) and bottom face (tile 1) use the default ordering:
            // bottom-left, top-left, top-right, bottom-right.
            0 | 1 => {
                uvs.push([u_min, v_bottom]);
                uvs.push([u_min, v_top]);
                uvs.push([u_max, v_top]);
                uvs.push([u_max, v_bottom]);
            }
            // Right face (+x, tile 2): our vertex order is:
            // bottom-back, bottom-front, top-front, top-back.
            // Map these so that u_min aligns with the back edge.
            2 => {
                uvs.push([u_min, v_bottom]); // bottom-back
                uvs.push([u_max, v_bottom]); // bottom-front
                uvs.push([u_max, v_top]);    // top-front
                uvs.push([u_min, v_top]);    // top-back
            }
            // Left face (-x, tile 3): flip horizontally relative to the right face.
            3 => {
                uvs.push([u_max, v_bottom]); // bottom-back becomes right edge
                uvs.push([u_min, v_bottom]); // bottom-front becomes left edge
                uvs.push([u_min, v_top]);    // top-front becomes left edge
                uvs.push([u_max, v_top]);    // top-back becomes right edge
            }
            // Back face (+z, tile 4): use default ordering.
            4 => {
                uvs.push([u_min, v_bottom]);
                uvs.push([u_min, v_top]);
                uvs.push([u_max, v_top]);
                uvs.push([u_max, v_bottom]);
            }
            // Forward face (-z, tile 5): flip horizontally so the texture is oriented correctly.
            5 => {
                uvs.push([u_max, v_bottom]); // bottom-left becomes right edge
                uvs.push([u_max, v_top]);    // top-left becomes right edge
                uvs.push([u_min, v_top]);    // top-right becomes left edge
                uvs.push([u_min, v_bottom]); // bottom-right becomes left edge
            }
            _ => unreachable!(),
        }
    }
    
    

    // === Define Cube Geometry ===
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

    // Apply the computed rotation to each vertex position.
    let rotated_positions: Vec<[f32; 3]> = positions
        .into_iter()
        .map(|p| {
            let pos = Vec3::from(p);
            [pos.x, pos.y, pos.z]
        })
        .collect();

    // Normals for each vertex (aligned with faces).
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

    // Rotate the normals as well.
    let rotated_normals: Vec<[f32; 3]> = normals
        .into_iter()
        .map(|n| {
            let norm = Vec3::from(n);
            [norm.x, norm.y, norm.z]
        })
        .collect();

    // The indices remain unchanged.
    let indices = Indices::U32(vec![
        // Top face (+y)
        0, 3, 1, 1, 3, 2,
        // Bottom face (-y)
        4, 5, 7, 5, 6, 7,
        // Right face (+x)
        8, 11, 9, 9, 11, 10,
        // Left face (-x)
        12, 13, 15, 13, 14, 15,
        // Back face (+z)
        16, 19, 17, 17, 19, 18,
        // Forward face (-z)
        20, 21, 23, 21, 22, 23,
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
    // === Texture Atlas Setup (same as your current code) ===
    let num_tiles_x = 6.0;
    let num_rows_f = NUM_VOXELS as f32;
    let v_top = tile_row as f32 / num_rows_f;
    let v_bottom = (tile_row as f32 + 1.0) / num_rows_f;

    // Geometry buffers for positions, UVs, normals, and indices.
    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0u32; // Running count of vertices

    // --- Helper: Add a cuboid (or rectangular prism) with correct per-face normals ---
    fn add_cuboid(
        positions: &mut Vec<[f32; 3]>,
        uvs: &mut Vec<[f32; 2]>,
        normals: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        index_offset: &mut u32,
        dims: [f32; 3],   // full width, height, depth
        offset: [f32; 3], // center offset
        v_top: f32,
        v_bottom: f32,
    ) {
        // Compute half-dimensions.
        // (Using the same 0.8 factor from your code)
        let hx = dims[0] * 0.8;
        let hy = dims[1] * 0.8;
        let hz = dims[2] * 0.8;

        // Helper closure to push a face
        let mut push_face = |face_positions: &[[f32; 3]; 4],
                               face_uvs: &[[f32; 2]; 4],
                               normal: [f32; 3]| {
            let start = *index_offset;
            positions.extend_from_slice(face_positions);
            uvs.extend_from_slice(face_uvs);
            for _ in 0..4 {
                normals.push(normal);
            }
            // Two triangles: 0,1,2 and 0,2,3
            indices.extend_from_slice(&[start, start + 1, start + 2, start, start + 2, start + 3]);
            *index_offset += 4;
        };

        // Top face (+Y): y = offset[1] + hy, normal = [0,1,0]
        push_face(
            &[
                [offset[0] - hx, offset[1] + hy, offset[2] - hz], // bottom left
                [offset[0] - hx, offset[1] + hy, offset[2] + hz], // top left
                [offset[0] + hx, offset[1] + hy, offset[2] + hz], // top right
                [offset[0] + hx, offset[1] + hy, offset[2] - hz], // bottom right
            ],
            &[
                [0.0, v_bottom],
                [0.0, v_top],
                [1.0, v_top],
                [1.0, v_bottom],
            ],
            [0.0, 1.0, 0.0],
        );

        // Bottom face (-Y): y = offset[1] - hy, normal = [0,-1,0]
        push_face(
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] - hz], // bottom left
                [offset[0] + hx, offset[1] - hy, offset[2] - hz], // bottom right
                [offset[0] + hx, offset[1] - hy, offset[2] + hz], // top right
                [offset[0] - hx, offset[1] - hy, offset[2] + hz], // top left
            ],
            &[
                [0.0, v_bottom],
                [1.0, v_bottom],
                [1.0, v_top],
                [0.0, v_top],
            ],
            [0.0, -1.0, 0.0],
        );

        // Right face (+X): x = offset[0] + hx, normal = [1,0,0]
        push_face(
            &[
                [offset[0] + hx, offset[1] - hy, offset[2] - hz], // bottom left
                [offset[0] + hx, offset[1] + hy, offset[2] - hz], // top left
                [offset[0] + hx, offset[1] + hy, offset[2] + hz], // top right
                [offset[0] + hx, offset[1] - hy, offset[2] + hz], // bottom right
            ],
            &[
                [0.0, v_bottom],
                [0.0, v_top],
                [1.0, v_top],
                [1.0, v_bottom],
            ],
            [1.0, 0.0, 0.0],
        );

        // Left face (-X): x = offset[0] - hx, normal = [-1,0,0]
        push_face(
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] + hz], // bottom left
                [offset[0] - hx, offset[1] + hy, offset[2] + hz], // top left
                [offset[0] - hx, offset[1] + hy, offset[2] - hz], // top right
                [offset[0] - hx, offset[1] - hy, offset[2] - hz], // bottom right
            ],
            &[
                [0.0, v_bottom],
                [0.0, v_top],
                [1.0, v_top],
                [1.0, v_bottom],
            ],
            [-1.0, 0.0, 0.0],
        );

        // Front face (-Z): z = offset[2] - hz, normal = [0,0,-1]
        push_face(
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] - hz], // bottom left
                [offset[0] - hx, offset[1] + hy, offset[2] - hz], // top left
                [offset[0] + hx, offset[1] + hy, offset[2] - hz], // top right
                [offset[0] + hx, offset[1] - hy, offset[2] - hz], // bottom right
            ],
            &[
                [0.0, v_bottom],
                [0.0, v_top],
                [1.0, v_top],
                [1.0, v_bottom],
            ],
            [0.0, 0.0, -1.0],
        );

        // Back face (+Z): z = offset[2] + hz, normal = [0,0,1]
        push_face(
            &[
                [offset[0] - hx, offset[1] - hy, offset[2] + hz], // bottom left
                [offset[0] + hx, offset[1] - hy, offset[2] + hz], // bottom right
                [offset[0] + hx, offset[1] + hy, offset[2] + hz], // top right
                [offset[0] - hx, offset[1] + hy, offset[2] + hz], // top left
            ],
            &[
                [0.0, v_bottom],
                [1.0, v_bottom],
                [1.0, v_top],
                [0.0, v_top],
            ],
            [0.0, 0.0, 1.0],
        );
    }

    // --- 1. Add the Cable Core ---
    let core_dims = [0.2, 0.2, 0.2]; // width, height, depth
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
    );

    // --- 2. Add Extensions Based on Connectivity ---
    // Note: Adjust the offsets for each extension so they extend in the correct directions.
    if connections[0] {
        // +X extension.
        let ext_width = 0.5 - core_dims[0] / 2.0;
        let ext_dims = [ext_width, 0.2, 0.2];
        // Change offset to extend from the +X side.
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
        );
    }
    if connections[1] {
        // -X extension.
        let ext_width = 0.5 - core_dims[0] / 2.0;
        let ext_dims = [ext_width, 0.2, 0.2];
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
        );
    }
    if connections[2] {
        // +Y extension.
        let ext_height = 0.5 - core_dims[1] / 2.0;
        let ext_dims = [0.2, ext_height, 0.2];
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
        );
    }
    if connections[3] {
        // -Y extension.
        let ext_height = 0.5 - core_dims[1] / 2.0;
        let ext_dims = [0.2, ext_height, 0.2];
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
        );
    }
    if connections[4] {
        // +Z extension.
        let ext_depth = 0.5 - core_dims[2] / 2.0;
        let ext_dims = [0.2, 0.2, ext_depth];
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
        );
    }
    if connections[5] {
        // -Z extension.
        let ext_depth = 0.5 - core_dims[2] / 2.0;
        let ext_dims = [0.2, 0.2, ext_depth];
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
        );
    }

    // --- 3. Build and Return the Mesh ---
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}