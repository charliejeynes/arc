//! Monte-Carlo radiative transfer simulation sub-module.

pub mod camera;
pub mod hit;
pub mod imaging_loop;
pub mod settings;
pub mod tracer;

pub use self::{camera::*, hit::*, imaging_loop::*, settings::*, tracer::*};

use crate::{
    util::ParProgressBar,
    world::{Grid, Verse},
};
use ndarray::Array2;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run an photography simulation.
#[inline]
#[must_use]
pub fn run(sett: &Settings, cam: &Camera, verse: &Verse, grid: &Grid) -> Array2<f64> {
    let pb = ParProgressBar::new("Imaging Loop", cam.num_pix() as u64);
    let pb = Arc::new(Mutex::new(pb));
    let thread_ids: Vec<usize> = (0..sett.num_threads()).collect();

    let num_pix = cam.num_pix();

    let mut imgs: Vec<_> = thread_ids
        .par_iter()
        .map(|id| {
            imaging_loop::run_thread(
                *id,
                sett,
                cam,
                verse,
                grid,
                &Arc::clone(&pb),
                ((num_pix as u64 / sett.num_threads() as u64) / 100).max(10) as u64,
            )
        })
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Complete.");

    let mut img = imgs.pop().expect("Did not receive any images.");
    for i in imgs {
        img += &i;
    }

    img
}
