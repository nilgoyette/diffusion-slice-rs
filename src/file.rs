use std::path::Path;

use nalgebra::Vector3;
use ndarray::{Array3, Ix3};
use nifti::{DataElement, InMemNiftiVolume, IntoNdArray, NiftiHeader, NiftiObject, ReaderOptions};
use trk_io::Reader;

use crate::Image;

/// Read a NIfTI image into a `Array3<T>` object.
///
/// Panics if the image isn't in 3D.
pub fn read_3d_image<P, T>(path: P) -> (NiftiHeader, Array3<T>)
where
    P: AsRef<Path>,
    T: DataElement,
{
    let path = path.as_ref();
    if !path.exists() {
        panic!("Image {path:?} doesn't exist.");
    }

    let nifti_object = ReaderOptions::new()
        .fix_header(true)
        .read_file(path)
        .expect("NIfTI file has a valid and readable format.");
    let mut header = nifti_object.header().clone();
    let mut volume = nifti_object.into_volume();

    // Fix wrong dimensions on some 3D images and check if the requested dimension is equal to
    // the actual number of dimensions of the image.
    if header.dim[header.dim[0] as usize] == 1 {
        header.dim[0] -= 1;
        volume = InMemNiftiVolume::from_raw_data(&header, volume.into_raw_data()).unwrap();
    }

    let dyn_data = volume.into_ndarray::<T>().unwrap();
    let data = dyn_data
        .into_dimensionality::<Ix3>()
        .expect("Loaded NIfTI image must be a 3D array");
    (header, data)
}

/// Creates a TrackVis file reader for further data mapping.
pub fn fibers_reader<P: AsRef<Path>>(path: P, nifti_header: &NiftiHeader) -> Reader {
    let path = path.as_ref();
    if !path.exists() {
        panic!("Fibers {path:?} doesn't exist.");
    }
    let pixdim = &nifti_header.pixdim;

    Reader::new(path)
        .expect("The path exists")
        .to_voxel_space(Vector3::new(pixdim[1], pixdim[2], pixdim[3]))
}

pub fn save_image(img: Image, output_path: &Path) {
    if let Err(error) = img.save(output_path) {
        panic!("Failed to save the image at {output_path:?}: {error}");
    }
}
