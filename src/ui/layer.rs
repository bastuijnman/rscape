use bevy::prelude::*;

use crate::layer::Layer;

pub fn layer(layer: Layer, is_active: bool) -> impl Bundle {
    (
        Node {
            width: percent(100),
            padding: UiRect::all(px(5)),
            ..default()
        },
        BackgroundColor(if is_active {
            Color::srgba_u8(20, 71, 230, 255)
        } else {
            Color::srgba(0.0, 0.0, 0.0, 0.0)
        }),
        children![(
            Node { ..default() },
            Text::new(layer.layer_type.to_string()),
            TextColor::WHITE,
            TextFont::default().with_font_size(12.0)
        )],
    )
}
