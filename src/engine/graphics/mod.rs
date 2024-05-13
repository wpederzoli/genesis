use std::{borrow::Cow, path::Path};

use wgpu::{
    util::DeviceExt, Adapter, Device, DeviceDescriptor, Queue, Surface, SurfaceConfiguration,
};

use self::vertex_buffers::Vertex;

mod helpers;
mod pipeline;
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
    pub window: &'a winit::window::Window,
}

impl<'a> Graphics<'a> {
    pub fn new(window: &'a winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

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

        Graphics {
            device,
            queue,
            adapter,
            surface,
            config,
            clear_color: DEFAULT_CLEAR_COLOR,
            pipelines: Vec::new(),
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

            for (index, pipeline) in self.pipelines.iter().enumerate() {
                let vb = pipeline.vertex_buffer.as_ref();
                render_pass.set_pipeline(&pipeline.render_pipeline);
                render_pass.set_vertex_buffer(index as u32, vb.unwrap().slice(..));
                render_pass.draw(0..pipeline.vertex_count, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }

    #[track_caller]
    pub fn load_shader(&mut self, file_path: &str, vertices: Option<&[Vertex]>) {
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

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let contents = if let Some(vertices) = vertices {
            vertices
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

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
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

        let index_size = contents.len();

        self.pipelines.push(pipeline::Pipeline::new(
            render_pipeline,
            Some(vertex_buffer),
            index_size as u32,
        ));
    }
}
