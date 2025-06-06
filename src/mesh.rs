use bevy_color::{Color, ColorToComponents, Srgba};
use bevy_math::Vec3;
use bevy_render::{
    mesh::{Indices, Mesh, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};

#[derive(Default)]
pub struct PointsMesh {
    pub vertices: Vec<Vec3>,
    pub colors: Option<Vec<Color>>,
}

impl FromIterator<Vec3> for PointsMesh {
    fn from_iter<T: IntoIterator<Item = Vec3>>(iter: T) -> Self {
        Self {
            vertices: iter.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl From<Mesh> for PointsMesh {
    fn from(m: Mesh) -> Self {
        if let Some(VertexAttributeValues::Float32x3(arr)) = m.attribute(Mesh::ATTRIBUTE_POSITION) {
            let mut p = PointsMesh {
                vertices: arr.iter().map(|item| Vec3::from_array(*item)).collect(),
                ..Default::default()
            };
            if let Some(VertexAttributeValues::Float32x4(array)) =
                m.attribute(Mesh::ATTRIBUTE_COLOR)
            {
                p.colors = Some(
                    array
                        .iter()
                        .map(|item| Srgba::from_f32_array(*item).into())
                        .collect(),
                );
            }
            return p;
        }
        Self::default()
    }
}

impl From<PointsMesh> for Mesh {
    fn from(m: PointsMesh) -> Self {
        let vertices: Vec<[f32; 3]> = m
            .vertices
            .iter()
            .flat_map(|p| {
                let arr = p.to_array();
                [arr, arr, arr, arr]
            })
            .collect();
        let uv_set = [[0., 0.], [1., 0.], [1., 1.], [0., 1.]];
        let uvs: Vec<[f32; 2]> = m.vertices.iter().flat_map(|_| uv_set).collect();
        let indices = Indices::U32(
            m.vertices
                .iter()
                .enumerate()
                .flat_map(|(i, _)| {
                    let idx = (i * 4) as u32;
                    [idx, idx + 1, idx + 3, idx + 2, idx + 3, idx + 1]
                })
                .collect(),
        );
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        if let Some(color) = m.colors {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                color
                    .iter()
                    .flat_map(|c| {
                        let arr = c.to_srgba().to_f32_array();
                        [arr, arr, arr, arr]
                    })
                    .collect::<Vec<[f32; 4]>>(),
            );
        }
        mesh.insert_indices(indices);
        mesh
    }
}
