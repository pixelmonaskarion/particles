particles: $0;

global_workgroup_size: $1;

offset: $2;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // let i = global_id.x * global_workgroup_size.y * global_workgroup_size.z + global_id.y * global_workgroup_size.z + global_id.z;
    var position = vec4f(f32(global_id.x + offset.x), f32(global_id.y + offset.y), f32(global_id.z + offset.z), 1.0);
    // let dist = distance(position.xyz, vec3f(f32(global_workgroup_size.x)/2, f32(global_workgroup_size.y)/2, f32(global_workgroup_size.z)/2));
    // if (dist >= f32(global_id.x)/2.0) {
    //     position = vec4f(0.0, 0.0, 0.0, 1.0);
    // }
    let random_dir = pcg3d(global_id);
    let velocity = vec4f(
        (f32(random_dir.x % 100000) - 50000.0) / 5000.0,
        (f32(random_dir.y % 100000) - 50000.0) / 5000.0,
        (f32(random_dir.z % 100000) - 50000.0) / 5000.0,
        1.0
    );
    // instance.velocity /= distance(instance.velocity.xyz, vec3f(0.0, 0.0, 0.0));
    // instance.velocity.w = 1.0;
    // instance.color = vec4f(f32(global_id.x)/f32(global_workgroup_size.x), f32(global_id.y)/f32(global_workgroup_size.y), f32(global_id.z)/f32(global_workgroup_size.z), 1.0);
    // instance.color = vec4f(1.0);

    textureStore(particles, global_id, position);
    textureStore(particles, vec3u(global_id.x + global_workgroup_size.x, global_id.yz), velocity);
    textureStore(particles, vec3u(global_id.x + global_workgroup_size.x * 2, global_id.yz), vec4f(f32(global_id.x)/f32(global_workgroup_size.x), f32(global_id.y)/f32(global_workgroup_size.y), f32(global_id.z)/f32(global_workgroup_size.z), 0.4));
}