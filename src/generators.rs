use crate::layer::Layer;
use libnoise::prelude::*;

pub fn generate(layer: Layer) -> Vec<f64> {
    match layer.layer_type {
        _ => generate_hills(layer),
    }
}

fn generate_hills(layer: Layer) -> Vec<f64> {
    let generator = Source::simplex(layer.seed).fbm(5, 0.013, 2.0, 0.5);
    NoiseBuffer::<2>::new([1024, 1024], &generator).buffer
}
