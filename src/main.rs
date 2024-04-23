use std::borrow::Cow;

use engine::{graphics::Graphics, Engine};
use winit::{
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
};
mod engine;

//TODO: clear resources in event loop run (verify)
//TODO: windowed and full-screen feature
//TODO: Draw player to screen and movement
//TODO: Abstarct drawing defaults
//TODO: Abstract game logic to its own context

fn my_input_handler(input_key: &PhysicalKey, target: &EventLoopWindowTarget<()>) {
    match input_key {
        PhysicalKey::Code(key_code) => match key_code {
            KeyCode::Escape => {
                target.exit();
            }
            _ => (),
        },
        _ => (),
    }
}

fn my_draw_function(graphics: &Graphics) {
    let Graphics {
        device,
        surface,
        adapter,
        queue,
        ..
    } = graphics;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_cap = surface.get_capabilities(&adapter);
    let swapchain_formats = swapchain_cap.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            targets: &[Some(swapchain_formats.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    let frame = surface
        .get_current_texture()
        .expect("Failed to get texture");

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(graphics.clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        rpass.set_pipeline(&render_pipeline);
        rpass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
}

fn main() {
    Engine::new("Genesis")
        .with_clear_color(wgpu::Color::BLACK)
        .with_input_system(my_input_handler)
        .with_draw_system(my_draw_function)
        .run();
}
