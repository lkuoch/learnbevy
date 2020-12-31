use crate::{
    player::{components::*, state::*, vars::mask_dude as MaskDude},
    plugins::input::event::InputEvent,
};
use bevy::prelude::*;
use std::collections::HashMap;

pub(super) fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut textures_map = HashMap::<MaskDude::States, Handle<TextureAtlas>>::new();

    for texture in MaskDude::TEXTURES {
        let texture_handle = asset_server.load(texture.path);

        let handle = texture_atlases.add(TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(32.0, 32.0),
            texture.frames,
            1,
        ));

        textures_map.insert(texture.state, handle);
    }

    if let Some(default_texture) = textures_map.get(&MaskDude::States::Idle) {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: default_texture.clone(),
                transform: Transform::from_scale(Vec3::splat(2.5)),
                ..Default::default()
            })
            .with(Timer::from_seconds(0.1, true))
            .with(Player {
                textures: textures_map,
            });
    }
}

pub(super) fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

pub(super) fn handle_input_event(
    mut event_reader: Local<EventReader<InputEvent>>,
    events: ResMut<Events<InputEvent>>,
    mut player_state: ResMut<PlayerState>,
) {
    for event in event_reader.iter(&events) {
        if let Some(key) = event.pressed {
            match key {
                KeyCode::A => {
                    player_state.move_left();
                }
                KeyCode::D => {
                    player_state.move_right();
                }
                _ => {}
            }
        }

        if let Some(key) = event.released {
            match key {
                KeyCode::A => {
                    player_state.reset_movement();
                }
                KeyCode::D => {
                    player_state.reset_movement();
                }
                _ => {}
            }
        }
    }
}

pub(super) fn react_player_state(player_state: Res<PlayerState>) {
    let current = player_state;

    println!("{current:?}");
}
