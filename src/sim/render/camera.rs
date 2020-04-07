//! Camera implementation.

use crate::{access, clone, img::AspectRatio};
use nalgebra::{Point3, Unit, Vector3};

/// Image building structure.
pub struct Camera {
    /// Position.
    pos: Point3<f64>,
    /// Target point.
    tar: Point3<f64>,
    /// Forward direction.
    forward: Unit<Vector3<f64>>,
    /// Up axis.
    up: Unit<Vector3<f64>>,
    /// Right axis.
    right: Unit<Vector3<f64>>,
    /// Field of view.
    fov: (f64, f64),
    /// Image resolution.
    res: (usize, usize),
    /// Scanning deltas.
    delta: (f64, f64),
    /// Sub-sampling deltas.
    sub_delta: (f64, f64),
}

impl Camera {
    access!(pos, Point3<f64>);
    access!(tar, Point3<f64>);
    access!(forward, Unit<Vector3<f64>>);
    access!(up, Unit<Vector3<f64>>);
    access!(right, Unit<Vector3<f64>>);
    clone!(fov, (f64, f64));
    clone!(res, (usize, usize));
    clone!(delta, (f64, f64));
    clone!(sub_delta, (f64, f64));

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        pos: Point3<f64>,
        tar: Point3<f64>,
        fov_hz: f64,
        aspect_ratio: &AspectRatio,
        tar_pix: usize,
        ss_power: usize,
    ) -> Self {
        debug_assert!(fov_hz > 0.0);
        debug_assert!(tar_pix > 0);

        let forward = Unit::new_normalize(tar - pos);
        let up = Vector3::z_axis();
        let right = Unit::new_normalize(forward.cross(&up));

        let fov = (fov_hz, fov_hz / aspect_ratio.ratio());
        let res = aspect_ratio.resolution(tar_pix);

        let delta = (fov.0 / (res.0 - 1) as f64, fov.1 / (res.1 - 1) as f64);
        let sub_delta = (delta.0 / ss_power as f64, delta.1 / ss_power as f64);

        Self {
            pos,
            tar,
            forward,
            up,
            right,
            fov,
            res,
            delta,
            sub_delta,
        }
    }

    /// Calculate the number of pixels in the final image.
    #[inline]
    #[must_use]
    pub fn total_pixels(&self) -> usize {
        self.res.0 * self.res.1
    }
}
