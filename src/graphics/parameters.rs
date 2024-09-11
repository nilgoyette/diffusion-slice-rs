use glam::{vec2, Mat4, Vec2, Vec3};

use super::ContextInputs;

pub struct Parameters {
    pub fit_scale: f32,
    pub tractogram_alignment: Mat4,
    pub tractogram_projection: Mat4,
}

impl Parameters {
    pub fn new(inputs: &ContextInputs) -> Self {
        let (dst_size, size_3d) = (inputs.dst_img_size.as_vec2(), inputs.size_3d.as_vec3());
        let fit_scale = fit_scale(dst_size, size_3d);

        Self {
            fit_scale,
            tractogram_projection: tractogram_projection(dst_size, fit_scale * size_3d),
            tractogram_alignment: tractogram_alignment(fit_scale, size_3d),
        }
    }
}

/// Calculates the maximum scaling factor that fits within boundaries,
/// maintains the aspect ratio, and ensures uniformity across all three axes.
fn fit_scale(dst_size: Vec2, size_3d: Vec3) -> f32 {
    let max_size = vec2(size_3d.x, size_3d.z).max(vec2(size_3d.y, size_3d.y));
    (dst_size / max_size).min_element()
}

// Centers the tractogram's bounding box at the origin (0, 0) and applies scaling.
fn tractogram_alignment(fit_scale: f32, size_3d: Vec3) -> Mat4 {
    Mat4::from_scale(Vec3::splat(fit_scale)) * Mat4::from_translation(-size_3d / 2.)
}

/// Creates an orthographic projection matrix. This is used to change the basis from
/// voxel space to screen space and also defines the depth range.
fn tractogram_projection(dst_size: Vec2, scaled_size_3d: Vec3) -> Mat4 {
    let half = dst_size / 2.;
    let depth = scaled_size_3d.max_element() / 2.;
    Mat4::orthographic_rh(-half.x, half.x, -half.y, half.y, -depth, depth)
}
