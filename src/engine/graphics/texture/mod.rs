use std::{fs, path::Path};

use super::Graphics;

pub struct Texture {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Texture {
    pub fn load(file_path: &str, graphics: &Graphics) -> Self {
        let img_bytes = fs::read(file_path).unwrap();
        let loaded_img = image::load_from_memory(&img_bytes).unwrap();
        let img_rgba = loaded_img.to_rgba8();

        use image::GenericImageView;
        let img_dimensions = loaded_img.dimensions();

        let texture_size = wgpu::Extent3d {
            width: img_dimensions.0,
            height: img_dimensions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = graphics.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Diffuse texture"),
            view_formats: &[],
        });

        graphics.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * img_dimensions.0),
                rows_per_image: Some(img_dimensions.1),
            },
            texture_size,
        );

        let texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = graphics.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group_layout =
            graphics
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture binding group layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let bind_group = graphics
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Texture bind group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture_sampler),
                    },
                ],
            });

        Texture {
            bind_group,
            bind_group_layout,
        }
    }
}
