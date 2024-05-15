pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub vertex_count: u32,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: u32,
}

impl Pipeline {
    pub fn new(
        render_pipeline: wgpu::RenderPipeline,
        vertex_buffer: Option<wgpu::Buffer>,
        vertex_count: u32,
        index_buffer: Option<wgpu::Buffer>,
        index_count: u32,
    ) -> Self {
        Pipeline {
            render_pipeline,
            vertex_buffer,
            vertex_count,
            index_buffer,
            index_count,
        }
    }
}
