use bespoke_engine::{binding::{Binding, Uniform, UniformBinding}, compute::ComputeShader, texture::StorageTexture};
use wgpu::{BindGroupLayout, Device, Queue};

use crate::instance_compute::BananaInstances;

pub struct ScreenCompute {
    shader: ComputeShader,
    pub workgroup_size_binding: UniformBinding<[u32; 3]>,
}

impl ScreenCompute {
    pub fn new(instance_compute: &BananaInstances, texture_layout: &BindGroupLayout, camera_uniform: &dyn Uniform, screen_info_uniform: &dyn Uniform, shader_source: &str, device: &Device) -> Self {
        let workgroup_size_binding = UniformBinding::new(device, "Workgroup Size", instance_compute.num_bananas, None);
        let compute_shader = ComputeShader::new(
            shader_source, 
            &[&instance_compute.buffer_layout, texture_layout, &workgroup_size_binding.layout, camera_uniform.layout(), &screen_info_uniform.layout()], 
            vec![&instance_compute.buffer_type, &StorageTexture::shader_type(), &workgroup_size_binding.shader_type, camera_uniform.shader_type(), screen_info_uniform.shader_type()],
            device);
        Self {
            shader: compute_shader,
            workgroup_size_binding,
        }
    }
    
    pub fn render_instances(&mut self, instance_compute: &BananaInstances, screen_texture: &dyn Uniform, camera: &dyn Uniform, screen_info: &dyn Uniform, device: &Device, queue: &Queue) {
        let buffer_bindings = &instance_compute.buffer_bindings;
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.shader.pipeline);
            cpass.set_bind_group(1, Some(screen_texture.binding()), &[]);
            cpass.set_bind_group(2, Some(&self.workgroup_size_binding.binding), &[]);
            cpass.set_bind_group(3, Some(camera.binding()), &[]);
            cpass.set_bind_group(4, Some(screen_info.binding()), &[]);
            for i in 0..buffer_bindings.len() {
                cpass.set_bind_group(0, Some(&buffer_bindings[i]), &[]);
                cpass.dispatch_workgroups(instance_compute.num_bananas[0], instance_compute.num_bananas[1], instance_compute.num_bananas[2]);
            }
        }
        queue.submit(Some(encoder.finish()));
        // for buffer in buffer_bindings {
        //     self.shader.run_once(vec![buffer, &screen_texture.binding, &self.workgroup_size_binding.binding, &camera.binding, &screen_info.binding], [instance_compute.num_bananas[0] as u32, instance_compute.num_bananas[1] as u32, instance_compute.num_bananas[2] as u32], device, queue);
        // }
    }
}