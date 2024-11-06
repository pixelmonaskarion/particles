@group(0) @binding(0) var<uniform> camera: mat4x4<f32>;
@group(1) @binding(0) var<uniform> time: f32;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    // @location(3) padding: vec3<f32>,
};

struct InstanceInput {
    @location(5) position: vec4f,
    @location(6) _velocity: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    let matrix = mat4x4f (
        vec4f(1.0, 0.0, 0.0, 0.0),
        vec4f(0.0, 1.0, 0.0, 0.0),
        vec4f(0.0, 0.0, 1.0, 0.0),
        instance.position
    );
    out.clip_position = camera * matrix * vec4<f32>(model.position, 1.0);
    out.color = model.color.xyz;
    out.normal = model.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4f(in.color*(dot(in.normal, vec3f(0.0, 1.0, 0.0))/2.0 + 0.5), 1.0);
    // return vec4f(in.color*dot(in.normal, vec3f(cos(time/10.0), sin(time/10.0), 0.0)), 1.0);
}