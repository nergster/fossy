/************************************************************************
 *  Fossy - A FOSS video game experiment.
 *  Copyright (C) 2023  Nergster
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 ************************************************************************/

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(player_movement)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("textures/icon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
    ));
}

fn player_movement(
    time: Res<Time>,
    mut player: Query<(&Player, &mut Transform)>,
    input: Res<Input<KeyCode>>,
) {
    for (_player, mut transform) in &mut player {
        for key_code in input.get_pressed() {
            match *key_code {
                KeyCode::Right => transform.translation.x += 100. * time.delta_seconds(),
                KeyCode::Left => transform.translation.x -= 100. * time.delta_seconds(),
                KeyCode::Up => transform.translation.y += 100. * time.delta_seconds(),
                KeyCode::Down => transform.translation.y -= 100. * time.delta_seconds(),
                _ => (),
            }
        }
    }
}
