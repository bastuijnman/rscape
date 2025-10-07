use bevy::prelude::*;

#[derive(Component)]
pub struct PanelContainer;

fn panel_title(title: String) -> impl Bundle {
    (
        Node {
            width: percent(100),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        children![(
            Text::new(title),
            TextColor(Color::WHITE),
            TextFont::default().with_font_size(14.0)
        )],
    )
}

pub struct PanelProperties {
    pub title: String,
    pub width: usize,
}

pub fn panel(
    properties: PanelProperties,
    children: impl Bundle,
    component: impl Component,
) -> impl Bundle {
    (
        Node {
            width: percent(properties.width),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(px(1)),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        children![
            panel_title(properties.title.clone()),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                PanelContainer,
                component,
                children
            )
        ],
    )
}
