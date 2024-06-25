use cgmath::{Point3, Vector3};

use super::{camera_uniform::CameraUniform, Camera};

pub struct GameCamera {
    pub camera: Camera,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
}
