// use crate::phases::OpaqueRenderPhase;
use rafx::api::RafxResult;
use rafx::assets::distill::loader::handle::Handle;
use rafx::assets::MaterialInstanceAsset;
use rafx::assets::{
    AssetManager, BufferAsset, DefaultAssetTypeHandler, DefaultAssetTypeLoadHandler,
};
use rafx::framework::{BufferResource, DescriptorSetArc, MaterialPass, ResourceArc};
use rafx::rafx_visibility::VisibleBounds;
use serde::{Deserialize, Serialize};
// use shaders::mesh_frag::MaterialDataStd140;
use std::sync::Arc;
use type_uuid::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct MeshPartAssetData {
    pub vertex_buffer_offset_in_bytes: u32,
    pub vertex_buffer_size_in_bytes: u32,
    pub index_buffer_offset_in_bytes: u32,
    pub index_buffer_size_in_bytes: u32,
    pub material_instance: Handle<MaterialInstanceAsset>,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone)]
#[uuid = "cf232526-3757-4d94-98d1-c2f7e27c979f"]
pub struct MeshAssetData {
    pub mesh_parts: Vec<MeshPartAssetData>,
    pub vertex_buffer: Handle<BufferAsset>, //Vec<MeshVertex>,
    pub index_buffer: Handle<BufferAsset>,  //Vec<u16>,
    pub visible_bounds: VisibleBounds,
}

pub struct MeshAssetPart {
    pub opaque_pass: MaterialPass,
    pub opaque_material_descriptor_set: DescriptorSetArc,
    pub vertex_buffer_offset_in_bytes: u32,
    pub vertex_buffer_size_in_bytes: u32,
    pub index_buffer_offset_in_bytes: u32,
    pub index_buffer_size_in_bytes: u32,
}

pub struct MeshAssetInner {
    pub mesh_parts: Vec<Option<MeshAssetPart>>,
    pub vertex_buffer: ResourceArc<BufferResource>,
    pub index_buffer: ResourceArc<BufferResource>,
    pub asset_data: MeshAssetData,
}

#[derive(TypeUuid, Clone)]
#[uuid = "689a0bf0-e320-41c0-b4e8-bdb2055a7a57"]
pub struct MeshAsset {
    pub inner: Arc<MeshAssetInner>,
}

pub struct MeshLoadHandler;

impl DefaultAssetTypeLoadHandler<MeshAssetData, MeshAsset> for MeshLoadHandler {
    fn load(asset_manager: &mut AssetManager, mesh_asset: MeshAssetData) -> RafxResult<MeshAsset> {
        let vertex_buffer = asset_manager
            .latest_asset(&mesh_asset.vertex_buffer)
            .unwrap()
            .buffer
            .clone();
        let index_buffer = asset_manager
            .latest_asset(&mesh_asset.index_buffer)
            .unwrap()
            .buffer
            .clone();

        // TODO
        //     let mesh_parts: Vec<_> = mesh_asset
        // .mesh_parts
        // .iter()
        // .map(|mesh_part| {
        //     let material_instance = asset_manager
        //         .committed_asset(&mesh_part.material_instance)
        //         .unwrap();

        //     let opaque_pass_index = material_instance
        //         .material
        //         .find_pass_by_phase::<OpaqueRenderPhase>();

        //     if opaque_pass_index.is_none() {
        //         log::error!(
        //             "A mesh part with material {:?} has no opaque phase",
        //             material_instance.material_handle
        //         );
        //         return None;
        //     }

        //     let opaque_pass_index = opaque_pass_index.unwrap();

        //     const PER_MATERIAL_DESCRIPTOR_SET_LAYOUT_INDEX: usize = 1;

        //     Some(MeshAssetPart {
        //         opaque_pass: material_instance.material.passes[opaque_pass_index].clone(),
        //         opaque_material_descriptor_set: material_instance.material_descriptor_sets
        //             [opaque_pass_index][PER_MATERIAL_DESCRIPTOR_SET_LAYOUT_INDEX]
        //             .as_ref()
        //             .unwrap()
        //             .clone(),
        //         vertex_buffer_offset_in_bytes: mesh_part.vertex_buffer_offset_in_bytes,
        //         vertex_buffer_size_in_bytes: mesh_part.vertex_buffer_size_in_bytes,
        //         index_buffer_offset_in_bytes: mesh_part.index_buffer_offset_in_bytes,
        //         index_buffer_size_in_bytes: mesh_part.index_buffer_size_in_bytes,
        //     })
        // })
        // .collect();
        let mesh_parts = vec![];

        let inner = MeshAssetInner {
            vertex_buffer,
            index_buffer,
            asset_data: mesh_asset,
            mesh_parts,
        };

        Ok(MeshAsset {
            inner: Arc::new(inner),
        })
    }
}

pub type MeshAssetType = DefaultAssetTypeHandler<MeshAssetData, MeshAsset, MeshLoadHandler>;
