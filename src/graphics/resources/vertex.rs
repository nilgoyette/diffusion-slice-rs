use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use nalgebra::{Point3, Vector3};
use wgpu::{vertex_attr_array, VertexAttribute, VertexBufferLayout};

pub trait Vertex
where
    Self: Sized + Copy + Clone + Pod + Zeroable,
{
    fn buffer_layout(attributes: &[VertexAttribute]) -> VertexBufferLayout {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes,
        }
    }

    // The format of the vertex attributes array must match its struct definition.
    fn attributes() -> Vec<VertexAttribute>;
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct ImageVertex {
    pub canon: Vec2,
    pub uv: Vec2,
}

impl Vertex for ImageVertex {
    fn attributes() -> Vec<wgpu::VertexAttribute> {
        Vec::from(vertex_attr_array![0 => Float32x2, 1 => Float32x2])
    }
}

/// This `struct` uses `nalgebra` types to avoid unnecessary conversions.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct FiberVertex {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Vertex for FiberVertex {
    fn attributes() -> Vec<wgpu::VertexAttribute> {
        Vec::from(vertex_attr_array![0 => Float32x3, 1 => Float32x3])
    }
}
