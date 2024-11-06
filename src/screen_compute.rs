use bespoke_engine::{binding::UniformBinding, compute::ComputeShader, texture::StorageTexture};
use wgpu::{BindGroupLayout, Device, Queue};

use crate::instance_compute::BananaInstances;

pub struct ScreenCompute {
    shader: ComputeShader,
    pub workgroup_size_binding: UniformBinding<[u32; 3]>,
}

impl ScreenCompute {
    pub fn new(instance_compute: &BananaInstances, texture_layout: &BindGroupLayout, camera_layout: &BindGroupLayout, screen_info_layout: &BindGroupLayout, shader_source: &str, device: &Device) -> Self {
        let workgroup_size_binding = UniformBinding::new(device, "Workgroup Size", [instance_compute.num_bananas[0] as u32, instance_compute.num_bananas[1] as u32, instance_compute.num_bananas[2] as u32], None);
        let compute_shader = ComputeShader::new(shader_source, &[&instance_compute.buffer_layout, texture_layout, &workgroup_size_binding.layout, camera_layout, &screen_info_layout], device);
        Self {
            shader: compute_shader,
            workgroup_size_binding,
        }
    }
    
    pub fn render_instances(&mut self, instance_compute: &BananaInstances, screen_texture: &UniformBinding<StorageTexture>, camera: &UniformBinding<[[f32; 4]; 4]>, screen_info: &UniformBinding<[f32; 4]>, device: &Device, queue: &Queue) {
        let buffer_bindings = instance_compute.output_buffer_bindings();
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.shader.pipeline);
            cpass.set_bind_group(1, Some(&screen_texture.binding), &[]);
            cpass.set_bind_group(2, Some(&self.workgroup_size_binding.binding), &[]);
            cpass.set_bind_group(3, Some(&camera.binding), &[]);
            cpass.set_bind_group(4, Some(&screen_info.binding), &[]);
            for i in 0..buffer_bindings.len() {
                cpass.set_bind_group(0, Some(&buffer_bindings[i]), &[]);
                cpass.dispatch_workgroups(instance_compute.num_bananas[0] as u32, instance_compute.num_bananas[1] as u32, instance_compute.num_bananas[2] as u32);
            }
        }
        queue.submit(Some(encoder.finish()));
        // for buffer in buffer_bindings {
        //     self.shader.run_once(vec![buffer, &screen_texture.binding, &self.workgroup_size_binding.binding, &camera.binding, &screen_info.binding], [instance_compute.num_bananas[0] as u32, instance_compute.num_bananas[1] as u32, instance_compute.num_bananas[2] as u32], device, queue);
        // }
    }
}