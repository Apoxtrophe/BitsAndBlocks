use crate::prelude::*;

pub fn update_emissive (
    graphics_query: Query<(&Voxel, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (voxel, material_handle) in graphics_query.iter() {

        if voxel.state == true {
            if let Some(material) = materials.get_mut(material_handle) {
                material.emissive = LinearRgba::new(0.0, 0.4, 0.0, 1.0);
            }
        } else {
            if let Some(material) = materials.get_mut(material_handle) {
                material.emissive = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
            }
        }
    }
}