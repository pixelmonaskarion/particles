particles: $0;

t_screen: $1;

global_workgroup_size: $2;

camera: $3;

screen_info: $4;

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
    let clip_position = camera.view_proj * position;
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