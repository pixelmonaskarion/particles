@group(0)
@binding(0)
var dst_particles: texture_storage_3d<rgba32float, read_write>;

@group(1) @binding(0)
var<uniform> delta_time: f32;

@group(2) @binding(0)
var<uniform> global_workgroup_size: vec3<u32>;

@group(3) @binding(0)
var<uniform> screen_info: ScreenInfo;

struct ScreenInfo {
    screen_size: vec2f,
    time: f32,
}

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var position = textureLoad(dst_particles, global_id);
    var velocity = textureLoad(dst_particles, vec3u(global_id.x + global_workgroup_size.x, global_id.yz));
    // let random_dir_0u = pcg4d(vec4<u32>(global_id, u32(screen_info.time)));
    // let random_dir_1u = pcg4d(vec4<u32>(global_id, u32(screen_info.time)+1));
    // let random_dir_0 = vec3f((f32(random_dir_0u.x % 100000) - 50000.0) / 50000.0, (f32(random_dir_0u.y % 100000) - 50000.0) / 50000.0, (f32(random_dir_0u.z % 100000) - 50000.0) / 50000.0);
    // let random_dir_1 = vec3f((f32(random_dir_1u.x % 100000) - 50000.0) / 50000.0, (f32(random_dir_1u.y % 100000) - 50000.0) / 50000.0, (f32(random_dir_1u.z % 100000) - 50000.0) / 50000.0);
    // velocity += vec4f((random_dir_1-random_dir_0) * delta_time, 1.0);



    // last_frame.velocity.y -= 9.0;
    // if (screen_info.time*1000.0 > f32(i)) {
        position.x += velocity.x * delta_time;
        position.y += velocity.y * delta_time;
        position.z += velocity.z * delta_time;
    // }
    // if (last_frame.model_matrix_3.y < -100.0) {
    //     last_frame.model_matrix_3.y = -100.0;
    //     last_frame.velocity.y *= -0.99;
    //     last_frame.velocity.x *= 0.99;
    //     last_frame.velocity.z *= 0.99;
    // }
    // for (var j = 0u; j < global_workgroup_size.x*global_workgroup_size.y*global_workgroup_size.z; j++) {
    //     let other_molecule = copy_instances[j];
    //     let delta = other_molecule.model_matrix_3.xyz-last_frame.model_matrix_3.xyz;
    //     let dist = distance(delta, vec3f(0.0, 0.0, 0.0));
    //     if (dist != 0.0) {
    //         var force = 0.2/dist;
    //         force -= 0.01/dist;
    //         last_frame.velocity.x += delta.x*force;
    //         last_frame.velocity.y += delta.y*force;
    //         last_frame.velocity.z += delta.z*force;
    //     }
    // }
    // last_frame.position = vec4f(f32(global_id.x), f32(global_id.y), f32(global_id.z), 1.0);
    textureStore(dst_particles, global_id, position);
    textureStore(dst_particles, vec3u(global_id.x + global_workgroup_size.x, global_id.yz), velocity);
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

fn pcg4d(p: vec4u) -> vec4u {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y*v.w; v.y += v.z*v.x; v.z += v.x*v.y; v.w += v.y*v.z;
    v ^= v >> vec4u(16u);
    v.x += v.y*v.w; v.y += v.z*v.x; v.z += v.x*v.y; v.w += v.y*v.z;
    return v;
}