dst_particles: $0;

delta_time: $1;

global_workgroup_size: $2;

screen_info: $3;

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