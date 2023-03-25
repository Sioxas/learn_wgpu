// 顶点着色器

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}


// 片元着色器

[[group(0), binding(0)]]
var y_texture: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var y:f32 = textureSample(y_texture, s_diffuse, in.tex_coords).r;
    var u:f32 = 0.0;
    var v:f32 = 0.0;
    var rgb: vec3<f32>;
    rgb.r = y + (1.403 * v);
    rgb.g = y - (0.344 * u) - (0.714 * v);
    rgb.b = y + (1.770 * u);
    return vec4<f32>(rgb, 1.0);
}

