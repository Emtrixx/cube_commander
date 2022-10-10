struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

// CAMERA
struct CameraUniform {
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: CameraUniform;


// LIGHT
struct LightUniform {
    position: vec3<f32>,
    color: vec3<f32>,
}
@group(1) @binding(0)
var<uniform> light: LightUniform;


// VERTEX OUT
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

// VERTEX SHADER
@vertex 
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.world_position = in.position;
    out.color = in.color;
    out.normal = in.normal;
    return out;
}



// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = vec4(in.color, 1.0);

    // Ambient Light
    let ambient_strength = 0.05;
    let ambient_color = light.color * ambient_strength;

    // Diffuse Positional Light
    let light_dir = normalize(light.position - in.world_position);
    let diffuse_strength = max(dot(in.normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_strength;
    // Diffuse Directional Light
    // let light_dir: vec3<f32> = normalize(vec3(2.0, 3.0, 0.0));
    // let diffuse_strength = max(dot(in.normal, light_dir), 0.0);
    // let diffuse_color = light.color * diffuse_strength;

    let result = object_color.xyz * (ambient_color + diffuse_color);

    return vec4<f32>(result, object_color.a);
}