use std::{borrow::Cow, path::Path};

use wgpu::{
    util::DeviceExt, Adapter, Device, DeviceDescriptor, Queue, Surface, SurfaceConfiguration,
};

use self::{texture::Texture, vertex_buffers::Vertex};

use super::camera::{camera_uniform::CameraUniform, Camera};

mod helpers;
mod pipeline;
pub mod texture;
pub mod vertex_buffers;

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.3,
    g: 0.5,
    b: 0.4,
    a: 1.0,
};

pub struct Graphics<'a> {
    pub device: Device,
    pub queue: Queue,
    pub adapter: Adapter,
    pub surface: Surface<'a>,
    pub config: SurfaceConfiguration,
    pub clear_color: wgpu::Color,
    pipelines: Vec<pipeline::Pipeline>,
    textures: Vec<Texture>,
    pub camera: Camera,
    camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    pub window: &'a winit::window::Window,
}

impl<'a> Graphics<'a> {
    pub fn new(window: &'a winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        //window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        let size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
            },
            None,
        ))
        .unwrap();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Camera_binding_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_binding_group"),
        });

        Graphics {
            device,
            queue,
            adapter,
            surface,
            config,
            clear_color: DEFAULT_CLEAR_COLOR,
            pipelines: Vec::new(),
            textures: Vec::new(),
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
            window,
        }
    }

    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.clear_color = color;
    }

    pub fn render(&mut self) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Unable to get current texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for pipeline in &self.pipelines {
                let vb = pipeline.vertex_buffer.as_ref();
                let ib = pipeline.index_buffer.as_ref();
                render_pass.set_pipeline(&pipeline.render_pipeline);

                if let Some(texture_index) = pipeline.texture_index {
                    let texture = &self.textures[texture_index];
                    render_pass.set_bind_group(0, &texture.bind_group, &[]);
                    render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                } else {
                    render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                }

                render_pass.set_vertex_buffer(0, vb.unwrap().slice(..));
                render_pass.set_index_buffer(ib.unwrap().slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..pipeline.index_count, 0, 0..1)
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }

    #[track_caller]
    pub fn load_texture(&mut self, file_path: &str) -> usize {
        let current_dir = std::env::current_dir().unwrap();
        let caller_location = std::panic::Location::caller().file();
        let parent = Path::new(caller_location).parent().unwrap();
        let absolute_path = current_dir.join(parent).join(file_path);

        let texture = Texture::load(absolute_path.to_str().unwrap(), self);
        self.textures.push(texture);
        self.textures.len() - 1
    }

    #[track_caller]
    pub fn load_shader(
        &mut self,
        file_path: &str,
        vertices: Option<&[Vertex]>,
        indices: Option<&[u16]>,
        texture_index: Option<usize>,
    ) {
        let current_dir = std::env::current_dir().unwrap();
        let caller_location = std::panic::Location::caller().file();
        let parent = Path::new(caller_location).parent().unwrap();
        let absolute_path = current_dir.join(parent).join(file_path);

        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&helpers::file_contents(
                    absolute_path.to_str().unwrap(),
                ))),
            });

        let bind_group_layouts_storage;

        let bind_group_layouts: &[&wgpu::BindGroupLayout] = if let Some(tex_index) = texture_index {
            let texture = &self.textures[tex_index];
            bind_group_layouts_storage =
                vec![&texture.bind_group_layout, &self.camera_bind_group_layout];
            &bind_group_layouts_storage
        } else {
            bind_group_layouts_storage = vec![&self.camera_bind_group_layout];
            &bind_group_layouts_storage
        };

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let contents = if let Some(vertices) = vertices {
            vertices
        } else {
            &[]
        };

        let index_contents = if let Some(indices) = indices {
            indices
        } else {
            &[]
        };

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(contents),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(index_contents),
                usage: wgpu::BufferUsages::INDEX,
            });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        let vertex_index_size = contents.len();
        let index_count = index_contents.len();

        self.pipelines.push(pipeline::Pipeline::new(
            render_pipeline,
            Some(vertex_buffer),
            vertex_index_size as u32,
            Some(index_buffer),
            index_count as u32,
            texture_index,
        ));
    }
}
