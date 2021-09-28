// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::camera::{Camera, PerspectiveProjection};
use bevy_skybox_cubemap::{SkyboxBundle, SkyboxMaterial, SkyboxPlugin, SkyboxTextureConversion};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::PINK))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(SkyboxPlugin)
        .add_startup_system(setup.system())
        .add_system(spin_camera.system())
        .run();
}

fn spin_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Camera>, With<PerspectiveProjection>)>,
) {
    const SPEED: f32 = 0.5;
    const DIST: f32 = 20.0;

    const V_SPEED: f32 = 0.1;

    let t = time.seconds_since_startup() as f32;
    let (sin, cos) = (t * SPEED).sin_cos();
    let vcos = (t * V_SPEED).cos() * 0.9;
    let vsin = (1.0 - vcos * vcos).sqrt();

    for mut trans in query.iter_mut() {
        *trans = Transform::from_xyz(vsin * sin * DIST, vcos * DIST, vsin * cos * DIST)
            .looking_at(Vec3::ZERO, Vec3::Y);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut skyboxes: ResMut<Assets<SkyboxMaterial>>,
    mut skybox_conversion: ResMut<SkyboxTextureConversion>,
) {
    let skybox_texture = asset_server.load("labeled_skybox.png");
    // Skybox textures are stacked 2d images and need to be converted to a 2d texture array before
    // they can be sampled.
    skybox_conversion.make_array(skybox_texture.clone());

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // plane
    let sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.25,
        subdivisions: 4,
    }));
    // Positive X pointer.
    commands.spawn_bundle(PbrBundle {
        mesh: sphere.clone(),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(1.0, 0.5, 0.0),
        ..Default::default()
    });
    // Positive Y pointer.
    commands.spawn_bundle(PbrBundle {
        mesh: sphere.clone(),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..Default::default()
    });
    // Positive Z pointer.
    commands.spawn_bundle(PbrBundle {
        mesh: sphere,
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 1.0),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    // skybox
    commands.spawn_bundle(SkyboxBundle {
        material: skyboxes.add(SkyboxMaterial {
            texture: Some(skybox_texture),
            ..Default::default()
        }),
        ..Default::default()
    });
}
