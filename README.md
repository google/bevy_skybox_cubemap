[![Crates.io](https://img.shields.io/crates/v/bevy_skybox_cubemap.svg)](https://crates.io/crates/bevy_skybox_cubemap)
[![Docs.rs](https://img.shields.io/docsrs/bevy_skybox_cubemap)](https://docs.rs/bevy_skybox_cubemap)
[![Workflow Status](https://github.com/google/bevy_skybox_cubemap/workflows/Rust/badge.svg)](https://github.com/google/bevy_skybox_cubemap/actions?query=workflow%3A%22Rust%22)
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

# **\[DEPRECATED\]** Bevy Skybox Cubemap

Bevy 0.11 provides native support for this and this crate has not been updated since Bevy 0.5 and will not work with any more recent version of Bevy.

Provides cubemap-based skyboxes for Bevy.

## Overview

This crate provides a material type, [`SkyboxMaterial`], and bundle, [`SkyboxBundle`], which
make it easy to add a skybox to a scene. Skyboxes are implemented as normal entities using a
special shader to ensure they always appear around the camera and behind all other objects in
the scene.

## Basic usage

```rust
// Install the skybox plugin:
App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(SkyboxPlugin)
    .add_startup_system(setup.system());

// Configure the skybox.
fn setup(
   mut commands: Commands,
   asset_server: Res<AssetServer>,
   mut skyboxes: ResMut<Assets<SkyboxMaterial>>,
   mut skybox_conversion: ResMut<SkyboxTextureConversion>,
) {
    // Load a texture to use as the skybox.
    let skybox_texture = asset_server.load("labeled_skybox.png");
    // Convert a flat image the 6 faces above one another into a 6-layer array texture that's
    // appropriate for skybox use.
    skybox_conversion.make_array(skybox_texture.clone());
    // Spawn a skybox entity.
    commands.spawn_bundle(SkyboxBundle::new(
        skyboxes.add(SkyboxMaterial::from_texture(skybox_texture)),
    ));
}
```

See below for details on the required texture format.

Skyboxes are more or less normal entities. Normal Bevy features like render layers and render
pass selection should work on them, so it should be possible to have different skyboxes in
different cameras using render layers.

The skybox is implemented almost entirely in shader code, so aside from the initial texture
conversion (which you can do yourself if you prefer), there's no need for additional cameras or
marker components or complicated transform setup on the camera or skybox. The shader will ensure
that the skybox is always drawn behind all other entities and that the position of both the
camera and skybox have no effect.

In case you want your skybox to have a different orientation, the rotation compoenent of the skybox's
transform *is* respected.

## Texture Layout

In order to use a Skybox, you need a properly formatted Skybox texture. Appropriate textures for
`SkyboxMaterial` should have 6 identically sized square layers which make up the 6 faces of the
Skybox. A helper is provided to convert a single-layer `N x 6N` image into a 6 layer image
appropriate for a skybox.

This is the net of the cube that the orientation of the faces is based on. It is *not* the
texture layout that is actually used for rendering.

<img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/expected_net.png" />

|           | Top (+Y)    |            |           |
|-----------|-------------|------------|-----------|
| Left (-X) | Front (-Z)  | Right (+X) | Back (+Z) |
|           | Bottom (-Y) |            |           |

For rendering, the faces are used as separate layers of an array texture in this order:

* Right (+X)
* Left (-X)
* Top (+Y)
* Bottom (-Y)
* Back (+Z)
* Front (-Z)

Currently the easiest way to create an image with the appropriate layers is to rearrange the
sections of the cube net into a single vertical image in the required order, then when you load
the image, send it to [`SkyboxTextureConversion`], which will use
[`Texture::reinterpret_stacked_2d_as_array`] to convert it to a 6 layer array once it is loaded.

Here is the above net rearranged into the correct order for a skybox texture:

<img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/array_format.png" />

When converting from a net or a collection of images representing the faces of the skybox, pay
attention to their orientation relative to the canonical net above. If you have a net with a
differnt face connected to the top and bottom, the easiest thing to do is to simply interpret
whatever face matches the top and bottom as the "front" when rearranging the faces into the
vertical array format.

For example, if the top and bottom branch off of the third square instead of the second, you
could interpret the net this way:

<img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/shifted_net.png" />

You would then rearrange from this net to the same vertical layout as above.

Alternately, if want a specific face to be used as the "front" and that face isn't the one that
matches the orientation of the top and bottom, you could instead rotate the top and bottom when
building the stacked array texture. However, since you can also rotate the skybox using the
skybox entity's transform, that's probably not necessary.

## Maintenance of this Crate

Bevy is a cool project and I am excited for it to succeed. However, I don't necessarily have
time to always keep this crate up to date with the latest versions of Bevy, especially if it
gets relatively low usage.

That said, I *will* respond to pull requests and will release new versions based on pull
requests to update to newer versions of Bevy. Creating a pull request is preferable to opening
an issue asking me to update, because I can more easily spare the time to merge a pull request
than to do all the necessary updates myself, but if I do get issues asking me to update to new
versions of Bevy, I will respond to them on a best-effort basis.

I will create releases targeting the latest published version of Bevy, not `main`. If you are
working on main, and need to modify this crate to work with the latest `HEAD`, I recommend
forking and then sending me a pull request once Bevy publishes an updated version.

In terms of features, I would consider this project largely feature-complete as-is. Texture
packaging seems to be outside the scope of features supported by Bevy, so I'm not going to add
tools to automate building textures for skyboxes. This crate also isn't intended to support any
kind of dynamic skyboxes, so there doesn't seem to be much more that needs to be done besides
keeping up with latest versions of Bevy. However, if you have any ideas for new features or API
changes, I'm happy to hear them.

## Disclaimer

This is not an officially supported Google product.

License: Apache-2.0
