struct Instance {
    @location(5) position: vec4<f32>,
    @location(6) velocity: vec4<f32>,
};

@group(0)
@binding(0)
var<storage, read_write> dst_instances: array<Instance>;

@group(1) @binding(0)
var<uniform> global_workgroup_size: vec3<u32>;

@group(2) @binding(0)
var<uniform> offset: vec3u;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x * global_workgroup_size.y * global_workgroup_size.z + global_id.y * global_workgroup_size.z + global_id.z;
    var instance: Instance;
    instance.position = vec4f(f32(global_id.x + offset.x), f32(global_id.y + offset.y), f32(global_id.z + offset.z), 1.0);
    // let dist = distance(matrix[3].xyz, vec3f(f32(global_id.x), f32(global_id.y), f32(global_id.z)));
    // if (dist >= f32(global_id.x)/2.0) {
        // matrix[3] = vec4f(0.0, 0.0, 0.0, 1.0);
    // }
    let random_dir = pcg3d(global_id);
    instance.velocity = vec4f(
        (f32(random_dir.x % 100000) - 50000.0) / 50000.0,
        (f32(random_dir.y % 100000) - 50000.0) / 50000.0,
        (f32(random_dir.z % 100000) - 50000.0) / 50000.0,
        1.0
    );
    // instance.velocity /= distance(instance.velocity.xyz, vec3f(0.0, 0.0, 0.0));
    // instance.velocity.w = 1.0;
    // instance.color = vec4f(f32(global_id.x)/f32(global_workgroup_size.x), f32(global_id.y)/f32(global_workgroup_size.y), f32(global_id.z)/f32(global_workgroup_size.z), 1.0);
    // instance.color = vec4f(1.0);

    dst_instances[i] = instance;
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