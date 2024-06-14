# diffusion-slice-rs

Take screenshots of 3D and 4D images

This project aims to

- offer a simple cmdline application
- build a [wgpu](https://github.com/gfx-rs/wgpu) 3D scene from
  - a slice of a NIfTI image (`.nii; .nii.gz`)
  - if required, a TrackVis file (`.trk`)
- save the buffer into a 2D image (`.png`)
- not build a windows; the process must be done in offcreen rendering

This is **not** an image viewer (like [MI-Brain](https://github.com/imeka/mi-brain) and others) and we have no intention to create one.

## Roadmap

- [ ] Display toy image data and save the result to png
- [ ] Choose the surface dimension
- [ ] Load actual NIfTI files, using [nifti-rs](https://github.com/Enet4/nifti-rs)
- [ ] Display characters in the image
- [ ] Save several image slices instead of one
- [ ] Display toy streamlines/fibers, using [trk-io](https://github.com/imeka/trk-io)
- [ ] Add various streamlines display options
- [ ] Load actual TrackVis files

The following features might be added

- [ ] White background option (instead of black)
- [ ] LUT to color the image
