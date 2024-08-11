use std::ops::Range;

use nalgebra::Vector3;
use trk_io::{Point, Reader};
use wgpu::{Buffer, Device};

use super::{buffer, vertex::FiberVertex, Client};

type Streamline = Vec<Point>;

pub struct FiberBatch {
    pub vertices: Buffer,
    pub indices: Buffer,
    pub index_count: u32,
}

impl FiberBatch {
    fn new(streamlines: Vec<Streamline>, device: &Device) -> Self {
        let (vertices, indices) = geometry(streamlines);
        let name = "Fiber";

        Self {
            vertices: buffer::init_vertices(name, &vertices, device),
            indices: buffer::init_indices(name, &indices, device),
            index_count: indices.len() as u32,
        }
    }
}

pub fn batches(fibers: Reader, client: &Client) -> Vec<FiberBatch> {
    let mut iter = fibers.into_streamlines_iter().peekable();
    let mut batches = vec![];

    while iter.peek().is_some() {
        let streamlines = iter.by_ref().take(client.streamline_batch_size).collect();
        batches.push(FiberBatch::new(streamlines, &client.device));
    }
    batches
}

fn geometry(streamlines: Vec<Streamline>) -> (Vec<FiberVertex>, Vec<u32>) {
    let (mut vertices, ranges) = vertices(streamlines);
    let mut indices: Vec<u32> = Vec::with_capacity((vertices.len() - ranges.len()) * 2);

    for range in ranges {
        for i in range.start..range.end {
            vertices[i].direction = (vertices[i + 1].position - vertices[i].position).normalize();
            indices.extend([i as u32, i as u32 + 1]);
        }
        // Last point of the streamline uses the previous direction.
        vertices[range.end].direction = vertices[range.end - 1].direction;
    }
    (vertices, indices)
}

fn vertices(streamlines: Vec<Streamline>) -> (Vec<FiberVertex>, Vec<Range<usize>>) {
    let mut delimiter = 0;
    let mut ranges = vec![]; // Streamlines ranges

    let vertices: Vec<FiberVertex> = streamlines
        .into_iter()
        .flat_map(|streamline| {
            let start = delimiter;
            ranges.push(start..start + streamline.len() - 1);
            delimiter += streamline.len();

            streamline.into_iter().map(|point| FiberVertex {
                position: point,
                direction: Vector3::default(), // Calculated later
            })
        })
        .collect();

    (vertices, ranges)
}
