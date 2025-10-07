use bevy::{
    asset::RenderAssetUsages, dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*,
    render::render_resource::Extent3d,
};

use libnoise::prelude::*;

use crate::{
    generators::generate,
    layer::{Layer, LayerType},
    ui::{UIPlugin, settings::SettingChanged},
};

pub mod generators;
pub mod layer;
pub mod ui;

#[derive(Resource)]
struct ActiveLayers(Vec<Layer>);

#[derive(Resource)]
struct SelectedLayer(usize);

#[derive(Resource)]
struct HMImageHandle(Handle<Image>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin)
        // .add_plugins(FpsOverlayPlugin { ..default() })
        .insert_resource(ActiveLayers(Vec::new()))
        .add_systems(Startup, setup)
        .add_systems(Update, add)
        .add_systems(Update, process)
        .add_observer(on_layer_setting_change)
        .run();
}

fn on_layer_setting_change(
    change: On<SettingChanged<LayerType>>,
    mut layers: ResMut<ActiveLayers>,
    selected: Res<SelectedLayer>,
) {
    layers.0[selected.0].layer_type = change.value.clone();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    let image = Image::new(
        Extent3d {
            width: 1024,
            height: 1024,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        vec![0; 1024 * 1024 * 4],
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    let handle = images.add(image);
    commands.insert_resource(HMImageHandle(handle.clone()));
}

fn add(keys: Res<ButtonInput<KeyCode>>, mut layers: ResMut<ActiveLayers>) {
    if keys.just_pressed(KeyCode::Space) {
        layers
            .0
            .push(Layer::new(layer::LayerType::Hills, rand::random::<u64>()));
    }
}

fn process(
    layers: Res<ActiveLayers>,
    mut images: ResMut<Assets<Image>>,
    handle: Res<HMImageHandle>,
) {
    if !layers.is_changed() {
        return;
    }

    // TODO: this is slow -.-
    let mut base: Vec<f64> = vec![0.0; 1024 * 1024];
    for layer in layers.0.clone() {
        let buffer = generate(layer);
        base = base.iter().zip(buffer).map(|(a, b)| a + b).collect();
    }

    // Update image data after processing layers
    if let Some(image) = images.get_mut(&handle.0) {
        let mut data: Vec<u8> = Vec::with_capacity(base.len() * 4);
        for v in base {
            let n = ((v + 1.0) * 0.5).clamp(0.0, 1.0);
            let b = (n * 255.0) as u8;
            data.extend_from_slice(&[b, b, b, 255]);
        }
        image.data = Some(data);
    }
}
