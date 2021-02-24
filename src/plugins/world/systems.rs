use crate::world::{components::*, vars::*};
use bevy::prelude::*;

pub(super) fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    if let Ok(wall_color) = Color::hex(world::WALL_COLOR_1) {
        let wall_material = materials.add(wall_color.into());
        let bounds = Vec2::new(1000.0, 700.0);

        commands
            // left
            .spawn(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    world::WALL_THICKNESS,
                    bounds.y + world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .with(Collider::Wall)
            // right
            .spawn(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    world::WALL_THICKNESS,
                    bounds.y + world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .with(Collider::Wall)
            // bottom
            .spawn(SpriteBundle {
                material: wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    bounds.x + world::WALL_THICKNESS,
                    world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .with(Collider::Ground)
            // top
            .spawn(SpriteBundle {
                material: wall_material,
                transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
                sprite: Sprite::new(Vec2::new(
                    bounds.x + world::WALL_THICKNESS,
                    world::WALL_THICKNESS,
                )),
                ..Default::default()
            })
            .with(Collider::Ceiling);
    }
}
