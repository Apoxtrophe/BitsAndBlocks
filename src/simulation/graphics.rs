use crate::prelude::*;

pub fn update_emissive (
    graphics_query: Query<(&Voxel, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (voxel, material_handle) in graphics_query.iter() {

        if Bits16::any_set(voxel.state){
            if let Some(material) = materials.get_mut(&material_handle.0) {
                match voxel.kind {
                VoxelType::Component(ComponentVariants::Light) => {
                      material.emissive = LinearRgba::new(0.4, 0.8, 0.4, 0.8);
                  }
                  _ => {
                      material.emissive = LinearRgba::new(0.0, 0.1, 0.0, 0.2);  
                  }
                }
                

            }
        } else {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.emissive = LinearRgba::new(0.0, 0.0, 0.0, 1.0);
            }
        }
    }
}