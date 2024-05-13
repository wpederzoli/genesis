pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub vertex_count: u32,
}

impl Pipeline {
    pub fn new(
        render_pipeline: wgpu::RenderPipeline,
        vertex_buffer: Option<wgpu::Buffer>,
        vertex_count: u32,
    ) -> Self {
        Pipeline {
            render_pipeline,
            vertex_buffer,
            vertex_count,
        }
    }
}
