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
var u_texture: texture_2d<f32>;
[[group(0), binding(2)]]
var v_texture: texture_2d<f32>;
[[group(0), binding(3)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var y:f32 = textureSample(y_texture, s_diffuse, in.tex_coords).r;
    var u:f32 = textureSample(u_texture, s_diffuse, in.tex_coords).r;
    var v:f32 = textureSample(v_texture, s_diffuse, in.tex_coords).r;

    if(y == 0.0 || u == 0.0 || v == 0.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    y = y - 0.0625;
    u = u - 0.5;
    v = v - 0.5;


    // var y:f32 = 0.0;
    // var u:f32 = 0.0;
    // var v:f32 = 0.0;
    var rgb: vec3<f32>;
    // rgb.r = y + (1.403 * v);
    // rgb.g = y - (0.344 * u) - (0.714 * v);
    // rgb.b = y + (1.770 * u);
    // rgb.r = y + 1.13983 * v;
    // rgb.g = y - 0.39465 * u - 0.58060 * v;
    // rgb.b = y + 2.03211 * u;
    rgb.r = 1.164 * y + 1.792 * v;
    rgb.g = 1.164 * y - 0.213 * u - 0.534 * v;
    rgb.b = 1.164 * y + 2.114 * u;
    return vec4<f32>(rgb, 1.0);
}
// [[stage(fragment)]]
// fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
//     var Y:u32 = textureSample(y_texture, s_diffuse, in.tex_coords).r;
//     var Cb:u32 = textureSample(u_texture, s_diffuse, in.tex_coords).r;
//     var Cr:u32 = textureSample(v_texture, s_diffuse, in.tex_coords).r;

//     var R:f32 = 1.164 *(Y - 16.0) + 1.596 *(Cr - 128.0);                    
//     var G:f32 = 1.164 *(Y - 16.0) - 0.392 *(Cb - 128.0) - 0.812 *(Cr - 128.0);
//     var B:f32 = 1.164 *(Y - 16.0) + 2.016 *(Cb - 128.0);   
//     R = R / 255.0;
//     G = G / 255.0;
//     B = B / 255.0;     
//     return vec4<f32>(R, G, B, 1.0);
// }

