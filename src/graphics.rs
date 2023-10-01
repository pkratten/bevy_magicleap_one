use ash::vk::{self, CommandBuffer, CommandBufferAllocateInfo, Handle, PipelineStageFlags};
use bevy::render::camera::{ExtractedCamera, ManualTextureView, NormalizedRenderTarget, Viewport};
use bevy::render::camera::{ManualTextureViewHandle, ManualTextureViews};
use bevy::render::render_resource::TextureView;
use bevy::render::renderer::RenderInstance;
use bevy::render::texture::BevyDefault;
use bevy::utils::HashMap;
use bevy::{prelude::*, render::renderer::RenderDevice};
use magicleap_one_lumin_sdk_sys::magicleap_c_api;
use std::mem;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;
use wgpu;
use wgpu_hal::TextureUses;

#[derive(Resource, Deref, Clone)]
pub struct MagicLeapGraphicsHandle(u64);

// pub fn initialize_renderer(app: &mut App) {

//     let options = app
//             .world
//             .get_resource::<bevy::render::settings::WgpuSettings>()
//             .cloned()
//             .unwrap_or_default();

//             let instance = app
//             .world
//             .remove_resource::<RenderInstance>()
//             .unwrap_or_else(|| RenderInstance(wgpu::Instance::new(options.backends)));

//     request_adapter_options: &RequestAdapterOptions;

//     let adapter = instance
//         .request_adapter(request_adapter_options)
//         .await
//         .expect();

//     let adapter_info = adapter.get_info();
//     info!("{:?}", adapter_info);

//     #[cfg(feature = "wgpu_trace")]
//     let trace_path = {
//         let path = std::path::Path::new("wgpu_trace");
//         // ignore potential error, wgpu will log it
//         let _ = std::fs::create_dir(path);
//         Some(path)
//     };
//     #[cfg(not(feature = "wgpu_trace"))]
//     let trace_path = None;

//     // Maybe get features and limits based on what is supported by the adapter/backend
//     let mut features = wgpu::Features::empty();
//     let mut limits = options.limits.clone();
//     if matches!(options.priority, WgpuSettingsPriority::Functionality) {
//         features = adapter.features() | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;
//         if adapter_info.device_type == wgpu::DeviceType::DiscreteGpu {
//             // `MAPPABLE_PRIMARY_BUFFERS` can have a significant, negative performance impact for
//             // discrete GPUs due to having to transfer data across the PCI-E bus and so it
//             // should not be automatically enabled in this case. It is however beneficial for
//             // integrated GPUs.
//             features -= wgpu::Features::MAPPABLE_PRIMARY_BUFFERS;
//         }
//         limits = adapter.limits();
//     }

//     // Enforce the disabled features
//     if let Some(disabled_features) = options.disabled_features {
//         features -= disabled_features;
//     }
//     // NOTE: |= is used here to ensure that any explicitly-enabled features are respected.
//     features |= options.features;

//     // Enforce the limit constraints
//     if let Some(constrained_limits) = options.constrained_limits.as_ref() {
//         // NOTE: Respect the configured limits as an 'upper bound'. This means for 'max' limits, we
//         // take the minimum of the calculated limits according to the adapter/backend and the
//         // specified max_limits. For 'min' limits, take the maximum instead. This is intended to
//         // err on the side of being conservative. We can't claim 'higher' limits that are supported
//         // but we can constrain to 'lower' limits.
//         limits = wgpu::Limits {
//             max_texture_dimension_1d: limits
//                 .max_texture_dimension_1d
//                 .min(constrained_limits.max_texture_dimension_1d),
//             max_texture_dimension_2d: limits
//                 .max_texture_dimension_2d
//                 .min(constrained_limits.max_texture_dimension_2d),
//             max_texture_dimension_3d: limits
//                 .max_texture_dimension_3d
//                 .min(constrained_limits.max_texture_dimension_3d),
//             max_texture_array_layers: limits
//                 .max_texture_array_layers
//                 .min(constrained_limits.max_texture_array_layers),
//             max_bind_groups: limits
//                 .max_bind_groups
//                 .min(constrained_limits.max_bind_groups),
//             max_dynamic_uniform_buffers_per_pipeline_layout: limits
//                 .max_dynamic_uniform_buffers_per_pipeline_layout
//                 .min(constrained_limits.max_dynamic_uniform_buffers_per_pipeline_layout),
//             max_dynamic_storage_buffers_per_pipeline_layout: limits
//                 .max_dynamic_storage_buffers_per_pipeline_layout
//                 .min(constrained_limits.max_dynamic_storage_buffers_per_pipeline_layout),
//             max_sampled_textures_per_shader_stage: limits
//                 .max_sampled_textures_per_shader_stage
//                 .min(constrained_limits.max_sampled_textures_per_shader_stage),
//             max_samplers_per_shader_stage: limits
//                 .max_samplers_per_shader_stage
//                 .min(constrained_limits.max_samplers_per_shader_stage),
//             max_storage_buffers_per_shader_stage: limits
//                 .max_storage_buffers_per_shader_stage
//                 .min(constrained_limits.max_storage_buffers_per_shader_stage),
//             max_storage_textures_per_shader_stage: limits
//                 .max_storage_textures_per_shader_stage
//                 .min(constrained_limits.max_storage_textures_per_shader_stage),
//             max_uniform_buffers_per_shader_stage: limits
//                 .max_uniform_buffers_per_shader_stage
//                 .min(constrained_limits.max_uniform_buffers_per_shader_stage),
//             max_uniform_buffer_binding_size: limits
//                 .max_uniform_buffer_binding_size
//                 .min(constrained_limits.max_uniform_buffer_binding_size),
//             max_storage_buffer_binding_size: limits
//                 .max_storage_buffer_binding_size
//                 .min(constrained_limits.max_storage_buffer_binding_size),
//             max_vertex_buffers: limits
//                 .max_vertex_buffers
//                 .min(constrained_limits.max_vertex_buffers),
//             max_vertex_attributes: limits
//                 .max_vertex_attributes
//                 .min(constrained_limits.max_vertex_attributes),
//             max_vertex_buffer_array_stride: limits
//                 .max_vertex_buffer_array_stride
//                 .min(constrained_limits.max_vertex_buffer_array_stride),
//             max_push_constant_size: limits
//                 .max_push_constant_size
//                 .min(constrained_limits.max_push_constant_size),
//             min_uniform_buffer_offset_alignment: limits
//                 .min_uniform_buffer_offset_alignment
//                 .max(constrained_limits.min_uniform_buffer_offset_alignment),
//             min_storage_buffer_offset_alignment: limits
//                 .min_storage_buffer_offset_alignment
//                 .max(constrained_limits.min_storage_buffer_offset_alignment),
//             max_inter_stage_shader_components: limits
//                 .max_inter_stage_shader_components
//                 .min(constrained_limits.max_inter_stage_shader_components),
//             max_compute_workgroup_storage_size: limits
//                 .max_compute_workgroup_storage_size
//                 .min(constrained_limits.max_compute_workgroup_storage_size),
//             max_compute_invocations_per_workgroup: limits
//                 .max_compute_invocations_per_workgroup
//                 .min(constrained_limits.max_compute_invocations_per_workgroup),
//             max_compute_workgroup_size_x: limits
//                 .max_compute_workgroup_size_x
//                 .min(constrained_limits.max_compute_workgroup_size_x),
//             max_compute_workgroup_size_y: limits
//                 .max_compute_workgroup_size_y
//                 .min(constrained_limits.max_compute_workgroup_size_y),
//             max_compute_workgroup_size_z: limits
//                 .max_compute_workgroup_size_z
//                 .min(constrained_limits.max_compute_workgroup_size_z),
//             max_compute_workgroups_per_dimension: limits
//                 .max_compute_workgroups_per_dimension
//                 .min(constrained_limits.max_compute_workgroups_per_dimension),
//             max_buffer_size: limits
//                 .max_buffer_size
//                 .min(constrained_limits.max_buffer_size),
//             max_bindings_per_bind_group: todo!(),
//         };
//     }

//     let (device, queue) = adapter
//         .request_device(
//             &wgpu::DeviceDescriptor {
//                 label: options.device_label.as_ref().map(|a| a.as_ref()),
//                 features,
//                 limits,
//             },
//             trace_path,
//         )
//         .await
//         .unwrap();

//     let device = Arc::new(device);
//     let queue = Arc::new(queue);
//     let adapter = Arc::new(adapter);

//     let (device, queue, adapter_info, adapter) =
//     (
//         RenderDevice::from(device),
//         RenderQueue(queue),
//         RenderAdapterInfo(adapter_info),
//         RenderAdapter(adapter),
//     );

//     if let Some(ref adapter) = adapter {
//         app.insert_resource(adapter.clone());
//     }
//     app.insert_resource(device.clone())
//                 .insert_resource(queue.clone())
//                 .insert_resource(adapter_info.clone());

// }

pub fn create_magicleap_one_graphics_client(app: &mut App) {
    let render_device = app.world.resource::<RenderDevice>();

    let graphics_options = magicleap_c_api::MLGraphicsOptions {
        graphics_flags: magicleap_c_api::MLGraphicsFlags_MLGraphicsFlags_Default,
        color_format: magicleap_c_api::MLSurfaceFormat_MLSurfaceFormat_RGBA8UNormSRGB,
        depth_format: magicleap_c_api::MLSurfaceFormat_MLSurfaceFormat_D32Float,
    };

    let mut graphics_client_handle = magicleap_c_api::MLHandle::default();

    unsafe {
        let (instance, physical_device, device) = render_device
            .wgpu_device()
            .as_hal::<wgpu_hal::vulkan::Api, _, _>(
                |hal_device: Option<&wgpu_hal::vulkan::Device>| {
                    if let Some(hal_device) = hal_device {
                        (
                            hal_device.shared_instance().raw_instance().handle(),
                            hal_device.raw_physical_device(),
                            hal_device.raw_device().handle(),
                        )
                    } else {
                        panic!("Couldn't get wgpu_hal::vulkan::Device");
                    }
                },
            );

        info!(
            "{:?}",
            (&graphics_options, &instance, &physical_device, &device,)
        );

        info!(
            "GraphicsClientParams before: {:?}",
            (instance.as_raw(), physical_device.as_raw(), device.as_raw())
        );

        info!(
            "GraphicsClientParams after: {:?}",
            (
                mem::transmute::<_, magicleap_c_api::VkInstance>(instance.as_raw()),
                mem::transmute::<_, magicleap_c_api::VkPhysicalDevice>(physical_device.as_raw()),
                mem::transmute::<_, magicleap_c_api::VkDevice>(device.as_raw())
            )
        );

        if let Err(result) = magicleap_c_api::MLGraphicsCreateClientVk(
            &graphics_options,
            // instance.as_raw() as VkInstance,
            // physical_device.as_raw() as VkPhysicalDevice,
            // device.as_raw() as VkDevice,
            mem::transmute(instance.as_raw()),
            mem::transmute(physical_device.as_raw()),
            mem::transmute(device.as_raw()),
            &mut graphics_client_handle,
        )
        .ok()
        {
            panic!("Creating Vulkan client failed! {}", String::from(result));
        }
    }

    info!("GraphicsClientHandle: {:?}", graphics_client_handle);

    app.world
        .insert_resource(MagicLeapGraphicsHandle(graphics_client_handle));
}

pub fn setup_magic_leap_one_render_targets(app: &App) {
    let graphics_client = app.world.resource::<MagicLeapGraphicsHandle>();
    let render_device = app.world.resource::<RenderDevice>();

    let mut render_targets = magicleap_c_api::MLGraphicsRenderTargetsInfo::default();

    unsafe { magicleap_c_api::MLGraphicsGetRenderTargets(**graphics_client, &mut render_targets) };

    info!("Render target: {:?}#######################", render_targets);

    // for (i, buffer) in render_targets.buffers.into_iter().enumerate() {
    //     //Get supplied Texture.
    //     let image = ash::vk::Image::from_raw(buffer.color.id);

    //     unsafe {
    //         render_device
    //             .wgpu_device()
    //             .as_hal::<wgpu_hal::vulkan::Api, _, _>(
    //                 |hal_device: Option<&wgpu_hal::vulkan::Device>| {
    //                     if let Some(hal_device) = hal_device {
    //                         transition_image_layout(
    //                             hal_device.raw_device(),
    //                             hal_device.raw_queue(),
    //                             image,
    //                         );
    //                     } else {
    //                         panic!("Couldn't get wgpu_hal::vulkan::Device");
    //                     }
    //                 },
    //             );
    //     }
    // }
}

fn transition_image_layout(device: &ash::Device, queue: ash::vk::Queue, image: vk::Image) {
    let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(vk::REMAINING_MIP_LEVELS)
        .base_array_layer(0)
        .layer_count(vk::REMAINING_ARRAY_LAYERS)
        .build();

    let memory_barrier = vk::ImageMemoryBarrier::builder()
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .image(image)
        .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_READ)
        .subresource_range(subresource_range)
        .build();

    let command_pool = unsafe {
        device
            .create_command_pool(
                &vk::CommandPoolCreateInfo::builder()
                    .queue_family_index(0) //queue_family_index)
                    .flags(
                        vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
                            | vk::CommandPoolCreateFlags::TRANSIENT,
                    ),
                None,
            )
            .expect("Unable to create command pool")
    };

    let alloc_info = vk::CommandBufferAllocateInfo::builder()
        .command_buffer_count(1)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_pool(command_pool);

    let command_buffer = unsafe {
        device
            .allocate_command_buffers(&alloc_info)
            .map(|mut b| b.pop().unwrap())
            .expect("Unable to allocate command buffer")
    };

    let begin_info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe {
        device
            .begin_command_buffer(command_buffer, &begin_info)
            .expect("Unable to begin command buffer")
    };

    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::TOP_OF_PIPE,
            PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[memory_barrier],
        )
    };

    unsafe {
        device
            .end_command_buffer(command_buffer)
            .expect("Unable to end command buffer");
    }

    info!("Image should have been transitioned!! ############################################################");

    // let command_buffers = &[command_buffer];

    // let submit_info = vk::SubmitInfo::builder()
    //     .command_buffers(command_buffers)
    //     .build();

    // let submit_info = &[submit_info];

    // unsafe {
    //     device
    //         .queue_submit(queue, submit_info, vk::Fence::null())
    //         .expect("Unable to submit to queue");
    //     device.queue_wait_idle(queue).expect("Unable to wait idle");
    //     device.free_command_buffers(command_pool, command_buffers)
    // }
}

pub fn initialize_magicleap_one_graphics_frame_render_systems(app: &mut App) {
    let graphics_client = app.world.resource::<MagicLeapGraphicsHandle>().clone();
    app.sub_app_mut(bevy::render::RenderApp)
        .insert_resource(graphics_client);

    app.sub_app_mut(bevy::render::RenderApp).add_systems(
        bevy::render::Render,
        (begin_magicleap_one_frame) //.in_set(bevy::render::RenderSet::Prepare),
            .after(bevy::render::RenderSet::ExtractCommands)
            .before(bevy::render::RenderSet::Prepare),
    );

    app.sub_app_mut(bevy::render::RenderApp).add_systems(
        bevy::render::Render,
        (end_magicleap_one_frame).after(bevy::render::RenderSet::CleanupFlush), //.before(bevy::render::RenderSet::Cleanup),
    );
}

fn print_cameras(cameras: Query<&ExtractedCamera>) {
    info!("Extracted Cameras: #######################################################################################################");
    for camera in cameras.iter() {
        info!("Extracted Camera: {:?}", camera);
    }
}

#[derive(Component)]
enum MagicLeapOneCamera {
    Combined,
    Left,
    Right,
}

impl MagicLeapOneCamera {
    fn index(&self) -> i32 {
        match self {
            MagicLeapOneCamera::Combined => -1,
            MagicLeapOneCamera::Left => 0,
            MagicLeapOneCamera::Right => 1,
        }
    }
}

#[derive(Debug)]
struct NotACameraError;

impl TryFrom<i32> for MagicLeapOneCamera {
    type Error = NotACameraError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(MagicLeapOneCamera::Combined),
            0 => Ok(MagicLeapOneCamera::Left),
            1 => Ok(MagicLeapOneCamera::Right),
            _ => Err(NotACameraError),
        }
    }
}

#[derive(Default, Resource)]
struct MagicLeapOneSwapChain {
    textures: HashMap<u64, Arc<wgpu::Texture>>,
    manual_texture_views: HashMap<u64, Vec<ManualTextureView>>,
}

//This needs to be reworked when rendering works to attach to a XrSceleton.
pub fn setup_magic_leap_one_cameras(
    mut commands: Commands,
    graphics_client: Res<MagicLeapGraphicsHandle>,
    render_device: Res<RenderDevice>,
    mut manual_texture_views: ResMut<ManualTextureViews>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut render_targets = magicleap_c_api::MLGraphicsRenderTargetsInfo::default();

    unsafe { magicleap_c_api::MLGraphicsGetRenderTargets(**graphics_client, &mut render_targets) };

    info!("Render target: {:?}#######################", render_targets);

    let mut swapchain = MagicLeapOneSwapChain::default();

    for (i, buffer) in render_targets.buffers.into_iter().enumerate() {
        //Get supplied Texture.
        let image = ash::vk::Image::from_raw(buffer.color.id);

        let size = wgpu::Extent3d {
            width: buffer.color.width,
            height: buffer.color.height,
            depth_or_array_layers: render_targets.num_virtual_cameras,
        };

        let hal_texture = unsafe {
            <wgpu_hal::api::Vulkan as wgpu_hal::Api>::Device::texture_from_raw(
                image,
                &wgpu_hal::TextureDescriptor {
                    label: Some(format!("ml_color_texture buffer {}", i).as_str()),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::bevy_default(),
                    usage: TextureUses::COLOR_TARGET | wgpu_hal::TextureUses::COPY_DST,
                    memory_flags: wgpu_hal::MemoryFlags::empty(),
                    view_formats: Vec::new(),
                },
                Some(Box::new(())),
            )
        };

        let texture = unsafe {
            render_device
                .wgpu_device()
                .create_texture_from_hal::<wgpu_hal::api::Vulkan>(
                    hal_texture,
                    &wgpu::TextureDescriptor {
                        label: Some("ml_color_texture"),
                        size,
                        sample_count: 1,
                        mip_level_count: 1,
                        format: wgpu::TextureFormat::bevy_default(),
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                            | wgpu::TextureUsages::COPY_DST,
                        dimension: wgpu::TextureDimension::D2,
                        view_formats: &[],
                    },
                )
        };

        let texture_views = Vec::<ManualTextureViewHandle>::new();

        for j in 0..render_targets.num_virtual_cameras {
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some(format!("ml_color_texture: {}", j).as_str()),
                format: Some(wgpu::TextureFormat::bevy_default()),
                dimension: Some(wgpu::TextureViewDimension::D2),
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: j,
                array_layer_count: Some(1),
            });

            let manual_texture_view = ManualTextureView {
                texture_view: TextureView::from(texture_view),
                size: UVec2 {
                    x: size.width,
                    y: size.height,
                },
                format: wgpu::TextureFormat::bevy_default(),
            };

            manual_texture_views.insert(ManualTextureViewHandle(j), manual_texture_view.clone());

            manual_texture_views.insert(
                ManualTextureViewHandle(buffer.color.id as u32 + j),
                manual_texture_view,
            );
        }

        swapchain
            .textures
            .insert(buffer.color.id, Arc::new(texture));
    }

    commands.insert_resource(swapchain);

    for i in 0..render_targets.num_virtual_cameras {
        let handle = images.add(Image::default());

        commands
            .spawn(Camera3dBundle {
                camera: Camera {
                    target: bevy::render::camera::RenderTarget::TextureView(
                        ManualTextureViewHandle(i),
                    ),
                    viewport: Some(Viewport {
                        physical_position: UVec2::ZERO,
                        physical_size: UVec2::new(1280, 960),
                        ..default()
                    }),
                    ..default()
                },
                transform: Transform::from_xyz(-4.0, 2.0, -4.0)
                    .looking_at(Vec3::X + Vec3::Y, Vec3::Y),
                ..default()
            })
            .insert(MagicLeapOneCamera::try_from(i as i32 - 1).expect(
                "This shouldn't happen! Are there more virtual cameras than the expected 3?",
            ));
    }
}

#[derive(Resource, Clone)]
struct MagicLeapOneFrameHandles {
    pub frame_handle: u64,
    pub semaphore_handles: Vec<u64>,
    //texture: Arc<wgpu::Texture>,
}

fn begin_magicleap_one_frame(
    mut commands: Commands,
    mut texture_views: ResMut<ManualTextureViews>,
    graphics_client: Res<MagicLeapGraphicsHandle>,
    render_device: Res<RenderDevice>,
    mut cameras: Query<(Entity, &mut ExtractedCamera)>, //&ExtractedView //maybe adjust transform and projection here later too
) {
    let mut frame_params = magicleap_c_api::MLGraphicsFrameParamsEx::default();
    let mut frame_info = magicleap_c_api::MLGraphicsFrameInfo::default();

    unsafe {
        magicleap_c_api::MLGraphicsFrameInfoInitWrapped(&mut frame_info);
        magicleap_c_api::MLGraphicsFrameParamsExInitWrapped(&mut frame_params);

        info!("Beginning ML Frame!");
        info!("FrameParams: {:?}", frame_params);

        let start = Instant::now();

        match magicleap_c_api::MLGraphicsBeginFrameEx(
            **graphics_client,
            &mut frame_params,
            &mut frame_info,
        )
        .ok()
        {
            Ok(()) => {
                info!("Got ML Frame!");

                let duration = Instant::now().duration_since(start).as_millis();

                info!("Begin Frame time delay: {:?} ########################################################################################################################################################################################", duration);

                info!(
                    "Frame handle: {:?} #######################",
                    frame_info.handle
                );

                info!(
                    "Frame color_id: {:?} #######################",
                    frame_info.color_id,
                );

                let mut present_handles = Vec::new();
                let mut sync_handles = Vec::new();

                //Create texture views and cameras.
                for n in 0..frame_info.num_virtual_cameras {
                    let camera = frame_info.virtual_cameras[n as usize];
                    let camera_name = camera.virtual_camera_name;

                    info!("Camera name: {:?} #############", camera_name);
                    let handle = ManualTextureViewHandle(frame_info.color_id as u32 + n); //(camera_name + 1) as u32);
                    present_handles.push(handle.clone());

                    sync_handles.push(camera.sync_object);
                }

                info!("{:?}", texture_views.keys());
                info!("{:?}", texture_views.values());
                info!("{:?}", cameras.is_empty());

                //It might be smarter to do propper handle management later.
                for (entity, mut camera) in cameras.iter_mut() {
                    info!("{:?}", camera);
                    if let Some(NormalizedRenderTarget::TextureView(handle)) = camera.target {
                        // if !present_handles.contains(&handle) {
                        //     commands.entity(entity).despawn();
                        // }
                        let n = frame_info.color_id as u32 + handle.0;
                        info!("ManualTextureViewHandle: {:?}  ##############################################################################", n);
                        let new_handle = Some(NormalizedRenderTarget::TextureView(
                            ManualTextureViewHandle(frame_info.color_id as u32 + handle.0),
                        ));
                        camera.target = new_handle;
                    }
                }

                sync_handles.push(frame_info.wait_sync_object);

                commands.insert_resource(MagicLeapOneFrameHandles {
                    frame_handle: frame_info.handle,
                    semaphore_handles: sync_handles,
                    //texture: Arc::new(texture),
                });
            }
            Err(result) => {
                info!(
                    "An error occured while beginning frame! {:?}",
                    String::from(result)
                );

                //It might be smarter to do propper handle management later.
                // for (entity, camera) in cameras.iter() {
                //     if let Some(NormalizedRenderTarget::TextureView(handle)) = camera.target {
                //         commands.entity(entity).despawn();
                //     }
                // }
            }
        }
    }
}

fn end_magicleap_one_frame(
    graphics_client: Res<MagicLeapGraphicsHandle>,
    frame_handles: Option<Res<MagicLeapOneFrameHandles>>,
    render_device: Res<RenderDevice>,
) {
    if let Some(frame_handles) = frame_handles {
        info!("Ending ML Frame!");
        unsafe {
            render_device
                .wgpu_device()
                .as_hal::<wgpu_hal::vulkan::Api, _, _>(
                    |wgpu_device: Option<&wgpu_hal::vulkan::Device>| {
                        if let Some(device) = wgpu_device {
                            let vk_device = device.raw_device();

                            for handle in frame_handles.clone().semaphore_handles {
                                let semaphore = ash::vk::Semaphore::from_raw(handle);
                                info!("{:?}", semaphore);

                                // let allocate_info = ash::vk::CommandBufferAllocateInfo::builder()
                                //     .command_buffer_count(1)
                                //     .command_pool() //Don't know how to get this.
                                //     .level(ash::vk::CommandBufferLevel::PRIMARY)
                                //     .build();

                                let submit_info = ash::vk::SubmitInfo::builder()
                                    .signal_semaphores(&[semaphore])
                                    .command_buffers(&[])
                                    .wait_dst_stage_mask(&[])
                                    .wait_semaphores(&[])
                                    .build();

                                let fence_create_info = ash::vk::FenceCreateInfo {
                                    s_type: ash::vk::StructureType::FENCE_CREATE_INFO,
                                    p_next: std::ptr::null(),
                                    flags: ash::vk::FenceCreateFlags::empty(),
                                };

                                let fence = vk_device
                                    .create_fence(&fence_create_info, None)
                                    .expect("Failed to create fence!");

                                vk_device
                                    .queue_submit(device.raw_queue(), &[submit_info], fence)
                                    .expect("Failed to submit queue!");

                                // Wait for the fence
                                //vk_device
                                //    .wait_for_fences(&[fence], true, std::u64::MAX)
                                //    .expect("Failed to wait for fence!");

                                // Remove the fence
                                vk_device.destroy_fence(fence, None);

                                info!("Semaphore should have been signaled! {:?}", semaphore);
                            }

                            if let Err(result) = magicleap_c_api::MLGraphicsEndFrame(
                                **graphics_client,
                                frame_handles.frame_handle,
                            )
                            .ok()
                            {
                                error!("Failed to end frame! {:?}", String::from(result));
                            }
                            info!("Ended ML frame!");
                        }
                    },
                );
        }
    }
}
