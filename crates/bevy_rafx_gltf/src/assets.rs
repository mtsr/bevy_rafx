use serde::{Deserialize, Serialize};
// use shaders::mesh_frag::MaterialDataStd140;

//TODO: These are extensions that might be interesting to try supporting. In particular, lights,
// LOD, and clearcoat
// Good explanations of upcoming extensions here: https://medium.com/@babylonjs/gltf-extensions-in-babylon-js-b3fa56de5483
//KHR_materials_clearcoat: https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Khronos/KHR_materials_clearcoat/README.md
//KHR_materials_pbrSpecularGlossiness: https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Khronos/KHR_materials_pbrSpecularGlossiness/README.md
//KHR_materials_unlit: https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Khronos/KHR_materials_unlit/README.md
//KHR_lights_punctual (directional, point, spot): https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Khronos/KHR_lights_punctual/README.md
//EXT_lights_image_based: https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Vendor/EXT_lights_image_based/README.md
//MSFT_lod: https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Vendor/MSFT_lod/README.md
//MSFT_packing_normalRoughnessMetallic: https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Vendor/MSFT_packing_normalRoughnessMetallic/README.md
// Normal: NG, Roughness: B, Metallic: A
//MSFT_packing_occlusionRoughnessMetallic: https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Vendor/MSFT_packing_occlusionRoughnessMetallic/README.md

// This is non-texture data associated with the material. Must convert to
// GltfMaterialDataShaderParam to bind to a shader uniform
#[derive(Serialize, Deserialize, Clone)]
#[repr(C)]
pub struct GltfMaterialData {
    // Using f32 arrays for serde support
    pub base_color_factor: [f32; 4],     // default: 1,1,1,1
    pub emissive_factor: [f32; 3],       // default: 0,0,0
    pub metallic_factor: f32,            //default: 1,
    pub roughness_factor: f32,           // default: 1,
    pub normal_texture_scale: f32,       // default: 1
    pub occlusion_texture_strength: f32, // default 1
    pub alpha_cutoff: f32,               // default 0.5

    pub has_base_color_texture: bool,
    pub has_metallic_roughness_texture: bool,
    pub has_normal_texture: bool,
    pub has_occlusion_texture: bool,
    pub has_emissive_texture: bool,
}

impl Default for GltfMaterialData {
    fn default() -> Self {
        GltfMaterialData {
            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            emissive_factor: [0.0, 0.0, 0.0],
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            normal_texture_scale: 1.0,
            occlusion_texture_strength: 1.0,
            alpha_cutoff: 0.5,
            has_base_color_texture: false,
            has_metallic_roughness_texture: false,
            has_normal_texture: false,
            has_occlusion_texture: false,
            has_emissive_texture: false,
        }
    }
}

// TODO
// pub type GltfMaterialDataShaderParam = MaterialDataStd140;

// impl Into<MaterialDataStd140> for GltfMaterialData {
//     fn into(self) -> MaterialDataStd140 {
//         MaterialDataStd140 {
//             base_color_factor: self.base_color_factor.into(),
//             emissive_factor: self.emissive_factor.into(),
//             metallic_factor: self.metallic_factor,
//             roughness_factor: self.roughness_factor,
//             normal_texture_scale: self.normal_texture_scale,
//             occlusion_texture_strength: self.occlusion_texture_strength,
//             alpha_cutoff: self.alpha_cutoff,
//             has_base_color_texture: if self.has_base_color_texture { 1 } else { 0 },
//             has_metallic_roughness_texture: if self.has_metallic_roughness_texture {
//                 1
//             } else {
//                 0
//             },
//             has_normal_texture: if self.has_normal_texture { 1 } else { 0 },
//             has_occlusion_texture: if self.has_occlusion_texture { 1 } else { 0 },
//             has_emissive_texture: if self.has_emissive_texture { 1 } else { 0 },
//             ..Default::default()
//         }
//     }
// }
