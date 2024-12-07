t_rendered: $0;
t_depth: $1;
t_screen: $2,0;
s_screen: $2,1;

screen_info: $3;
camera: $4;
camera_inverse: $5;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var screen = textureLoad(t_rendered, vec2<u32>(u32(in.tex_coords.x*screen_info.screen_size.x), u32(in.tex_coords.y*screen_info.screen_size.y)));
    if (screen.w == 0.0) {
        screen = textureSample(t_screen, s_screen, in.tex_coords);
    }
    return screen;
}