use bespoke_engine::{binding::UniformBinding, compute::ComputeShader};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, Buffer, Device, Queue};

use crate::banana_instance::BananaInstanceRaw;

pub struct BananaInstances {
    pub num_bananas: [usize; 3],
    pub buffers: u32,
    pub buffer_layout: BindGroupLayout,
    pub buffer_group_0: Vec<Buffer>,
    pub buffer_group_1: Vec<Buffer>,
    pub buffer_binding_0: Vec<BindGroup>,
    pub buffer_binding_1: Vec<BindGroup>,
    pub buffer: u32,
    shader: ComputeShader,
    pub workgroup_size_binding: UniformBinding<[u32; 3]>,
}

impl BananaInstances {
    pub fn new(num_bananas: [usize; 3], buffers: [u32; 3], shader_source: &str, setup_shader_source: &str, time_layout: &BindGroupLayout, screen_info_layout: &BindGroupLayout, device: &Device, queue: &Queue) -> Self {
        let buffer_layout = 
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            // We will change the values in this buffer
                            read_only: false,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }]
            });
        let workgroup_size_binding = UniformBinding::new(device, "Workgroup Size", [num_bananas[0] as u32, num_bananas[1] as u32, num_bananas[2] as u32], None);
        let mut offset_binding = UniformBinding::new(device, "Offset", [0u32,0,0], None);
        let mut buffer_group_0 = vec![];
        let mut buffer_group_1 = vec![];
        let mut buffer_binding_0 = vec![];
        let mut buffer_binding_1 = vec![];
        for x in 0..buffers[0] {
            for y in 0..buffers[1] {
                for z in 0..buffers[2] {
                    offset_binding.set_data(device, [x*num_bananas[0] as u32, y*num_bananas[1] as u32, z*num_bananas[2] as u32]);
                    let buffer_0 = device.create_buffer(&wgpu::BufferDescriptor { 
                        label: Some("Buffer 0"), 
                        size: (num_bananas[0] * num_bananas[1] * num_bananas[2] * size_of::<BananaInstanceRaw>()) as u64, 
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, 
                        mapped_at_creation: false,
                    });
                    let buffer_1 = device.create_buffer(&wgpu::BufferDescriptor { 
                        label: Some("Buffer 1"), 
                        size: (num_bananas[0] * num_bananas[1] * num_bananas[2] * size_of::<BananaInstanceRaw>()) as u64, 
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, 
                        mapped_at_creation: false,
                    });
                    // Self::setup_instances(&workgroup_size_binding, &buffer_1, &buffer_layout, &offset_binding, setup_shader_source, device, queue);
                    Self::setup_instances(&workgroup_size_binding, &buffer_0, &buffer_layout, &offset_binding, setup_shader_source, device, queue);
                    buffer_binding_0.push(Self::_buffer_bind_group(&buffer_layout, device, &buffer_0));
                    buffer_binding_1.push(Self::_buffer_bind_group(&buffer_layout, device, &buffer_1));
                    buffer_group_0.push(buffer_0);
                    buffer_group_1.push(buffer_1);
                }
                println!("y {y} / {}", buffers[1]);
            }
            println!("x {x} / {}", buffers[0]);
        }

        
        let compute_shader = ComputeShader::new(shader_source, &[&buffer_layout, &buffer_layout, time_layout, screen_info_layout, &workgroup_size_binding.layout], device);
        Self {
            buffer_layout,
            shader: compute_shader,
            num_bananas,
            buffers: buffers[0]*buffers[1]*buffers[2],
            buffer: 0,
            buffer_group_0,
            buffer_group_1,
            buffer_binding_0,
            buffer_binding_1,
            workgroup_size_binding,
        }
    }
    
    pub fn create_bananas(&mut self, time_bind_group: &BindGroup, screen_info_binding: &BindGroup, device: &Device, queue: &Queue) {
        self.buffer = (self.buffer + 1) % 2;
        let (src_buffers, dst_buffers) = if self.buffer == 0 {
            (&self.buffer_group_0, &self.buffer_group_1)
        } else {
            (&self.buffer_group_1, &self.buffer_group_0)
        };

        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.shader.pipeline);
            cpass.set_bind_group(2, Some(time_bind_group), &[]);
            cpass.set_bind_group(3, Some(&self.workgroup_size_binding.binding), &[]);
            cpass.set_bind_group(4, Some(screen_info_binding), &[]);
            for i in 0..self.buffers {
                let src_bind_group = self.buffer_bind_group(device, &src_buffers[i as usize]);
                let dst_bind_group = self.buffer_bind_group(device, &dst_buffers[i as usize]);
                cpass.set_bind_group(0, Some(&src_bind_group), &[]);
                cpass.set_bind_group(1, Some(&dst_bind_group), &[]);
                cpass.dispatch_workgroups(self.num_bananas[0] as u32, self.num_bananas[1] as u32, self.num_bananas[2] as u32);
            }
        }
        queue.submit(Some(encoder.finish()));
        // for i in 0..self.buffers {
        //     let src_bind_group = self.buffer_bind_group(device, &src_buffers[i as usize]);
        //     let dst_bind_group = self.buffer_bind_group(device, &dst_buffers[i as usize]);
        //     self.shader.run_once(vec![&src_bind_group, &dst_bind_group, time_bind_group, &self.workgroup_size_binding.binding, screen_info_binding], [self.num_bananas[0] as u32, self.num_bananas[1] as u32, self.num_bananas[2] as u32], device, queue);
        // }
    }

    pub fn buffer_bind_group(&self, device: &Device, buffer: &Buffer) -> BindGroup {
        Self::_buffer_bind_group(&self.buffer_layout, device, buffer)
    }

    pub fn output_buffer_bindings(&self) -> &Vec<BindGroup> {
        if self.buffer == 0 {
            &self.buffer_binding_0
        } else {
            &self.buffer_binding_1
        }
    }

    pub fn output_buffers(&self) -> &Vec<Buffer> {
        if self.buffer == 0 {
            &self.buffer_group_0
        } else {
            &self.buffer_group_1
        }
    }

    fn setup_instances(workgroup_size_binding: &UniformBinding<[u32; 3]>, buffer: &Buffer, buffer_layout: &BindGroupLayout, offset_binding: &UniformBinding<[u32; 3]>, setup_shader_source: &str, device: &Device, queue: &Queue) {
        let compute_shader = ComputeShader::new(setup_shader_source, &[buffer_layout, &workgroup_size_binding.layout, &offset_binding.layout], device);
        compute_shader.run_once(vec![&Self::_buffer_bind_group(buffer_layout, device, buffer), &workgroup_size_binding.binding, &offset_binding.binding], workgroup_size_binding.value, device, queue);
    }

    fn _buffer_bind_group(buffer_layout: &BindGroupLayout, device: &Device, buffer: &Buffer) -> BindGroup {
        let dst_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: buffer_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }]
        });
        return dst_bind_group;
    }
}