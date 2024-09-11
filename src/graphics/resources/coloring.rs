use std::ops::Range;

use nalgebra::Vector3;

use super::vertex::FiberVertex;

#[derive(Clone, Copy)]
pub enum Coloring {
    Local,
    Endpoint,
    Uniform(Vector3<u32>),
}

impl Coloring {
    pub fn assign_vertex_colors(&self, vertices: &mut [FiberVertex], ranges: &[Range<usize>]) {
        match self {
            Coloring::Local => Self::assign_local_color(vertices, ranges),
            Coloring::Endpoint => Self::assign_endpoint(vertices, ranges),
            Coloring::Uniform(color) => Self::assign_uniform(vertices, *color),
        }
    }

    fn assign_local_color(vertices: &mut [FiberVertex], ranges: &[Range<usize>]) {
        for range in ranges {
            for i in range.start..range.end {
                vertices[i].color = (vertices[i + 1].position - vertices[i].position)
                    .normalize()
                    .abs();
            }
            // Last point of the streamline uses the previous color.
            vertices[range.end].color = vertices[range.end - 1].color;
        }
    }

    fn assign_endpoint(vertices: &mut [FiberVertex], ranges: &[Range<usize>]) {
        for range in ranges {
            let (first, last) = (&vertices[range.start], &vertices[range.end]);
            let color = (first.position - last.position).normalize().abs();

            for i in range.start..=range.end {
                vertices[i].color = color
            }
        }
    }

    fn assign_uniform(vertices: &mut [FiberVertex], color: Vector3<u32>) {
        let color = color.map(|c| c as f32 / 255.);

        for vertex in vertices {
            vertex.color = color;
        }
    }
}
