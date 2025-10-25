use std::f32::consts::PI;

use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::Extent3d};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy_heightmap::HeightMap;

use crate::{
    generators::generate,
    layer::{AddLayer, Layer, LayerType},
    processors::process,
    ui::ui,
};

pub mod generators;
pub mod layer;
pub mod processors;
pub mod ui;

#[derive(Resource)]
pub struct ActiveLayers(Vec<Layer>);

#[derive(Resource)]
pub struct HMImageHandle(Handle<Image>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .insert_resource(ActiveLayers(Vec::new()))
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, ui)
        .add_systems(Update, update_3d_preview)
        .add_systems(Update, handle_layer_change)
        .add_observer(on_add_layer)
        .run();
}

fn on_add_layer(_: On<AddLayer>, mut layers: ResMut<ActiveLayers>) {
    layers
        .0
        .push(Layer::new(layer::LayerType::Hills, rand::random::<u64>()));
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: PI / 4.,
            near: 0.1,
            far: 2000.,
            ..default()
        }),
        Transform::from_xyz(0.0, -750.0, 750.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 4500.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 750.0)
            .with_rotation(Quat::from_axis_angle(Vec3::ONE, -PI / 6.)),
    ));

    let image = Image::new(
        Extent3d {
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        vec![0; 512 * 512 * 4],
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    let handle = images.add(image);
    commands.insert_resource(HMImageHandle(handle.clone()));
}

#[derive(Component)]
struct Preview3D;

fn update_3d_preview(
    layers: ResMut<ActiveLayers>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<Preview3D>>,
) {
    if !layers.is_changed() {
        return;
    }

    for entity in query {
        commands.entity(entity).despawn();
    }

    let data = generate_layers_result(layers.0.clone());
    let map = Map { data };
    let mesh = map.build_mesh(UVec2 { x: 512, y: 512 });
    let handle = meshes.add(mesh);

    commands.spawn((
        Preview3D,
        Mesh3d(handle),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform {
            scale: Vec2::splat(512.0).extend(100.0),
            ..default()
        },
    ));
}

struct Map {
    data: Vec<f64>,
}
impl HeightMap for Map {
    fn h(&self, p: Vec2) -> f32 {
        let x = (((p.x + 0.5) * 512.0) as usize).clamp(0, 511);
        let y = (((p.y + 0.5) * 512.0) as usize).clamp(0, 511);
        self.data[(y * 512) + x] as f32
    }
}

// base input vec is usually between -1 and 1, needs to be normalized to 0..1 for heightmap
fn normalize(base: Vec<f64>) -> Vec<f64> {
    base.iter()
        .map(|v| ((v + 1.0) * 0.5).clamp(0.0, 1.0))
        .collect()
}

fn generate_layers_result(layers: Vec<Layer>) -> Vec<f64> {
    let mut base: Vec<f64> = vec![0.0; 512 * 512];
    for (i, layer) in layers.iter().enumerate() {
        let buffer = match layer.layer_type {
            LayerType::Clamp => process(base.clone(), layer.clone()),
            LayerType::SimpleErosion => process(base.clone(), layer.clone()),
            _ => normalize(generate(layer.clone())),
        };

        // Blending
        if layer.is_blendable() {
            base = base
                .iter()
                .zip(buffer)
                .map(|(a, b)| a + b)
                .map(|a| a * if i > 1 { 0.5 } else { 1.0 })
                .collect();
        } else {
            base = buffer;
        }
    }
    base
}

fn handle_layer_change(
    layers: Res<ActiveLayers>,
    mut images: ResMut<Assets<Image>>,
    handle: Res<HMImageHandle>,
) {
    if !layers.is_changed() {
        return;
    }
    let base = generate_layers_result(layers.0.clone());

    // Update image data after processing layers
    if let Some(image) = images.get_mut(&handle.0) {
        let mut data: Vec<u8> = Vec::with_capacity(base.len() * 4);
        for v in base {
            let b = (v * 255.0) as u8;
            data.extend_from_slice(&[b, b, b, 255]);
        }
        image.data = Some(data);
    }
}
