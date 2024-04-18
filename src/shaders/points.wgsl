#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::mesh_bindings::mesh
#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_world, mesh_position_local_to_clip}

struct PointMaterial {
    point_size: f32,
    opacity: f32,
    color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: PointMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(2) color: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(1) color: vec4<f32>,
#endif
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let uv = vertex.uv;
    let delta: vec2<f32> = (uv - vec2<f32>(0.5, 0.5)) * material.point_size;
    let world = mesh_position_local_to_world(get_model_matrix(vertex.instance_index), vec4<f32>(vertex.position, 1.0));
#ifdef POINT_SIZE_PERSPECTIVE
    var view_position: vec4<f32> = view.inverse_view * world;
    view_position = vec4<f32>(view_position.xy - delta.xy, view_position.zw);
    let clip_position = view.projection * view_position;
#else
    var clip_position = view.view_proj * world;
    let r: f32 = view.viewport.z / view.viewport.w;
    let s: f32 = max(view.viewport.z, view.viewport.w);
    let w: f32 = clip_position.w / s;
    clip_position = vec4<f32>(clip_position.xy - delta * vec2(1.0, r) * w, clip_position.zw);
#endif
    out.clip_position = clip_position;
    out.uv = uv;
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
    return out;
}

struct FragmentInput {
    @location(0) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(1) color: vec4<f32>,
#endif
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
#ifdef POINT_SHAPE_CIRCLE
    let d: f32 = distance(input.uv, vec2<f32>(0.5, 0.5));
    if d > 0.5 {
        discard;
    }
#endif
#ifdef VERTEX_COLORS
    return material.color * input.color * vec4(1.0, 1.0, 1.0, material.opacity);
#else
    return material.color * vec4(1.0, 1.0, 1.0, material.opacity);
#endif
}