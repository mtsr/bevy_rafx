use std::{io::Write, path::Path, sync::Arc};

use bevy::{
    asset,
    core::Time,
    log,
    math::UVec2,
    prelude::{App, IntoSystem, Plugin, Res, ResMut, StageLabel, SystemStage},
    window::Windows,
    winit::WinitWindows,
};
use rafx::{
    api::{
        RafxApi, RafxApiDef, RafxBufferDef, RafxColorClearValue, RafxCommandBuffer, RafxExtents2D,
        RafxFormat, RafxPrimitiveTopology, RafxQueue, RafxQueueType, RafxResourceState,
        RafxResourceType, RafxSampleCount, RafxSwapchainDef, RafxSwapchainHelper,
        RafxVertexBufferBinding,
    },
    framework::{
        CookedShaderPackage, DescriptorSetBindings, FixedFunctionState, MaterialPass,
        ResourceManager, VertexDataLayout, VertexDataSetLayout,
    },
    graph::{
        PreparedRenderGraph, RenderGraphBuilder, RenderGraphImageConstraint,
        RenderGraphImageExtents, RenderGraphImageSpecification, RenderGraphQueue,
        SwapchainSurfaceInfo,
    },
    nodes::{PreparedRenderData, RenderPhase, RenderPhaseIndex, SubmitNode},
    render_feature_write_job_prelude::RafxResult,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    /// Stage where render resources are set up
    RenderResource,
    /// Stage where Render Graph systems are run. In general you shouldn't add systems to this
    /// stage manually.
    RenderGraphSystems,
    // Stage where draw systems are executed. This is generally where Draw components are setup
    Draw,
    Render,
    PostRender,
}

#[derive(Default)]
pub struct RafxPlugin;

impl Plugin for RafxPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(BevyRafx::default())
            .add_startup_system(setup.system())
            .add_stage_after(
                asset::AssetStage::AssetEvents,
                RenderStage::Render,
                SystemStage::parallel(),
            )
            // .set_runner(print_schedule_runner)
            .add_system_to_stage(RenderStage::Render, render.system());
    }
}

fn setup(windows: Res<Windows>, winit_windows: Res<WinitWindows>, mut bevy_rafx: ResMut<BevyRafx>) {
    let window_id = {
        windows
            .get_primary()
            .expect("No primary window available")
            .id()
    };

    let window = winit_windows
        .get_window(window_id)
        .expect("Primary window not found in WinitWindows");

    let rafx_api = unsafe {
        rafx::api::RafxApi::new(window, &RafxApiDef {}).expect("RafxApi failed to create")
    };

    let device_context = rafx_api.device_context();

    let window = windows.get_primary().unwrap();
    let window_size = UVec2::new(window.physical_width(), window.physical_height());

    let winit_window = winit_windows.get_window(window.id()).unwrap();

    let swapchain = device_context
        .create_swapchain(
            winit_window,
            &RafxSwapchainDef {
                width: window_size.x,
                height: window_size.y,
                enable_vsync: true,
            },
        )
        .unwrap();

    let mut swapchain_helper = RafxSwapchainHelper::new(&device_context, swapchain, None).unwrap();

    let graphics_queue = device_context
        .create_queue(RafxQueueType::Graphics)
        .unwrap();

    let render_registry = rafx::nodes::RenderRegistryBuilder::default()
        .register_render_phase::<OpaqueRenderPhase>("Opaque")
        .build();

    let mut resource_manager =
        rafx::framework::ResourceManager::new(&device_context, &render_registry);

    let resource_context = resource_manager.resource_context();

    let cooked_shaders_base_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/cooked_shaders");

    // Create the vertex shader module and find the entry point
    let cooked_vertex_shader_stage =
        load_cooked_shader_stage(&cooked_shaders_base_path, "shader.vert.cookedshaderpackage")
            .unwrap();
    let vertex_shader_module = resource_context
        .resources()
        .get_or_create_shader_module_from_cooked_package(&cooked_vertex_shader_stage)
        .unwrap();
    let vertex_entry_point = cooked_vertex_shader_stage
        .find_entry_point("main")
        .unwrap()
        .clone();

    // Create the fragment shader module and find the entry point
    let cooked_fragment_shader_stage =
        load_cooked_shader_stage(&cooked_shaders_base_path, "shader.frag.cookedshaderpackage")
            .unwrap();
    let fragment_shader_module = resource_context
        .resources()
        .get_or_create_shader_module_from_cooked_package(&cooked_fragment_shader_stage)
        .unwrap();
    let fragment_entry_point = cooked_fragment_shader_stage
        .find_entry_point("main")
        .unwrap()
        .clone();

    let fixed_function_state = Arc::new(FixedFunctionState {
        rasterizer_state: Default::default(),
        depth_state: Default::default(),
        blend_state: Default::default(),
    });

    let material_pass = MaterialPass::new(
        &resource_context,
        fixed_function_state,
        vec![vertex_shader_module, fragment_shader_module],
        &[&vertex_entry_point, &fragment_entry_point],
    )
    .unwrap();

    resource_manager
        .graphics_pipeline_cache()
        .register_material_to_phase_index(
            &material_pass.material_pass_resource,
            OpaqueRenderPhase::render_phase_index(),
        );

    let vertex_layout = Arc::new(
        VertexDataLayout::build_vertex_layout(
            &PositionColorVertex::default(),
            |builder, vertex| {
                builder.add_member(&vertex.position, "POSITION", RafxFormat::R32G32_SFLOAT);
                builder.add_member(&vertex.color, "COLOR", RafxFormat::R32G32B32_SFLOAT);
            },
        )
        .into_set(RafxPrimitiveTopology::TriangleList),
    );

    bevy_rafx.rafx_api.replace(rafx_api);
    bevy_rafx.swapchain_helper.replace(swapchain_helper);
    bevy_rafx.resource_manager.replace(resource_manager);
    bevy_rafx.vertex_layout.replace(vertex_layout);
    bevy_rafx.material_pass.replace(material_pass);
    bevy_rafx.graphics_queue.replace(graphics_queue);
}

#[derive(Default)]
struct BevyRafx {
    rafx_api: Option<RafxApi>,
    swapchain_helper: Option<RafxSwapchainHelper>,
    resource_manager: Option<ResourceManager>,
    vertex_layout: Option<Arc<VertexDataSetLayout>>,
    material_pass: Option<MaterialPass>,
    graphics_queue: Option<RafxQueue>,
}

fn render(mut bevy_rafx: ResMut<BevyRafx>, windows: Res<Windows>, time: Res<Time>) {
    let BevyRafx {
        rafx_api,
        swapchain_helper,
        resource_manager,
        vertex_layout,
        material_pass,
        graphics_queue,
        ..
    } = &mut *bevy_rafx;
    let rafx_api = rafx_api.as_ref().unwrap();
    let swapchain_helper = swapchain_helper.as_mut().unwrap();
    let resource_manager = resource_manager.as_mut().unwrap();
    let resource_context = resource_manager.resource_context();
    let vertex_layout = vertex_layout.as_ref().unwrap().clone();
    let material_pass = material_pass.as_ref().unwrap();
    let graphics_queue = graphics_queue.as_ref().unwrap();

    let window = windows.get_primary().unwrap();
    let window_size = UVec2::new(window.physical_width(), window.physical_height());

    let presentable_frame = swapchain_helper
        .acquire_next_image(window_size.x, window_size.y, None)
        .unwrap();

    resource_manager.on_frame_complete().unwrap();

    let swapchain_image = resource_context
        .resources()
        .insert_image(presentable_frame.swapchain_texture().clone());

    let swapchain_image_view = resource_context
        .resources()
        .get_or_create_image_view(&swapchain_image, None)
        .unwrap();

    let mut graph_builder = RenderGraphBuilder::default();

    let node = graph_builder.add_node("opaque", RenderGraphQueue::DefaultGraphics);
    let color_attachment = graph_builder.create_color_attachment(
        node,
        0,
        Some(RafxColorClearValue([0.0, 0.0, 0.0, 0.0])),
        RenderGraphImageConstraint {
            samples: Some(RafxSampleCount::SampleCount4),
            format: Some(swapchain_helper.format()),
            ..Default::default()
        },
        Default::default(),
    );
    graph_builder.set_image_name(color_attachment, "color");

    let captured_vertex_layout = vertex_layout.clone();
    let captured_material_pass = material_pass.clone();
    let seconds = time.seconds_since_startup() as f32;
    graph_builder.set_renderpass_callback(node, move |args| {
        let vertex_layout = &captured_vertex_layout;
        let material_pass = &captured_material_pass;

        #[rustfmt::skip]
        let vertex_data = [
            PositionColorVertex { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
            PositionColorVertex { position: [-0.5 + (seconds.cos() / 2. + 0.5), -0.5], color: [0.0, 1.0, 0.0] },
            PositionColorVertex { position: [0.5 - (seconds.cos() / 2. + 0.5), -0.5], color: [0.0, 0.0, 1.0] },
        ];

        assert_eq!(20, std::mem::size_of::<PositionColorVertex>());

        let color = (seconds.cos() + 1.0) / 2.0;
        let uniform_data = [color, 0.0, 1.0 - color, 1.0];

        let resource_allocator = args.graph_context.resource_context().create_dyn_resource_allocator_set();
        let vertex_buffer = args.graph_context.device_context().create_buffer(
            &RafxBufferDef::for_staging_vertex_buffer_data(&vertex_data)
        )?;

        vertex_buffer.copy_to_host_visible_buffer(&vertex_data)?;

        let vertex_buffer = resource_allocator.insert_buffer(vertex_buffer);

        let descriptor_set_layout = material_pass.material_pass_resource
        .get_raw()
        .descriptor_set_layouts[0]
        .clone();

        let mut descriptor_set_allocator = args.graph_context.resource_context().create_descriptor_set_allocator();
        let mut dyn_descriptor_set = descriptor_set_allocator.create_dyn_descriptor_set_uninitialized(&descriptor_set_layout)?;
        dyn_descriptor_set.set_buffer_data(0, &uniform_data);
        dyn_descriptor_set.flush(&mut descriptor_set_allocator).unwrap();
        descriptor_set_allocator.flush_changes().unwrap();

        let descriptor_set = dyn_descriptor_set.descriptor_set();

        let pipeline = args
        .graph_context
        .resource_context()
        .graphics_pipeline_cache()
        .get_or_create_graphics_pipeline(
            OpaqueRenderPhase::render_phase_index(),
            &material_pass.material_pass_resource,
            &args.render_target_meta,
            &vertex_layout
        ).unwrap();

        let cmd_buffer = args.command_buffer;
        cmd_buffer.cmd_bind_pipeline(&pipeline.get_raw().pipeline)?;
        cmd_buffer.cmd_bind_vertex_buffers(
            0,
            &[RafxVertexBufferBinding {
                buffer: &vertex_buffer.get_raw().buffer,
                byte_offset: 0,
            }],
        )?;

        descriptor_set.bind(&cmd_buffer)?;
        cmd_buffer.cmd_draw(3, 0)?;

        Ok(())
    });

    graph_builder.set_output_image(
        color_attachment,
        swapchain_image_view,
        RenderGraphImageSpecification {
            samples: RafxSampleCount::SampleCount1,
            format: swapchain_helper.format(),
            resource_type: RafxResourceType::TEXTURE | RafxResourceType::RENDER_TARGET_COLOR,
            extents: RenderGraphImageExtents::MatchSurface,
            layer_count: 1,
            mip_count: 1,
        },
        Default::default(),
        RafxResourceState::PRESENT,
    );

    let swapchain_def = swapchain_helper.swapchain_def();
    let swapchain_surface_info = SwapchainSurfaceInfo {
        format: swapchain_helper.format(),
        extents: RafxExtents2D {
            width: swapchain_def.width,
            height: swapchain_def.height,
        },
    };

    let executor = PreparedRenderGraph::new(
        &rafx_api.device_context(),
        &resource_context,
        graph_builder,
        &swapchain_surface_info,
    )
    .unwrap();

    let command_buffers = executor
        .execute_graph(PreparedRenderData::empty(), &graphics_queue)
        .unwrap();

    let refs: Vec<&RafxCommandBuffer> = command_buffers.iter().map(|x| &**x).collect();
    presentable_frame.present(&graphics_queue, &refs).unwrap();
}

#[derive(Default, Clone, Copy)]
struct PositionColorVertex {
    position: [f32; 2],
    color: [f32; 3],
}

fn load_cooked_shader_stage(
    base_path: &Path,
    shader_file: &str,
) -> RafxResult<CookedShaderPackage> {
    let cooked_shader_path = base_path.join(shader_file);
    let bytes = std::fs::read(cooked_shader_path).unwrap();

    let cooked_shader = bincode::deserialize::<CookedShaderPackage>(&bytes)
        .map_err(|x| format!("Failed to deserialize cooked shader: {:?}", x))
        .unwrap();

    Ok(cooked_shader)
}

rafx::declare_render_phase!(
    OpaqueRenderPhase,
    OPAQUE_RENDER_PHASE_INDEX,
    opaque_render_phase_sort_submit_nodes
);

fn opaque_render_phase_sort_submit_nodes(mut submit_nodes: Vec<SubmitNode>) -> Vec<SubmitNode> {
    // Sort by feature
    log::trace!(
        "Sort phase {}",
        OpaqueRenderPhase::render_phase_debug_name()
    );
    submit_nodes.sort_unstable_by_key(|a| a.feature_index());

    submit_nodes
}

fn print_schedule_runner(app: App) {
    let dot = bevy_mod_debugdump::schedule_graph_dot(&app.schedule);
    let mut file = std::fs::File::create("schedule.dot").unwrap();
    write!(file, "{}", dot).unwrap();
    println!("*** Updated schedule.dot");
}
