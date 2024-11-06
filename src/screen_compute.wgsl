@group(0)
@binding(0)
var particles: texture_storage_3d<rgba32float, read_write>;

@group(1) @binding(0)
var t_screen: texture_storage_2d<rgba32float, read_write>;

@group(2) @binding(0)
var<uniform> global_workgroup_size: vec3<u32>;

@group(3) @binding(0) var<uniform> camera: mat4x4<f32>;

struct ScreenInfo {
    screen_size: vec2f,
    time: f32,
}

@group(4) @binding(0) var<uniform> screen_info: ScreenInfo;

const RADIUS: i32 = 1;
const RADIUS_2: f32 = 25.0;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // let i = global_id.x * global_workgroup_size.y * global_workgroup_size.z + global_id.y * global_workgroup_size.z + global_id.z;
    // let instance = instances[i];
    // textureStore(t_screen, global_id.xy, vec4f(instance.position.xyz/100.0, 1.0));
    // if (instance.position.x == 0.0 && instance.position.y == 0.0 && instance.position.z == 0.0) {
    //     textureStore(t_screen, global_id.xy, vec4f(0.0, 1.0, 0.0, 1.0));
    // }
    let position = textureLoad(particles, global_id);
    let new_color = textureLoad(particles, vec3u(global_id.x + global_workgroup_size.x * 2, global_id.yz));
    let clip_position = camera * position;
    if (clip_position.z > 0.0) {
        let screen_pos = clip_position.xy / clip_position.w;
        if (screen_pos.x >= -1.0 && screen_pos.y >= -1.0 && screen_pos.x <= 1.0 && screen_pos.y <= 1.0) {
            let tex_coords_f = vec2f((screen_pos.x+1.0)/2.0, (screen_pos.y-1.0)/-2.0)*screen_info.screen_size;
            let tex_coords = vec2<u32>(u32(tex_coords_f.x), u32(tex_coords_f.y));
            let radius_dist = RADIUS;
            for (var x = radius_dist*-1; x <= radius_dist; x++) {
                for (var y = radius_dist*-1; y < radius_dist; y++) {
                    let current = textureLoad(t_screen, vec2<u32>(tex_coords.x+u32(x), tex_coords.y+u32(y)));
                    if (current.w < 1.0) {
                        // let new_color = vec4f(position.xyz/1000.0, 0.1);
                        let color = mix_colors(current, new_color);
                        textureStore(t_screen, vec2<u32>(tex_coords.x+u32(x), tex_coords.y+u32(y)), color);
                    }
                }
            }
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