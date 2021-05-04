use bevy::prelude::{
    AddAsset, Added, Assets, ChangeTrackers, Entity, GlobalTransform, Handle, Or, Plugin, Query,
    QuerySet, Res, ResMut, Transform, With,
};
use bevy::{
    ecs::{bundle::Bundle, system::IntoSystem},
    prelude::Changed,
};

pub use bevy_pbr::prelude::StandardMaterial;
pub use bevy_render::mesh::VertexAttributeValues;
pub use bevy_render::{
    color::Color,
    mesh::{self, Mesh},
    pipeline::PrimitiveTopology,
    texture,
};

use bevy_rafx_plugin::{RenderStage, VisibilityComponent};
use rafx::{
    base::slab::DropSlabKey,
    nodes::GenericRenderNodeHandle,
    nodes::RenderRegistryBuilder,
    rafx_visibility::{PolygonSoup, PolygonSoupIndex},
    visibility::{CullModel, EntityId, VisibilityRegion},
};

use rafx::render_feature_mod_prelude::*;
rafx::declare_render_feature!(MeshRenderFeature, MESH_FEATURE_INDEX);

mod extract;
mod mesh_render_node_set;

#[derive(Bundle, Default)]
pub struct PbrBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    // pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility_component: VisibilityComponent,
}

#[derive(Default)]
pub struct MeshRendererPlugin {}

impl Plugin for MeshRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.register_type::<VisibilityComponent>()
            .add_asset::<Mesh>()
            .add_asset::<StandardMaterial>()
            .add_startup_system(setup.system())
            .add_system_to_stage(RenderStage::Visibility, mesh_update_visibility.system())
            .add_system_to_stage(RenderStage::Extract, mesh_extract.system());
    }
}

fn setup(mut render_registry_builder_resource: ResMut<Option<RenderRegistryBuilder>>) {
    let render_registry_builder = render_registry_builder_resource
        .take()
        .unwrap()
        .register_feature::<MeshRenderFeature>();
    render_registry_builder_resource.replace(render_registry_builder);
}

fn mesh_update_visibility(
    mut query: Query<(
        Entity,
        &Handle<Mesh>,
        &Transform,
        &mut VisibilityComponent,
        ChangeTrackers<Handle<Mesh>>,
        ChangeTrackers<Transform>,
    )>,
    visibility_region: Res<VisibilityRegion>,
    meshes: Res<Assets<Mesh>>,
) {
    query.for_each_mut(
        |(
            entity,
            mesh_handle,
            transform,
            mut visibility_component,
            change_trackers_mesh_handle,
            change_trackers_transform,
        )| {
            // New VisibilityComponent
            if visibility_component.is_added() {
                let mesh = meshes.get(mesh_handle).unwrap();

                let cull_model = mesh_to_cull_model(mesh);

                let handle =
                    visibility_region.register_dynamic_object(EntityId::from(entity), cull_model);

                handle.set_transform(transform.translation, transform.rotation, transform.scale);

                // TODO
                // handle.add_feature(MeshRenderNodeHandle)

                visibility_component.handle.replace(handle);
            } else {
                if change_trackers_mesh_handle.is_changed() {
                    let mesh = meshes.get(mesh_handle).unwrap();

                    let cull_model = mesh_to_cull_model(mesh);

                    // TODO cannot currently update CullModel
                    // visibility_component
                    //     .handle
                    //     .unwrap()
                    //     .set_cull_model(Some(cull_model));

                    let handle = visibility_region
                        .register_dynamic_object(EntityId::from(entity), cull_model);

                    handle.set_transform(
                        transform.translation,
                        transform.rotation,
                        transform.scale,
                    );

                    visibility_component.handle.replace(handle);
                }
                if change_trackers_transform.is_changed() {
                    visibility_component.handle.as_ref().unwrap().set_transform(
                        transform.translation,
                        transform.rotation,
                        transform.scale,
                    );
                }
            }
        },
    );
}

fn mesh_to_cull_model(mesh: &Mesh) -> CullModel {
    let vertex_positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
        VertexAttributeValues::Float32x3(positions) => positions,
        _ => panic!(),
    }
    .iter()
    .map(|&floats| floats.into())
    .collect();

    let index = match mesh.indices().unwrap() {
        mesh::Indices::U16(u16) => PolygonSoupIndex::Indexed16(u16.clone()),
        mesh::Indices::U32(u32) => PolygonSoupIndex::Indexed32(u32.clone()),
    };

    CullModel::mesh(PolygonSoup {
        vertex_positions,
        index,
    })
}

fn mesh_extract() {}
