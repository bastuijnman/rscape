use crate::layer::{Layer, LayerType, settings::LayerSettings};

pub fn process(input: Vec<f64>, layer: Layer) -> Vec<f64> {
    match layer.layer_type {
        LayerType::Clamp => process_clamp(input, layer),
        LayerType::SimpleErosion => process_simple_erosion(input, layer),

        // In case this is not a supported processor we just return the input
        _ => input,
    }
}

fn process_clamp(input: Vec<f64>, layer: Layer) -> Vec<f64> {
    let LayerSettings::Clamp(settings) = layer.settings else {
        panic!("Incorrect settings being used in clamp processing")
    };

    input
        .iter()
        .map(|v| v.clamp(settings.clamp[0], settings.clamp[1]))
        .collect()
}

fn process_simple_erosion(input: Vec<f64>, layer: Layer) -> Vec<f64> {
    let LayerSettings::SimpleErosion(settings) = layer.settings else {
        panic!("Incorrect settings being used in simple eriosion processing");
    };

    let iterations = settings.iterations;
    let mut eroded = input.clone();
    for _ in 0..iterations {
        for index in 0..eroded.len() {
            let value = eroded[index];
            let neighbours = get_neighbours(eroded.len().isqrt(), index);
            for neighbour in neighbours.iter() {
                let delta = value - eroded[*neighbour];
                if delta > settings.max_angle {
                    let change = (delta - settings.max_angle) * settings.erosion_factor;
                    eroded[index] -= change;
                    eroded[*neighbour] += change;
                }
            }
        }
    }

    eroded
}

fn get_neighbours(size: usize, index: usize) -> Vec<usize> {
    let row = index / size;
    let col = index % size;

    let mut neighbours = Vec::new();

    // Loop over all surrounding deltas (-1, 0, 1)
    for dr in [-1isize, 0, 1] {
        for dc in [-1isize, 0, 1] {
            if dr == 0 && dc == 0 {
                continue; // skip self
            }

            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            // Ensure neighbour is within bounds
            if new_row >= 0 && new_row < size as isize && new_col >= 0 && new_col < size as isize {
                neighbours.push((new_row as usize) * size + (new_col as usize));
            }
        }
    }

    neighbours
}
