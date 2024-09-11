use std::ops::Range;

use nalgebra::Vector3;
use trk_io::{Point, Reader};
use wgpu::Buffer;

use super::{buffer, vertex::FiberVertex, Client, Coloring};

type Streamline = Vec<Point>;

pub struct FiberBatch {
    pub vertices: Buffer,
    pub indices: Buffer,
    pub index_count: u32,
}

impl FiberBatch {
    fn new(streamlines: Vec<Streamline>, client: &Client) -> Self {
        let (vertices, indices) = geometry(streamlines, &client.coloring);

        let name = "Fiber";

        Self {
            vertices: buffer::init_vertices(name, &vertices, &client.device),
            indices: buffer::init_indices(name, &indices, &client.device),
            index_count: indices.len() as u32,
        }
    }
}

pub fn batches(fibers: Reader, client: &Client) -> Vec<FiberBatch> {
    let mut iter = fibers.into_streamlines_iter();

    std::iter::from_fn(|| {
        let streamlines: Vec<Streamline> =
            iter.by_ref().take(client.streamline_batch_size).collect();

        (!streamlines.is_empty()).then(|| FiberBatch::new(streamlines, client))
    })
    .collect()
}

fn geometry(streamlines: Vec<Streamline>, coloring: &Coloring) -> (Vec<FiberVertex>, Vec<u32>) {
    let (mut vertices, ranges) = vertices(streamlines);
    coloring.assign_vertex_colors(&mut vertices, &ranges);

    (vertices, indices(&ranges))
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
                color: Vector3::default(), // Calculated later
            })
        })
        .collect();

    (vertices, ranges)
}

fn indices(ranges: &[Range<usize>]) -> Vec<u32> {
    ranges
        .iter()
        .cloned()
        .flat_map(|range| range.flat_map(|i| vec![i as u32, i as u32 + 1]))
        .collect()
}
