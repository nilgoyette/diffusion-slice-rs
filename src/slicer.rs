use std::str::FromStr;

use ndarray::{Array2, Array3, ShapeBuilder};
use nifti::NiftiHeader;

pub type ImageSlice = Array2<u8>;
pub type Spacing = (f32, f32, f32);

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    Sagittal = 0,
    Coronal,
    Axial,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum View {
    Left,
    Right, // TODO Switch camera
    Anterior,
    Posterior, // TODO Switch camera
    Superior,
    Inferior, // TODO Switch camera
}

impl FromStr for View {
    type Err = ();

    fn from_str(input: &str) -> Result<View, Self::Err> {
        match input {
            "left" => Ok(View::Left),
            "right" => Ok(View::Right),
            "anterior" => Ok(View::Anterior),
            "posterior" => Ok(View::Posterior),
            "superior" => Ok(View::Superior),
            "inferior" => Ok(View::Inferior),
            _ => Err(()),
        }
    }
}

impl View {
    pub fn axis(&self) -> Axis {
        match self {
            View::Left | View::Right => Axis::Sagittal,
            View::Anterior | View::Posterior => Axis::Coronal,
            View::Superior | View::Inferior => Axis::Axial,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            View::Left => "left",
            View::Right => "right",
            View::Anterior => "anterior",
            View::Posterior => "posterior",
            View::Superior => "superior",
            View::Inferior => "inferior",
        }
    }
}

pub struct Slice {
    pub data: ImageSlice,
    pub view: View,
    pub index: usize,
    pub depth: f32,
}

pub struct Slicer {
    pub header: NiftiHeader,
    pub spacing: Spacing,
    pub slices: Vec<Slice>,
}

impl Slicer {
    pub fn from_3d(
        header: NiftiHeader,
        data: Array3<f32>,
        nb_slices: usize,
        views: &[View],
        range: (f32, f32),
    ) -> Self {
        // Rescale whatever we've got to u8
        let min_value = data.fold(f32::MAX, |acc, &v| f32::min(acc, v));
        let max_value = data.fold(f32::MIN, |acc, &v| f32::max(acc, v));
        let image_range = max_value - min_value;
        let type_min = 0.0;
        let type_max = 255.0;
        let type_range = type_max - type_min;
        let rescale = |x| ((x - min_value) * (type_range / image_range) + type_min) as u8;
        let spacing = (header.pixdim[1], header.pixdim[2], header.pixdim[3]);

        // I tried doing a views.flat_map(indices.map()) but I have a borrow checker problem that
        // I'm unsable to fix.
        let mut slices = Vec::with_capacity(views.len() * nb_slices);
        for &view in views {
            let axis = view.clone().axis();
            for idx in build_indices(&data, nb_slices, axis, range) {
                let slice: Array2<u8> = data
                    .index_axis(ndarray::Axis(axis as usize), idx)
                    .mapv(rescale);

                // A NIfTI image is stored in 'f' order, but it doesn't know that much itself
                // so ndarray thinks it's the standard 'c' order. Because of this, we convert
                // it to 'f' order, which actually converts it to 'c' order. Voilà.
                let mut data_c = Array2::zeros(slice.dim().f());
                data_c.assign(&slice);

                slices.push(Slice {
                    data: data_c,
                    view,
                    index: idx,
                    depth: 0.0, // TODO Spacing * index
                });
            }
        }

        Slicer {
            header,
            spacing,
            slices,
        }
    }
}

fn build_indices(
    data: &Array3<f32>,
    mut nb_slices: usize,
    axis: Axis,
    range: (f32, f32),
) -> Vec<usize> {
    // TODO We can calculate the bbox to avoid the blank zones
    let width = data.shape()[axis as usize] as f32;
    let max_idx = width * range.1;
    let min_idx = width * range.0;

    if nb_slices == 1 {
        // User requested a single image. Lets give him the middle slice.
        vec![((min_idx + max_idx) / 2.0).round() as usize]
    } else {
        let mut step = (max_idx - min_idx) / (nb_slices - 1) as f32;
        if step < 1.0 {
            nb_slices = ((nb_slices + 1) as f32 * step) as usize;
            step = 1.0;
        }
        (0..nb_slices)
            .map(|i| (i as f32 * step + min_idx).round() as usize)
            .collect()
    }
}
