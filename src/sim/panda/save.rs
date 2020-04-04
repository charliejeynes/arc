//! Save functions.

use ::slice_of_array::prelude::*;
use log::info;
use ndarray::Array2;
use palette::{LinSrgba, Pixel, Srgba};
use png::{BitDepth, ColorType, Encoder};
use std::{fs::File, io::BufWriter, path::Path};

/// Save an array as a png image.
#[inline]
pub fn png(in_dir: &Path, name: &str, img: Array2<LinSrgba>) {
    info!("Saving camera image: {}", name);

    info!("Transforming image");
    let mut data = Array2::from_elem(
        (img.shape()[1], img.shape()[0]),
        Srgba::new(0.0, 0.0, 0.0, 1.0).into_linear(),
    );
    for xi in 0..img.shape()[1] {
        for yi in 0..img.shape()[0] {
            data[[img.shape()[1] - xi - 1, yi]] = img[[yi, xi]];
        }
    }
    let data = data.t();

    let path = &in_dir.join(format!("{}.png", name));

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = Encoder::new(w, data.shape()[0] as u32, data.shape()[1] as u32);
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .expect("Could not build image writer.");

    let data: Vec<[u8; 4]> = data
        .mapv(|col| Srgba::from_linear(col).into_format().into_raw())
        .into_raw_vec();
    writer
        .write_image_data(data.flat())
        .expect("Failed to save png.");

    info!("Image saved at: {}\n", path.display());
}
