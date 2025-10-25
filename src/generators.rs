use crate::layer::{Layer, LayerType, settings::LayerSettings};
use libnoise::prelude::*;

pub fn generate(layer: Layer) -> Vec<f64> {
    match layer.layer_type {
        LayerType::Mountain => generate_mountains(layer),
        _ => generate_hills(layer),
    }
}

fn generate_mountains(layer: Layer) -> Vec<f64> {
    let LayerSettings::Mountain(settings) = layer.settings else {
        return vec![0.0; 512];
    };

    let generator = Source::simplex(42)
        .ridgedmulti(6, 1.0, 2.0, 2.0)
        .scale([settings.scale, settings.scale]);
    let buf = NoiseBuffer::<2>::new([512, 512], &generator).buffer;

    buf
}

fn generate_hills(layer: Layer) -> Vec<f64> {
    let generator = Source::perlin(layer.seed).scale([0.0075, 0.0075]);
    NoiseBuffer::<2>::new([512, 512], &generator).buffer
}
