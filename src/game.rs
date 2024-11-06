use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}, u32};

use bespoke_engine::{binding::{create_layout, Descriptor, UniformBinding}, camera::Camera, model::{Model, Render, ToRaw}, shader::{Shader, ShaderConfig}, surface_context::SurfaceCtx, texture::{DepthTexture, StorageTexture, Texture}, window::{WindowConfig, WindowHandler}};
use bytemuck::{bytes_of, NoUninit};
use cgmath::{Vector2, Vector3};
use wgpu::{Color, Features, Limits, RenderPass};
use wgpu_text::{glyph_brush::{ab_glyph::FontRef, OwnedSection, OwnedText}, BrushBuilder, TextBrush};
use winit::{dpi::PhysicalPosition, event::{KeyEvent, TouchPhase}, keyboard::{KeyCode, PhysicalKey::Code}};
use crate::{banana_instance::BananaInstance, instance_compute::BananaInstances, load_resource, molecule::{generate_molecule_model, H_ion}, screen_compute::ScreenCompute};

pub struct Game {
    camera_binding: UniformBinding<[[f32; 4]; 4]>,
    camera_inverse_binding: UniformBinding<[[f32; 4]; 4]>,
    camera: Camera,
    screen_size: [f32; 2],
    screen_info_binding: UniformBinding<[f32; 4]>,
    delta_time_binding: UniformBinding<f32>,
    start_time: u128,
    keys_down: Vec<KeyCode>,
    touch_positions: HashMap<u64, PhysicalPosition<f64>>,
    ground_shader: Shader,
    sphere_model: Model,
    banana_instances_gen: BananaInstances,
    post_processing_shader: Shader,
    screen_compute: ScreenCompute,
    text_brush: TextBrush<FontRef<'static>>,
    text_section: OwnedSection,
}

#[repr(C)]
#[derive(NoUninit, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_pos: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    #[allow(dead_code)]
    pub fn pos(&self) -> Vector3<f32> {
        return Vector3::new(self.position[0], self.position[1], self.position[2]);
    }
}

impl Descriptor for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

impl ToRaw for Vertex {
    fn to_raw(&self) -> Vec<u8> {
        bytes_of(self).to_vec()
    }
}


impl Game {
    pub fn new(surface_ctx: &dyn SurfaceCtx) -> Self {
        let size = surface_ctx.size();
        let screen_size = [size.0 as f32, size.0 as f32];
        let screen_info_binding = UniformBinding::new(surface_ctx.device(), "Screen Size", [screen_size[0], screen_size[1], 0.0, 0.0], None);
        let camera = Camera {
            // eye: Vector3::new(20.0, 20.0, 20.0),
            eye: Vector3::new(0.0, 0.0, 0.0),
            aspect: screen_size[0] / screen_size[1],
            fovy: 70.0,
            znear: 0.1,
            zfar: 100.0,
            ground: 0.0,
            sky: 0.0,
        };
        let camera_binding = UniformBinding::new(surface_ctx.device(), "Camera", camera.build_view_projection_matrix_raw(), None);
        let camera_inverse_binding = UniformBinding::new(surface_ctx.device(), "Camera Inverse", camera.build_inverse_matrix_raw(), None);
        let delta_time_binding = UniformBinding::new(surface_ctx.device(), "Time", 0.0_f32, None);
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let ground_shader = Shader::new(include_str!("ground.wgsl"), surface_ctx.device(), surface_ctx.config().format, vec![&camera_binding.layout, &delta_time_binding.layout], &[crate::height_map::Vertex::desc(), BananaInstance::desc()], ShaderConfig {line_mode: wgpu::PolygonMode::Fill, ..Default::default() });
        let (sphere_vertices, sphere_indices) = generate_molecule_model(H_ion());
        let sphere_model = Model::new(sphere_vertices, &sphere_indices, surface_ctx.device());
        let banana_instances_gen = BananaInstances::new([1000, 1000, 100], [1, 1, 10], include_str!("banana_instances.wgsl"), include_str!("instances_setup.wgsl"), &delta_time_binding.layout, &screen_info_binding.layout, surface_ctx.device(), surface_ctx.queue());
        let post_processing_shader = Shader::new_post_process(include_str!("post_process.wgsl"), surface_ctx.device(), surface_ctx.config().format, &[&create_layout::<StorageTexture>(&surface_ctx.device()), &create_layout::<DepthTexture>(surface_ctx.device()), &create_layout::<Texture>(&surface_ctx.device()), &screen_info_binding.layout, &camera_binding.layout, &camera_inverse_binding.layout]);
        let screen_compute = ScreenCompute::new(&banana_instances_gen, &create_layout::<StorageTexture>(&surface_ctx.device()), &camera_binding.layout, &screen_info_binding.layout, include_str!("screen_compute.wgsl"), surface_ctx.device());

        let text_brush = BrushBuilder::using_font_bytes(load_resource("res/ComicSansMS.ttf").unwrap()).unwrap()
            .build(surface_ctx.device(), size.0, size.1, surface_ctx.config().format);
        let text_section = OwnedSection::default().add_text(OwnedText::new(format!("0")).with_scale(200.0)
            .with_color([0.0, 0.7490196078, 1.0, 1.0]));
        Self {
            camera_binding,
            camera_inverse_binding,
            camera,
            screen_size,
            screen_info_binding,
            delta_time_binding,
            // time_binding,
            start_time,
            keys_down: vec![],
            touch_positions: HashMap::new(),
            ground_shader,
            sphere_model,
            banana_instances_gen,
            post_processing_shader,
            screen_compute,
            text_brush,
            text_section,
        }
    }
}

impl WindowHandler for Game {
    fn resize(&mut self, surface_ctx: &dyn SurfaceCtx, new_size: Vector2<u32>) {
        self.camera.aspect = new_size.x as f32 / new_size.y as f32;
        self.screen_size = [new_size.x as f32, new_size.y as f32];
        self.text_brush.resize_view(new_size.x as f32, new_size.y as f32, surface_ctx.queue());
    }

    fn render<'s: 'b, 'b>(&'s mut self, surface_ctx: &dyn SurfaceCtx, render_pass: & mut RenderPass<'b>, delta: f64) {
        let speed = 0.02 * delta as f32;
        if self.keys_down.contains(&KeyCode::KeyW) {
            self.camera.eye += self.camera.get_walking_vec() * speed;
        }
        if self.keys_down.contains(&KeyCode::KeyS) {
            self.camera.eye -= self.camera.get_walking_vec() * speed;
        }
        if self.keys_down.contains(&KeyCode::KeyA) {
            self.camera.eye -= self.camera.get_right_vec() * speed;
        }
        if self.keys_down.contains(&KeyCode::KeyD) {
            self.camera.eye += self.camera.get_right_vec() * speed;
        }
        if self.keys_down.contains(&KeyCode::Space) {
            self.camera.eye += Vector3::unit_y() * speed;
        }
        if self.keys_down.contains(&KeyCode::ShiftLeft) {
            self.camera.eye -= Vector3::unit_y() * speed;
        }

        self.camera_binding.set_data(&surface_ctx.device(), self.camera.build_view_projection_matrix_raw());
        self.camera_inverse_binding.set_data(&surface_ctx.device(), self.camera.build_inverse_matrix_raw());

        render_pass.set_bind_group(0, &self.camera_binding.binding, &[]);
        
        self.delta_time_binding.set_data(&surface_ctx.device(), delta as f32 / 1000.0);
        let time = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()-self.start_time) as f32 / 1000.0;
        self.screen_info_binding.set_data(&surface_ctx.device(), [self.screen_size[0], self.screen_size[1], time, 0.0]);
        
        self.ground_shader.bind(render_pass);
        
        render_pass.set_bind_group(1, &self.delta_time_binding.binding, &[]);
        
        self.banana_instances_gen.create_bananas(&self.delta_time_binding.binding, &self.screen_info_binding.binding, &surface_ctx.device(), &surface_ctx.queue());
        // self.sphere_model.render_instances(render_pass, &self.banana_instances_gen.output_buffers()[0], 0..(self.banana_instances_gen.num_bananas[0]*self.banana_instances_gen.num_bananas[1]*self.banana_instances_gen.num_bananas[2]) -1);
        if (time as f64 + delta / 1000.0) as i32 > time as i32 { 
            self.text_section.text = vec![OwnedText::new((1000.0/delta).to_string()).with_scale(200.0)];
        }
    }

    fn config(&self) -> Option<WindowConfig> {
        Some(WindowConfig { background_color: Some(Color { r: 0.01, g: 0.01, b: 0.01, a: 1.0 }), enable_post_processing: Some(true) })
    }

    fn mouse_moved(&mut self, _surface_ctx: &dyn SurfaceCtx, _mouse_pos: PhysicalPosition<f64>) {

    }
    
    fn input_event(&mut self, _surface_ctx: &dyn SurfaceCtx, input_event: &KeyEvent) {
        if let Code(code) = input_event.physical_key {
            if input_event.state.is_pressed() {
                if !self.keys_down.contains(&code) {
                    self.keys_down.push(code);
                }
            } else {
                if let Some(i) = self.keys_down.iter().position(|x| x == &code) {
                    self.keys_down.remove(i);
                }
            }
        }
    }
    
    fn mouse_motion(&mut self, _surface_ctx: &dyn SurfaceCtx, delta: (f64, f64)) {
        self.camera.ground += (delta.0 / 500.0) as f32;
        self.camera.sky -= (delta.1 / 500.0) as f32;
        self.camera.sky = self.camera.sky.clamp(std::f32::consts::PI*-0.499, std::f32::consts::PI*0.499);
    }
    
    fn touch(&mut self, surface_ctx: &dyn SurfaceCtx, touch: &winit::event::Touch) {
        match touch.phase {
            TouchPhase::Moved => {
                if let Some(last_position) = self.touch_positions.get(&touch.id) {
                    let delta = (touch.location.x-last_position.x, touch.location.y-last_position.y);
                    self.mouse_motion(surface_ctx, delta);
                    self.touch_positions.insert(touch.id, touch.location);
                }
            }
            TouchPhase::Started => {
                self.touch_positions.insert(touch.id, touch.location);
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                self.touch_positions.remove(&touch.id);
            }
        }
    }
    
    fn post_process_render<'s: 'b, 'c: 'b, 'b>(&'s mut self, surface_ctx: &'b dyn SurfaceCtx, render_pass: & mut RenderPass<'b>, surface_texture: &'c UniformBinding<Texture>) {
        let writable_texture = Texture::blank_texture(surface_ctx.device(), surface_texture.value.texture.width(), surface_texture.value.texture.height(), wgpu::TextureFormat::Rgba32Float);
        let writable_texture_uniform = UniformBinding::new(surface_ctx.device(), "Writable Texture", StorageTexture::from_texture(writable_texture), None);
        self.screen_compute.render_instances(&self.banana_instances_gen, &writable_texture_uniform, &self.camera_binding, &self.screen_info_binding, surface_ctx.device(), surface_ctx.queue());
        self.post_processing_shader.bind(render_pass);
        render_pass.set_bind_group(0, &writable_texture_uniform.binding, &[]);
        render_pass.set_bind_group(1, &surface_ctx.depth_texture().binding, &[]);
        render_pass.set_bind_group(2, &surface_texture.binding, &[]);
        render_pass.set_bind_group(3, &self.screen_info_binding.binding, &[]);
        render_pass.set_bind_group(4, &self.camera_binding.binding, &[]);
        render_pass.set_bind_group(5, &self.camera_inverse_binding.binding, &[]);

        surface_ctx.screen_model().render(render_pass);
        self.text_brush.queue(surface_ctx.device(), surface_ctx.queue(), vec![&self.text_section]).unwrap();
        self.text_brush.draw(render_pass);
    }
    
    fn limits() -> wgpu::Limits {
        Limits {
            max_bind_groups: 7,
            ..Default::default()
        }
    }
    
    fn other_window_event(&mut self, _surface_ctx: &dyn SurfaceCtx, _event: &winit::event::WindowEvent) {
        
    }
    
    fn surface_config() -> Option<bespoke_engine::window::SurfaceConfig> {
        None
    }

    fn required_features() -> wgpu::Features {
        Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
    }
}