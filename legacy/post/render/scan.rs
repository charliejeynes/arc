//! Core scanning function.

use crate::{
    geom::Ray,
    sim::render::{Camera, Cell, Settings, Tracer},
    util::ParProgressBar,
};
use log::warn;
use nalgebra::{Unit, Vector3};
use ndarray::Array2;
use std::sync::{Arc, Mutex};

/// Render using a single thread.
#[allow(clippy::never_loop)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn run_thread(
    _id: usize,
    cam: &Camera,
    grid: &Cell,
    pb: &Arc<Mutex<ParProgressBar>>,
    block_size: usize,
    sett: &Settings,
) -> Vec<Array2<f64>> {
    let mut scene = Array2::zeros(cam.res());
    let mut sky = Array2::zeros(cam.res());
    let mut lights = Array2::zeros(cam.res());

    let mut layer_0 = Array2::zeros(cam.res());
    let mut layer_1 = Array2::zeros(cam.res());
    let mut layer_2 = Array2::zeros(cam.res());
    let mut layer_3 = Array2::zeros(cam.res());
    let mut layer_4 = Array2::zeros(cam.res());
    let mut layer_5 = Array2::zeros(cam.res());
    let mut layer_6 = Array2::zeros(cam.res());
    let mut layer_7 = Array2::zeros(cam.res());
    let mut layer_8 = Array2::zeros(cam.res());
    let mut layer_9 = Array2::zeros(cam.res());
    let mut layer_10 = Array2::zeros(cam.res());
    let mut layer_11 = Array2::zeros(cam.res());
    let mut layer_12 = Array2::zeros(cam.res());
    let mut layer_13 = Array2::zeros(cam.res());

    let super_samples = cam.ss_power().pow(2);

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size as u64);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n as usize % cam.res().0;
            let yi = n as usize / cam.res().0;

            for n in 0..super_samples {
                let mut tracer = Tracer::new(cam.gen_ss_ray(xi, yi, n));

                loop {
                    if let Some((new_tracer, _dist, norm, group)) = grid.observe(tracer) {
                        tracer = new_tracer;

                        match group {
                            -3 => {
                                let amb = ambient(sett);
                                let diff = diffuse(&tracer, &norm, sett);
                                let spec = specular(cam, &tracer, &norm, sett);
                                let shadow = shadow(grid, tracer.clone(), &norm, sett);
                                let ll = lamp_light(grid, tracer.clone(), &norm, sett);

                                let contribution = (amb + diff + spec)
                                    * (1.0 - shadow)
                                        .mul_add(sett.shadow_weight(), ll * sett.light_weight());

                                *scene.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                    contribution * 0.1;
                                *lights.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                    contribution * 0.1;
                                break;
                            }
                            -4 => {
                                tracer.ray_mut().refract(&norm, 1.0, 1.1);
                                tracer.travel(1.0e-6);
                                // break;
                            }
                            -1 => {
                                tracer.ray_mut().reflect(&norm);
                                tracer.travel(1.0e-6);
                            }
                            -2 => {
                                tracer.ray_mut().reflect(&norm);

                                let theta = ((tracer.ray().pos().x * 6.0).sin().powi(2)
                                    * (tracer.ray().pos().y * 6.0).sin().powi(2))
                                    * 0.5e-1;
                                let rot = nalgebra::Rotation3::from_axis_angle(
                                    &nalgebra::Vector3::y_axis(),
                                    theta,
                                );
                                *tracer.ray_mut().dir_mut() =
                                    Unit::new_normalize(rot * tracer.ray().dir().as_ref());
                                tracer.travel(1.0e-6);
                            }
                            0..=20 => {
                                let amb = ambient(sett);
                                let diff = diffuse(&tracer, &norm, sett);
                                let spec = specular(cam, &tracer, &norm, sett);
                                let shadow = shadow(grid, tracer.clone(), &norm, sett);
                                let ll = lamp_light(grid, tracer.clone(), &norm, sett);

                                let contribution = (amb + diff + spec)
                                    * (1.0 - shadow)
                                        .mul_add(sett.shadow_weight(), ll * sett.light_weight());

                                *match group {
                                    0 => &mut layer_0,
                                    1 => &mut layer_1,
                                    2 => &mut layer_2,
                                    3 => &mut layer_3,
                                    4 => &mut layer_4,
                                    5 => &mut layer_5,
                                    6 => &mut layer_6,
                                    7 => &mut layer_7,
                                    8 => &mut layer_8,
                                    9 => &mut layer_9,
                                    10 => &mut layer_10,
                                    11 => &mut layer_11,
                                    12 => &mut layer_12,
                                    13 => &mut layer_13,
                                    _ => {
                                        warn!("Do not know how to handle drawing group {}.", group);
                                        break;
                                    }
                                }
                                .get_mut((xi, yi))
                                .expect("Invalid pixel index.") += contribution;

                                *scene.get_mut((xi, yi)).expect("Invalid pixel index.") +=
                                    contribution;

                                break;
                            }
                            _ => {
                                warn!("Do not know how to handle group {}.", group);
                                break;
                            }
                        }
                    } else {
                        *sky.get_mut((xi, yi)).expect("Invalid pixel index.") += 1.0;
                        *scene.get_mut((xi, yi)).expect("Invalid pixel index.") -= 1.0;
                        break;
                    }
                }
            }
        }
    }

    vec![
        layer_0, layer_1, layer_2, layer_3, layer_4, layer_5, layer_6, layer_7, layer_8, layer_9,
        layer_10, layer_11, layer_12, layer_13, lights, sky, scene,
    ]
}

/// Calculate the ambient lighting coefficient.
#[inline]
#[must_use]
fn ambient(sett: &Settings) -> f64 {
    sett.ambient()
}

/// Calculate the diffuse lighting coefficient.
#[inline]
#[must_use]
fn diffuse(tracer: &Tracer, norm: &Unit<Vector3<f64>>, sett: &Settings) -> f64 {
    let light_dir = Unit::new_normalize(sett.sun_pos() - tracer.ray().pos());

    sett.diffuse() * norm.dot(&light_dir).abs()
}

/// Calculate the specular lighting coefficient.
#[inline]
#[must_use]
fn specular(cam: &Camera, tracer: &Tracer, norm: &Unit<Vector3<f64>>, sett: &Settings) -> f64 {
    let light_dir = Unit::new_normalize(sett.sun_pos() - tracer.ray().pos());
    let view_dir = Unit::new_normalize(cam.forward().pos() - tracer.ray().pos());

    let ref_dir = reflect(&-light_dir, norm);

    sett.specular() * view_dir.dot(&ref_dir).max(0.0).powi(sett.specular_pow())
}

/// Calculate the shadowing factor.
#[inline]
#[must_use]
fn shadow(grid: &Cell, mut tracer: Tracer, norm: &Unit<Vector3<f64>>, sett: &Settings) -> f64 {
    *tracer.ray_mut().dir_mut() = *norm;
    tracer.travel(1.0e-3);

    *tracer.ray_mut().dir_mut() = Unit::new_normalize(sett.sun_pos() - tracer.ray().pos());

    let mut light = 1.0;

    while let Some((new_tracer, _dist, _norm, group)) = grid.observe(tracer) {
        tracer = new_tracer;

        match group {
            -4 => {
                tracer.travel(1.0e-3);
                continue;
            }
            0 | 10 | 11 | 12 | 13 => {
                light *= 1.0 - sett.transparency();
            }
            -3..=20 => {
                return sett.shadow();
            }
            _ => {
                warn!("Do not know how to handle shadowing group {}.", group);
                return 0.0;
            }
        }

        tracer.travel(1.0e-3);
    }

    sett.shadow() * (1.0 - light)
}

/// Calculate the lamp lighting factor.
#[inline]
#[must_use]
fn lamp_light(grid: &Cell, mut tracer: Tracer, norm: &Unit<Vector3<f64>>, sett: &Settings) -> f64 {
    let mut light = 0.0;

    *tracer.ray_mut().dir_mut() = *norm;
    tracer.travel(1.0e-3);

    for lamp_pos in sett.lamps() {
        let light_dir = Unit::new_normalize(lamp_pos - tracer.ray().pos());
        let mut light_tracer = Tracer::new(Ray::new(*tracer.ray().pos(), light_dir));

        while let Some((new_tracer, _dist, _norm, group)) = grid.observe(light_tracer) {
            light_tracer = new_tracer;

            match group {
                0 | 10 | 11 | 12 | 13 => {
                    light *= 1.0 - sett.transparency();
                    light_tracer.travel(1.0e-3);
                }
                -3 => {
                    let dist = light_tracer.dist_travelled();
                    light += 1.0
                        / sett.lamp_quadratic().mul_add(
                            dist.powi(2),
                            sett.lamp_const() + (sett.lamp_linear() * dist),
                        );
                    break;
                }
                -4 => {
                    light_tracer.ray_mut().refract(&norm, 1.1, 1.0);
                    light_tracer.travel(1.0e-3);
                }
                _ => {
                    break;
                }
            }
        }
    }

    light
}

/// Calculate the reflection vector for a given input unit vector and surface normal.
#[inline]
#[must_use]
fn reflect(inc: &Unit<Vector3<f64>>, norm: &Unit<Vector3<f64>>) -> Unit<Vector3<f64>> {
    Unit::new_normalize(inc.as_ref() + (2.0 * (-inc.dot(norm)) * norm.as_ref()))
}
