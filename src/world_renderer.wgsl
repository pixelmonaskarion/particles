@group(0)
@binding(0)
var particles: texture_storage_3d<rgba32float, read_write>;

@group(1) @binding(0)
var screen_3d: texture_storage_3d<rgba8unorm, read_write>;

@group(2) @binding(0)
var<uniform> global_workgroup_size: vec3<u32>;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let position = textureLoad(particles, global_id);
    if (
        position.x >= 0.0 && position.x < f32(global_workgroup_size.x) && 
        position.y >= 0.0 && position.y < f32(global_workgroup_size.y) && 
        position.z >= 0.0 && position.z < f32(global_workgroup_size.z)) {
            let new_color = textureLoad(particles, vec3u(global_id.x + global_workgroup_size.x * 2, global_id.yz));
            let tex_pos = vec3u(u32(position.x), u32(position.y), u32(position.z));
            let current = textureLoad(screen_3d, tex_pos);
            if (current.w < 1.0) {
                // let new_color = vec4f(position.xyz/1000.0, 0.1);
                let color = mix_colors(current, new_color);
                textureStore(screen_3d, tex_pos, color);
            }
        }
}

fn mix_colors(back: vec4f, front: vec4f) -> vec4f{
    let pre_back = vec4f(back.rgb * back.a, back.a);
    let pre_front = vec4f(front.rgb * front.a, front.a);
    let final_rgb = back.rgb + (front.rgb * (1 - back.a));       
    let final_a = back.a + (front.a * (1.0 - back.a));
    return vec4f(final_rgb, final_a);

}

fn quaternion_to_matrix(quat: vec4f) -> mat4x4f {
    let x2 = quat.x + quat.x;
    let y2 = quat.y + quat.y;
    let z2 = quat.z + quat.z;

    let xx2 = x2 * quat.x;
    let xy2 = x2 * quat.y;
    let xz2 = x2 * quat.z;

    let yy2 = y2 * quat.y;
    let yz2 = y2 * quat.z;
    let zz2 = z2 * quat.z;

    let sy2 = y2 * quat.w;
    let sz2 = z2 * quat.w;
    let sx2 = x2 * quat.w;

    return mat4x4f(
        1.0 - yy2 - zz2, xy2 + sz2, xz2 - sy2, 0.0,
        xy2 - sz2, 1.0 - xx2 - zz2, yz2 + sx2, 0.0,
        xz2 + sy2, yz2 - sx2, 1.0 - xx2 - yy2, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
}

fn quaternion(axis: vec3f, angle: f32) -> vec4f {
    return vec4f(axis*sin(angle*0.5), cos(angle*0.5));
}

fn pcg(n: u32) -> u32 {
    var h = n * 747796405u + 2891336453u;
    h = ((h >> ((h >> 28u) + 4u)) ^ h) * 277803737u;
    return (h >> 22u) ^ h;
}

fn pcg2d(p: vec2u) -> vec2u {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    v.x += v.y * 1664525u; v.y += v.x * 1664525u;
    v ^= v >> vec2u(16u);
    return v;
}

fn pcg3d(p: vec3u) -> vec3u {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y*v.z; v.y += v.z*v.x; v.z += v.x*v.y;
    v ^= v >> vec3u(16u);
    v.x += v.y*v.z; v.y += v.z*v.x; v.z += v.x*v.y;
    return v;
}