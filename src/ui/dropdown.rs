use bevy::prelude::*;

use crate::{layer::LayerSettingField, ui::settings::SettingChanged};

#[derive(Component)]
pub struct Dropdown {
    pub field: LayerSettingField,
    pub options: Vec<String>,
    pub value: String,
}

#[derive(Component)]
struct DropdownPortal;

pub struct DropdownUiPlugin;
impl Plugin for DropdownUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, register_dropdown);
    }
}

fn register_dropdown(mut commands: Commands, query: Query<(Entity, &Dropdown), Added<Dropdown>>) {
    for (entity, dropdown) in query {
        let options = dropdown.options.clone();
        commands.entity(entity).observe(
            move |trigger: On<Pointer<Click>>, mut commands: Commands| {
                // Add dropdown portal
                let portal = commands
                    .spawn((
                        DropdownPortal,
                        Node {
                            position_type: PositionType::Absolute,
                            border: UiRect::all(px(1)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(Color::BLACK),
                        BorderColor::all(Color::WHITE),
                        ChildOf(trigger.entity),
                    ))
                    .id();

                // Render options and handle click
                for option in options.clone() {
                    commands
                        .spawn(dropdown_item(option.clone(), portal))
                        .observe(
                            move |mut trigger: On<Pointer<Click>>, mut commands: Commands| {
                                trigger.propagate(false);
                                commands.trigger(SettingChanged::<String> {
                                    field: LayerSettingField::LayerType,
                                    value: option.clone(),
                                });
                                commands.entity(portal).despawn();
                            },
                        );
                }
            },
        );
    }
}

pub fn dropdown(component: Dropdown) -> impl Bundle {
    let value = component.value.clone();
    (
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(5)),
            ..default()
        },
        children![
            (
                Node { ..default() },
                Text::new("Field Title"),
                TextFont::default().with_font_size(12.0),
            ),
            (
                component,
                Node {
                    border: UiRect::all(px(1)),
                    padding: UiRect::all(px(5)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                BorderRadius::all(px(5)),
                children![(Text::new(value), TextFont::default().with_font_size(12.0),)],
            )
        ],
    )
}

fn dropdown_item(value: String, parent: Entity) -> impl Bundle {
    (
        Node {
            padding: UiRect::all(px(5)),
            ..default()
        },
        ChildOf(parent),
        children![(Text::new(value), TextFont::default().with_font_size(12.0),)],
    )
}
