use bespoke_engine::{binding::Descriptor, model::ToRaw};
use bytemuck::bytes_of;
use cgmath::{Vector3, Zero};

#[derive(Clone)]
pub struct BananaInstance {
    pub position: cgmath::Vector3<f32>,
    pub velocity: cgmath::Vector3<f32>,
}

impl BananaInstance {
    pub fn raw(&self) -> BananaInstanceRaw {
        BananaInstanceRaw { position: self.position.extend(1.0).into(), velocity: self.velocity.extend(1.0).into() }
    }
}

impl Default for BananaInstance {
    fn default() -> Self {
        Self { position: Vector3::zero(), velocity: Vector3::zero() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct BananaInstanceRaw {
    position: [f32; 4],
    velocity: [f32; 4],
}

impl ToRaw for BananaInstance {
    fn to_raw(&self) -> Vec<u8> {
        let raw = self.raw();
        bytes_of(&raw).to_vec()
    }
}

impl Descriptor for BananaInstance {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<BananaInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as u64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}