use bespoke_engine::{binding::{create_layout_from_entries, UniformBinding}, compute::ComputeShader, texture::{StorageTexture3D, Texture}};
use wgpu::{BindGroup, BindGroupLayout, Device, Queue, TextureFormat};

use crate::instance_compute::BananaInstances;

pub struct WorldRenderer {
    shader: ComputeShader,
    pub workgroup_size_binding: UniformBinding<[u32; 3]>,
    pub output_texture_layout: BindGroupLayout,
    pub output_texture: StorageTexture3D,
    pub output_texture_binding: BindGroup,
    pub sampled_texture_layout: BindGroupLayout,
    pub sampled_texture_binding: BindGroup,
}

impl WorldRenderer {
    fn new_texture(device: &Device, size: [u32; 3]) -> StorageTexture3D {
        StorageTexture3D::from_texture(Texture::blank_texture_3d(device, size[0], size[1], size[2], TextureFormat::Rgba8Unorm))
    }

    pub fn new(instance_compute: &BananaInstances, shader_source: &str, device: &Device) -> Self {
        let workgroup_size_binding = UniformBinding::new(device, "Workgroup Size", instance_compute.num_bananas, None);
        let output_texture = Self::new_texture(device, instance_compute.num_bananas);
        let output_texture_layout = create_layout_from_entries(&[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::ReadWrite, format: wgpu::TextureFormat::Rgba8Unorm, view_dimension: wgpu::TextureViewDimension::D3 },
                count: None,
            },
        ], device);
        let output_texture_binding = UniformBinding::create_bind_group(&output_texture, "3d ouput texture", &output_texture_layout, device);
        let sampled_texture_layout = create_layout_from_entries(&[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D3,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ], device);
        let sampled_texture_binding = UniformBinding::create_bind_group(&output_texture.texture, "3d sampled texture", &sampled_texture_layout, device);
        let compute_shader = ComputeShader::new(shader_source, &[&instance_compute.buffer_layout, &output_texture_layout, &workgroup_size_binding.layout], device);
        Self {
            shader: compute_shader,
            workgroup_size_binding,
            output_texture_binding,
            output_texture_layout,
            output_texture,
            sampled_texture_layout,
            sampled_texture_binding,
        }
    }
    
    pub fn render(&mut self, instance_compute: &BananaInstances, device: &Device, queue: &Queue) {
        let buffer_binding = &instance_compute.buffer_bindings[0];
        self.output_texture = Self::new_texture(device, instance_compute.num_bananas);
        self.output_texture_binding = UniformBinding::create_bind_group(&self.output_texture, "3d ouput texture", &self.output_texture_layout, device);
        self.sampled_texture_binding = UniformBinding::create_bind_group(&self.output_texture.texture, "3d sampled texture", &self.sampled_texture_layout, device);
        self.shader.run_once(vec![buffer_binding, &self.output_texture_binding, &self.workgroup_size_binding.binding], instance_compute.num_bananas, device, queue);
    }
}