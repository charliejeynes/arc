//! Photon implementation.

<<<<<<< HEAD
use crate::{access, clone, geom::Ray};
=======
use crate::{clone, geom::Ray};
>>>>>>> 671c3d8935608ac0c3232ccb50f845e19b0e7372

/// Photon structure.
#[derive(Debug)]
pub struct Photon {
    /// Statistical weight.
    weight: f64,
    /// Wavelength [m].
    wavelength: f64,
    /// Power [J/s].
    power: f64,
    /// Ray of travel.
    ray: Ray,
}

impl Photon {
    clone!(weight, weight_mut, f64);
    clone!(wavelength, f64);
    clone!(power, f64);
<<<<<<< HEAD
    access!(ray, ray_mut, Ray);
=======
    clone!(ray, ray_mut, Ray);
>>>>>>> 671c3d8935608ac0c3232ccb50f845e19b0e7372

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(wavelength: f64, power: f64, ray: Ray) -> Self {
        assert!(wavelength > 0.0);
        assert!(power > 0.0);

        Self {
            weight: 1.0,
            wavelength,
            power,
            ray,
        }
    }
}
