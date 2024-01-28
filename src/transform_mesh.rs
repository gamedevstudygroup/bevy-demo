use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;

pub trait Bake<T> {
    fn bake(self, transform: T) -> Self;
}

impl Bake<Transform> for Mesh {
    fn bake(mut self, transform: Transform) -> Self {
        if let Some(VertexAttributeValues::Float32x3(ref mut positions)) =
            self.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            // Apply scale, rotation, and translation to vertex positions
            positions
                .iter_mut()
                .for_each(|pos| *pos = transform.transform_point(Vec3::from_slice(pos)).to_array());
        }

        // No need to rotate normals or tangents if rotation is near identity
        if transform.rotation.is_near_identity() {
            return self;
        }

        if let Some(VertexAttributeValues::Float32x3(ref mut normals)) =
            self.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
        {
            // Rotate normals by transform rotation
            normals.iter_mut().for_each(|normal| {
                *normal = (transform.rotation * Vec3::from_slice(normal)).to_array();
            });
        }

        if let Some(VertexAttributeValues::Float32x3(ref mut tangents)) =
            self.attribute_mut(Mesh::ATTRIBUTE_TANGENT)
        {
            // Rotate tangents by transform rotation
            tangents.iter_mut().for_each(|tangent| {
                *tangent = (transform.rotation * Vec3::from_slice(tangent)).to_array();
            });
        }

        self
    }
}
