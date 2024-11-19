struct ScreenInfo {
    screen_size: vec2f,
    time: f32,
}

@group(0) @binding(0)
var t_rendered: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(0)
var t_depth: texture_depth_2d;
@group(2) @binding(0)
var t_screen: texture_2d<f32>;
@group(2) @binding(1)
var s_screen: sampler;

@group(3) @binding(0) var<uniform> screen_info: ScreenInfo;
@group(4) @binding(0) var<uniform> camera: mat4x4<f32>;
@group(5) @binding(0) var<uniform> camera_inverse: mat4x4<f32>;

@group(6) @binding(0)
var t_world: texture_3d<f32>;
@group(6) @binding(1)
var s_world: sampler;

@group(7) @binding(0) var<uniform> camera_pos: vec3f;

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
    // var screen = textureLoad(t_rendered, vec2<u32>(u32(in.tex_coords.x*screen_info.screen_size.x), u32(in.tex_coords.y*screen_info.screen_size.y)));
    var color = vec4f(0.0);
    var position = camera[3].xyz;
    let z = 10.0;
    let clipPos = vec4(in.tex_coords.x * 2.0 - 1.0, in.tex_coords.y * -2.0 + 1.0, z, 1.0) - vec4f(camera_pos, 0.0);
    let viewPos = camera_inverse * clipPos;
    let worldPos = viewPos.xyz / viewPos.w;
    let direction = worldPos / distance(worldPos, vec3f(0.0));
    return vec4f(direction, 1.0);

    // let world_size = vec3f(1000.0);
    // while (
    //     position.x > -2*world_size.x && position.x < 2*world_size.x &&
    //     position.y > -2*world_size.y && position.y < 2*world_size.y &&
    //     position.z > -2*world_size.z && position.z < 2*world_size.z
    //     ) {
    //         position += direction;
    //         if (
    //             position.x > 0.0 && position.x < world_size.x &&
    //             position.y > 0.0 && position.y < world_size.y &&
    //             position.z > 0.0 && position.z < world_size.z
    //         ) {
    //             var screen = textureSample(t_world, s_world, position/world_size);
    //             color = mix_colors(color, screen);
    //         } 
        
    // }
    
    // return color;
}

fn mix_colors(back: vec4f, front: vec4f) -> vec4f{
    let pre_back = vec4f(back.rgb * back.a, back.a);
    let pre_front = vec4f(front.rgb * front.a, front.a);
    let final_rgb = back.rgb + (front.rgb * (1 - back.a));       
    let final_a = back.a + (front.a * (1.0 - back.a));
    return vec4f(final_rgb, final_a);

}