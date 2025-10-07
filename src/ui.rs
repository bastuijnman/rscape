use bevy::prelude::*;

use crate::{
    ActiveLayers, HMImageHandle, SelectedLayer,
    ui::{
        dropdown::DropdownUiPlugin,
        layer::layer,
        panel::{PanelProperties, panel},
        settings::{SettingChanged, settings},
    },
};

pub mod dropdown;
pub mod layer;
pub mod panel;
pub mod settings;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DropdownUiPlugin)
            .add_systems(Startup, ui)
            .add_systems(
                Update,
                render_layers_panel.run_if(
                    resource_changed::<ActiveLayers>
                        .or(resource_exists_and_changed::<SelectedLayer>),
                ),
            )
            .add_systems(Update, render_preview_panel);
    }
}

#[derive(Component)]
struct LayersPanel;

#[derive(Component)]
struct PreviewPanel;

// Renders the root node and the main panels
fn ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            column_gap: px(20),
            row_gap: px(20),
            padding: UiRect::all(px(25)),
            ..default()
        },
        children![
            panel(
                PanelProperties {
                    title: "Layers".to_string(),
                    width: 25,
                },
                (),
                LayersPanel,
            ),
            panel(
                PanelProperties {
                    title: "Preview".to_string(),
                    width: 75,
                },
                (),
                PreviewPanel,
            )
        ],
    ));
}

fn render_layers_panel(
    mut commands: Commands,
    layers: Res<ActiveLayers>,
    selected: Option<Res<SelectedLayer>>,
    panel: Single<Entity, With<LayersPanel>>,
) {
    commands.entity(*panel).despawn_children();
    commands.entity(*panel).with_children(|parent| {
        for (i, l) in layers.0.iter().enumerate() {
            let active = if let Some(ref selected) = selected
                && selected.0 == i
            {
                true
            } else {
                false
            };
            parent.spawn(layer(l.clone(), active)).observe(
                move |_t: On<Pointer<Click>>, mut c: Commands| c.insert_resource(SelectedLayer(i)),
            );
        }

        if let Some(selected) = selected {
            parent.spawn((
                Node {
                    margin: UiRect::vertical(px(15)),
                    border: UiRect::top(px(1)),
                    padding: UiRect::horizontal(px(5)),
                    ..default()
                },
                children![(
                    Node::default(),
                    Text::new("Settings"),
                    TextFont::default().with_font_size(14.0),
                )],
            ));
            parent.spawn(settings(layers.0[selected.0].clone()));
        }
    });
}

fn render_preview_panel(
    mut commands: Commands,
    image: Res<HMImageHandle>,
    panel: Single<Entity, With<PreviewPanel>>,
) {
    if !image.is_changed() {
        return;
    }

    commands.entity(*panel).despawn_children();
    commands.entity(*panel).with_children(|parent| {
        parent.spawn((
            Node { ..default() },
            ImageNode {
                image: image.0.clone(),
                ..default()
            },
        ));
    });
}
