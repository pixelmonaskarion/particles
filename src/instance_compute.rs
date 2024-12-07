use bespoke_engine::{binding::{create_layout, Binding, Uniform, UniformBinding}, compute::ComputeShader, shader::ShaderType, texture::{StorageTexture3D, Texture}};
use wgpu::{BindGroup, BindGroupLayout, Device, Queue, TextureFormat};

pub struct BananaInstances {
    pub num_bananas: [u32; 3],
    pub buffers: u32,
    pub buffer_layout: BindGroupLayout,
    pub buffer_type: ShaderType,
    pub buffer_bindings: Vec<BindGroup>,
    shader: ComputeShader,
    pub workgroup_size_binding: UniformBinding<[u32; 3]>,
}

impl BananaInstances {
    pub fn new(num_bananas: [u32; 3], buffers: [u32; 3], shader_source: &str, setup_shader_source: &str, time_uniform: &dyn Uniform, screen_info_uniform: &dyn Uniform, device: &Device, queue: &Queue) -> Self {
        let buffer_layout = create_layout::<StorageTexture3D>(device);
        let buffer_type = StorageTexture3D::shader_type();
        let workgroup_size_binding = UniformBinding::new(device, "Workgroup Size", num_bananas, None);
        let mut offset_binding = UniformBinding::new(device, "Offset", [0u32,0,0], None);
        let mut buffer_bindings = vec![];
        for x in 0..buffers[0] {
            for y in 0..buffers[1] {
                for z in 0..buffers[2] {
                    offset_binding.set_data(device, [x*num_bananas[0], y*num_bananas[1], z*num_bananas[2]]);
                    let buffer = StorageTexture3D::from_texture(Texture::blank_texture_3d(device, num_bananas[0]*3, num_bananas[1], num_bananas[2], TextureFormat::Rgba32Float));
                    let buffer_binding = UniformBinding::create_bind_group(&buffer, "", &buffer_layout, device);
                    Self::setup_instances(&workgroup_size_binding, &buffer_binding, &buffer_layout, &offset_binding, setup_shader_source, device, queue);
                    buffer_bindings.push(buffer_binding);
                }
                println!("y {y} / {}", buffers[1]);
            }
            println!("x {x} / {}", buffers[0]);
        }

        
        let compute_shader = ComputeShader::new(
            shader_source, 
            &[&buffer_layout, time_uniform.layout(), &workgroup_size_binding.layout, screen_info_uniform.layout()], 
            vec![&buffer_type, time_uniform.shader_type(), &workgroup_size_binding.shader_type, screen_info_uniform.shader_type()],
            device,
        );
        Self {
            buffer_layout,
            buffer_type,
            shader: compute_shader,
            num_bananas,
            buffers: buffers[0]*buffers[1]*buffers[2],
            // buffer_group,
            buffer_bindings,
            workgroup_size_binding,
        }
    }
    
    pub fn create_bananas(&mut self, time_bind_group: &BindGroup, screen_info_binding: &BindGroup, device: &Device, queue: &Queue) {
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.shader.pipeline);
            cpass.set_bind_group(1, Some(time_bind_group), &[]);
            cpass.set_bind_group(2, Some(&self.workgroup_size_binding.binding), &[]);
            cpass.set_bind_group(3, Some(screen_info_binding), &[]);
            for i in 0..self.buffers {
                cpass.set_bind_group(0, Some(&self.buffer_bindings[i as usize]), &[]);
                cpass.dispatch_workgroups(self.num_bananas[0], self.num_bananas[1], self.num_bananas[2]);
            }
        }
        queue.submit(Some(encoder.finish()));
    }

    fn setup_instances(workgroup_size_binding: &UniformBinding<[u32; 3]>, texture_binding: &BindGroup, buffer_layout: &BindGroupLayout, offset_binding: &UniformBinding<[u32; 3]>, setup_shader_source: &str, device: &Device, queue: &Queue) {
        let compute_shader = ComputeShader::new(
            setup_shader_source, 
            &[buffer_layout, &workgroup_size_binding.layout, &offset_binding.layout], 
            vec![&StorageTexture3D::shader_type(), &workgroup_size_binding.shader_type, &offset_binding.shader_type],
            device
        );
        compute_shader.run_once(vec![texture_binding, &workgroup_size_binding.binding, &offset_binding.binding], workgroup_size_binding.value, device, queue);
    }
}