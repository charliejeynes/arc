//! Grid implementation.

use crate::{
    access,
    geom::{Aabb, Ray},
    list::Cartesian::{X, Y, Z},
    math::{indexer, list},
    ord::{sort, InterSet, MatKey, MatSet, Set, SurfSet},
    sim::raman::Cell,
    util::ParProgressBar,
};
use nalgebra::{Point3, Unit, Vector3};
use ndarray::Array3;
use num_cpus;
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Material detection rays must be aimed at a triangle with at least this deviation from the triangle's plane.
const HIT_ANGLE_THRESHOLD: f64 = 1.0e-3;

/// Scaling to bump photons through boundaries within the grid.
const BUMP_SCALE: f64 = 1.0e-3;

/// Grid partition scheme.
pub struct Grid<'a> {
    /// Boundary.
    bound: Aabb,
    /// Cells.
    cells: Array3<Cell<'a>>,
}

impl<'a> Grid<'a> {
    access!(bound, Aabb);
    access!(cells, Array3<Cell<'a>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3], bound: Aabb, inters: &'a InterSet, surfs: &'a SurfSet) -> Self {
        let total_cells = res.get(X as usize).expect("Missing resolution index.")
            * res.get(1).expect("Missing resolution index.")
            * res.get(2).expect("Missing resolution index.");

        let mut cell_size = bound.widths();
        for (w, n) in cell_size.iter_mut().zip(&res) {
            *w /= *n as f64;
        }

        let pb = ParProgressBar::new("Building", total_cells as u64);
        let pb = Arc::new(Mutex::new(pb));
        let thread_ids: Vec<usize> = (0..num_cpus::get()).collect();

        let cell_blocks: Vec<_> = thread_ids
            .par_iter()
            .map(|_| {
                Self::init_cell_blocks(
                    &Arc::clone(&pb),
                    ((total_cells / num_cpus::get()) / 100).max(1) as u64,
                    &res,
                    &bound,
                    inters,
                    surfs,
                    &cell_size,
                )
            })
            .collect();
        pb.lock()
            .expect("Could not lock progress bar.")
            .finish_with_message("Grid building complete.");

        let cells = sort::stitch(cell_blocks);

        Self {
            bound,
            cells: Array3::from_shape_vec(res, cells).expect("Unable to construct cell array."),
        }
    }

    /// Initialise the cells populating the grid.
    #[inline]
    #[must_use]
    fn init_cell_blocks(
        pb: &Arc<Mutex<ParProgressBar>>,
        block_size: u64,
        res: &[usize; 3],
        bound: &Aabb,
        inters: &'a InterSet,
        surfs: &'a SurfSet,
        cell_size: &Vector3<f64>,
    ) -> Vec<(usize, Vec<Cell<'a>>)> {
        let mut cell_blocks = Vec::new();

        let gen_mat_ray = |p: &Point3<f64>| -> Ray {
            for inter in inters.map().values() {
                let surf = surfs.get(inter.surf());
                for tri in surf.tris() {
                    let tc = tri.tri().centre();

                    if bound.contains(&tc) {
                        let dir = Unit::new_normalize(tc - p);
                        if dir.dot(tri.tri().plane_norm()).abs() >= HIT_ANGLE_THRESHOLD {
                            return Ray::new(*p, dir);
                        }
                    }
                }
            }

            panic!("Unable to determine suitable material tracing ray.");
        };

        while let Some((start, end)) = {
            let mut pb = pb.lock().expect("Could not lock progress bar.");
            let b = pb.block(block_size);
            std::mem::drop(pb);
            b
        } {
            let mut cells: Vec<_> = Vec::with_capacity((end - start) as usize);
            for n in start..end {
                let index = indexer::three_dim(n as usize, *res);

                let x = cell_size.x * index[0] as f64;
                let y = cell_size.y * index[1] as f64;
                let z = cell_size.z * index[2] as f64;

                let mins = bound.mins() + Vector3::new(x, y, z);
                let maxs = mins + cell_size;
                let cell_bound = Aabb::new(mins, maxs);

                let p = cell_bound.centre();

                let mat = inters
                    .observe_mat(surfs, bound, &gen_mat_ray(&p))
                    .expect("Unable to observe material.");

                cells.push(Cell::new(Aabb::new(mins, maxs), mat, inters, surfs));
            }
            cell_blocks.push((start as usize, cells));
        }

        cell_blocks
    }

    /// Get the resolution of the grid.
    #[inline]
    #[must_use]
    pub fn res(&self) -> [usize; 3] {
        let res = self.cells.shape();

        [
            *res.get(X as usize).expect("Missing resolution index."),
            *res.get(Y as usize).expect("Missing resolution index."),
            *res.get(Z as usize).expect("Missing resolution index."),
        ]
    }

    /// Create a map of the material keys.
    #[inline]
    #[must_use]
    pub fn mat_keys(&self) -> Array3<&MatKey> {
        self.cells.map(Cell::mat)
    }

    /// Create a set of material maps.
    #[inline]
    #[must_use]
    pub fn mat_maps(&self, mats: &MatSet) -> Set<MatKey, Array3<f64>> {
        let mut set = BTreeMap::new();

        let keys = self.mat_keys();
        for key in mats.map().keys() {
            set.insert(key.clone(), keys.map(|k| if k == &key { 1.0 } else { 0.0 }));
        }

        Set::new(set)
    }

    /// Determine the number of intersecting interfaces in each cell.
    #[inline]
    #[must_use]
    pub fn interfaces(&self) -> Array3<f64> {
        self.cells().map(|c| c.inter_tris().len() as f64)
    }

    /// Determine a suitable bump distance for the grid.
    #[inline]
    #[must_use]
    pub fn bump_dist(&self) -> f64 {
        let mins: Vec<f64> = self
            .bound
            .widths()
            .iter()
            .zip(self.cells.shape())
            .map(|(dx, r)| *dx / *r as f64 * BUMP_SCALE)
            .collect();

        list::min(&mins)
    }
}
