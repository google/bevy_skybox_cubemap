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

//! Provides cubemap-based skyboxes for Bevy.
//!
//! # Overview
//!
//! This crate provides a material type, [`SkyboxMaterial`], and bundle, [`SkyboxBundle`], which
//! make it easy to add a skybox to a scene. Skyboxes are implemented as normal entities using a
//! special shader to ensure they always appear around the camera and behind all other objects in
//! the scene.
//!
//! # Basic usage
//!
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy_skybox_cubemap::{SkyboxBundle, SkyboxMaterial, SkyboxPlugin, SkyboxTextureConversion};
//! // Install the skybox plugin:
//! App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(SkyboxPlugin)
//!     .add_startup_system(setup.system());
//!
//! // Configure the skybox.
//! fn setup(
//!    mut commands: Commands,
//!    asset_server: Res<AssetServer>,
//!    mut skyboxes: ResMut<Assets<SkyboxMaterial>>,
//!    mut skybox_conversion: ResMut<SkyboxTextureConversion>,
//! ) {
//!     // Load a texture to use as the skybox.
//!     let skybox_texture = asset_server.load("labeled_skybox.png");
//!     // Convert a flat image the 6 faces above one another into a 6-layer array texture that's
//!     // appropriate for skybox use.
//!     skybox_conversion.make_array(skybox_texture.clone());
//!     // Spawn a skybox entity.
//!     commands.spawn_bundle(SkyboxBundle::new(
//!         skyboxes.add(SkyboxMaterial::from_texture(skybox_texture)),
//!     ));
//! }
//! ```
//!
//! See below for details on the required texture format.
//!
//! Skyboxes are more or less normal entities. Normal Bevy features like render layers and render
//! pass selection should work on them, so it should be possible to have different skyboxes in
//! different cameras using render layers.
//!
//! The skybox is implemented almost entirely in shader code, so aside from the initial texture
//! conversion (which you can do yourself if you prefer), there's no need for additional cameras or
//! marker components or complicated transform setup on the camera or skybox. The shader will ensure
//! that the skybox is always drawn behind all other entities and that the position of both the
//! camera and skybox have no effect.
//!
//! In case you want your skybox to have a different orientation, the rotation compoenent of the skybox's
//! transform *is* respected.
//!
//! # Texture Layout
//!
//! In order to use a Skybox, you need a properly formatted Skybox texture. Appropriate textures for
//! `SkyboxMaterial` should have 6 identically sized square layers which make up the 6 faces of the
//! Skybox. A helper is provided to convert a single-layer `N x 6N` image into a 6 layer image
//! appropriate for a skybox.
//!
//! This is the net of the cube that the orientation of the faces is based on. It is *not* the
//! texture layout that is actually used for rendering.
//!
//! <img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/expected_net.png" />
//!
//! |           | Top (+Y)    |            |           |
//! |-----------|-------------|------------|-----------|
//! | Left (-X) | Front (-Z)  | Right (+X) | Back (+Z) |
//! |           | Bottom (-Y) |            |           |
//!
//! For rendering, the faces are used as separate layers of an array texture in this order:
//!
//! * Right (+X)
//! * Left (-X)
//! * Top (+Y)
//! * Bottom (-Y)
//! * Back (+Z)
//! * Front (-Z)
//!
//! Currently the easiest way to create an image with the appropriate layers is to rearrange the
//! sections of the cube net into a single vertical image in the required order, then when you load
//! the image, send it to [`SkyboxTextureConversion`], which will use
//! [`Texture::reinterpret_stacked_2d_as_array`] to convert it to a 6 layer array once it is loaded.
//!
//! Here is the above net rearranged into the correct order for a skybox texture:
//!
//! <img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/array_format.png" />
//!
//! When converting from a net or a collection of images representing the faces of the skybox, pay
//! attention to their orientation relative to the canonical net above. If you have a net with a
//! differnt face connected to the top and bottom, the easiest thing to do is to simply interpret
//! whatever face matches the top and bottom as the "front" when rearranging the faces into the
//! vertical array format.
//!
//! For example, if the top and bottom branch off of the third square instead of the second, you
//! could interpret the net this way:
//!
//! <img src="https://raw.githubusercontent.com/google/bevy_skybox_cubemap/main/docimgs/shifted_net.png" />
//!
//! You would then rearrange from this net to the same vertical layout as above.
//!
//! Alternately, if want a specific face to be used as the "front" and that face isn't the one that
//! matches the orientation of the top and bottom, you could instead rotate the top and bottom when
//! building the stacked array texture. However, since you can also rotate the skybox using the
//! skybox entity's transform, that's probably not necessary.
//!
//! # Maintenance of this Crate
//!
//! Bevy is a cool project and I am excited for it to succeed. However, I don't necessarily have
//! time to always keep this crate up to date with the latest versions of Bevy, especially if it
//! gets relatively low usage.
//!
//! That said, I *will* respond to pull requests and will release new versions based on pull
//! requests to update to newer versions of Bevy. Creating a pull request is preferable to opening
//! an issue asking me to update, because I can more easily spare the time to merge a pull request
//! than to do all the necessary updates myself, but if I do get issues asking me to update to new
//! versions of Bevy, I will respond to them on a best-effort basis.
//!
//! I will create releases targeting the latest published version of Bevy, not `main`. If you are
//! working on main, and need to modify this crate to work with the latest `HEAD`, I recommend
//! forking and then sending me a pull request once Bevy publishes an updated version.
//!
//! In terms of features, I would consider this project largely feature-complete as-is. Texture
//! packaging seems to be outside the scope of features supported by Bevy, so I'm not going to add
//! tools to automate building textures for skyboxes. This crate also isn't intended to support any
//! kind of dynamic skyboxes, so there doesn't seem to be much more that needs to be done besides
//! keeping up with latest versions of Bevy. However, if you have any ideas for new features or API
//! changes, I'm happy to hear them.
//!
//! # Disclaimer
//!
//! This is not an officially supported Google product.

use bevy::asset::load_internal_asset;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::{Mesh, MeshVertexBufferLayout};
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderStage, SpecializedMeshPipelineError, TextureViewDimension,
};

/// Configures the skybox render pipeline and support for [`SkyboxMaterial`]. Also sets up the system for [`
pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SKYBOX_VERT_HANDLE,
            "skybox.vert",
            |source: &'static str| { Shader::from_glsl(source, ShaderStage::Vertex) }
        );
        load_internal_asset!(
            app,
            SKYBOX_FRAG_HANDLE,
            "skybox.frag",
            |source: &'static str| { Shader::from_glsl(source, ShaderStage::Fragment) }
        );

        app.add_plugin(MaterialPlugin::<SkyboxMaterial>::default())
            .init_resource::<SkyboxTextureConversion>()
            .add_system(convert_skyboxes);
        // app.add_asset::<SkyboxMaterial>()
        //     .add_system_to_stage(
        //         CoreStage::PostUpdate,
        //         asset_shader_defs_system::<SkyboxMaterial>.system(),
        //     )
        // add_skybox_graph(app.world_mut());
        add_skybox_mesh(&mut *app.world.get_resource_mut().unwrap());

        // add default SkyboxMaterial
        let mut materials = app
            .world
            .get_resource_mut::<Assets<SkyboxMaterial>>()
            .unwrap();
        materials.set_untracked(
            Handle::<SkyboxMaterial>::default(),
            SkyboxMaterial {
                color: Color::PINK,
                ..Default::default()
            },
        );
    }
}

/// Bundle for spawning Skybox entities. Note that you should be able to use defaults for everything
/// besides `material`. The only other field you may want to touch is `transform` which can be used
/// to rotate the skybox if desired. Translations applied to skyboxes are ignored.
///
/// When inserting a skybox bundle, you should generally use `..Default::default()` for every
/// property except the `material` and occasionally `transform` (if you want to rotate the skybox
/// from its default orientation).
///
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_skybox_cubemap::{SkyboxBundle, SkyboxMaterial, SkyboxPlugin, SkyboxTextureConversion};
/// # App::build()
/// #     .add_startup_system(setup.system());
/// # fn setup(
/// #     mut commands: Commands,
/// #     asset_server: Res<AssetServer>,
/// #     mut meshes: ResMut<Assets<Mesh>>,
/// #     mut materials: ResMut<Assets<StandardMaterial>>,
/// #     mut skyboxes: ResMut<Assets<SkyboxMaterial>>,
/// #     mut skybox_conversion: ResMut<SkyboxTextureConversion>,
/// # ) {
/// # let skybox_texture = asset_server.load("labeled_skybox.png");
/// # skybox_conversion.make_array(skybox_texture.clone());
/// commands.spawn_bundle(SkyboxBundle::new(
///     skyboxes.add(SkyboxMaterial::from_texture(skybox_texture)),
/// ));
/// # }
/// ```
#[derive(Bundle)]
pub struct SkyboxBundle {
    /// Material to use for the skybox. Defaults to a garish pink. In most usage this should be the
    /// only field you need to set.
    pub material: Handle<SkyboxMaterial>,
    /// Mesh to use for the skybox. Defaults to [`SKYBOX_MESH_HANDLE`], which is a unit cube. You
    /// shouldn't ever need to use any other mesh. Because of how cubemap sampling works, probably
    /// any mesh that completely surrounds the camera would work equally well, but only the unit
    /// cube is officially supported by this crate.
    pub mesh: Handle<Mesh>,
    /// This is included in every type that can be drawn. Can be used to hide the skybox.
    pub visibility: Visibility,
    /// This is included in every type that can be drawn. Can be used to hide the skybox.
    pub computed_visibility: ComputedVisibility,
    // /// Needs to be configured to use the skybox render pipeline.
    // pub render_pipelines: RenderPipelines,
    /// Transform can be used to manipulate the rotation of the skybox.
    pub transform: Transform,
    /// Transforms get computed into global transforms used for drawing based on parenting. Note
    /// that it doesn't make much sense to add a skybox as a child of any other entity; it should
    /// usually be freestanding.
    pub global_transform: GlobalTransform,
}

impl SkyboxBundle {
    /// Convenience constructor for [`SkyboxBundle`]. Sets the material and uses defaults for
    /// everything else. In most use cases you should only need to set the material.
    pub fn new(material: Handle<SkyboxMaterial>) -> Self {
        Self {
            material,
            ..Default::default()
        }
    }
}

impl Default for SkyboxBundle {
    fn default() -> Self {
        Self {
            material: Default::default(),
            mesh: SKYBOX_MESH_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            // render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            //     SKYBOX_PIPELINE_HANDLE.typed(),
            // )]),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// Material for a Skybox. Consists of a base color and an optional 6-sided array-texture.
///
/// When rendering, the color from the texure is multiplied by the base color. This can be used to
/// tint the skybox. When creating a new material, the default color is [`Color::WHITE`] which will
/// have no effect on the texture color.
///
/// It is also possible to use a skybox texture with only a [`Color`]. One reason you might want to
/// do this is that (at time of writing) Bevy does not seem to antialias against the window
/// [`ClearColor`] properly, instead antialiasing with white for objects that have not other 3d
/// object behind them. This leads to white borders around antialiased object that overlap the
/// window clear color. To avoid this, you could spawn a skybox using only a color. Since the skybox
/// is a 3d rendered object, antialiasing against it works properly.
///
/// Skyboxes should generally be spawned using [`SkyboxBundle`], and you can see that type for info
/// on what components are used with this material.
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
// UUID5 generated by first creating a URL-namespaced UUID5 for
// "https://github.com/google/bevy_skybox_cubemap" (24291f52-ea01-574a-b6ae-3d8182f6086b) then using
// that as the namespace with `bevy_skybox_cubemap::SkyboxMaterial` as the name.
#[uuid = "fca7708e-57bb-5a81-977f-95b0e5202de0"]
pub struct SkyboxMaterial {
    /// Base color of the skybox. Multiplied with the color from the texture if a texture is
    /// supplied, otherwise used by itself as the skybox color.
    #[uniform(0)]
    pub color: Color,
    // /// Texture to use for the skybox. This must be a an aray texture with 6 layers which are all
    // /// square and the same size. See [the crate overview](crate) for details on the required layer
    // /// order and how to get a texture in this format.
    // #[texture(1, dimension = "cube")]
    // #[sampler(2)]
    // pub texture: Option<Handle<Image>>,
}

impl SkyboxMaterial {
    /// Creates a `SkyboxMaterial` with just a texture. The color will be set to [`Color::WHITE`] to
    /// avoid tinting the texture.
    pub fn from_texture(texture: Handle<Image>) -> Self {
        Self {
            // texture: Some(texture),
            ..Default::default()
        }
    }

    /// Creates a `SkyboxMaterial` with only a color. This could be used in place of [`ClearColor`]
    /// if `ClearColor` is giving you issues with antialiasing. Otherwise it's not all that useful.
    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
}

impl Default for SkyboxMaterial {
    /// Creates a new skybox material with color set to white and no texture.
    fn default() -> Self {
        Self {
            // Set the default color to white, so when using with a texture the color doesn't impact
            // the texture color.
            color: Color::WHITE,
            // texture: None,
        }
    }
}

impl Material for SkyboxMaterial {
    fn fragment_shader() -> ShaderRef {
        SKYBOX_FRAG_HANDLE.typed().into()
    }

    fn vertex_shader() -> ShaderRef {
        SKYBOX_VERT_HANDLE.typed().into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // if key.bind_group_data.has_texture {
        //     let fragment = descriptor.fragment.as_mut().unwrap();
        //     fragment
        //         .shader_defs
        //         .push("SKYBOXMATERIAL_TEXTURE".to_string());
        // }
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

/// Resource to help with converting skyboxes stored as vertically stacked images as described in
/// the [crate] documentation into array textures in the correct format for use in a
/// [`SkyboxMaterial`].
///
/// The [`SkyboxPlugin`] will add this resource and install an associated system which handles the
/// actual texture conversion. Conversion is performed using
/// [`Texture::reinterpret_stacked_2d_as_array`]. If you prefer, you are free to handle converting
/// textures yourself, or use a texture format + loader which can load array textures directly.
#[derive(Default)]
pub struct SkyboxTextureConversion {
    /// List of texture handles that should be skyboxes.
    handles: Vec<Handle<Image>>,
}

impl SkyboxTextureConversion {
    /// Takes a handle to a texture whose dimensions are `N` wide by `6*N` high, waits for it to load,
    /// and then reinterprets that texture as an array of 6 textures suitable or a skybox. This is
    /// useful if your skybox texture is not in a format that has layers. This should only be done
    /// once per testure, and will panic if the texture has already be reinterpreted.
    pub fn make_array(&mut self, handle: Handle<Image>) {
        self.handles.push(handle);
    }
}

/// System to handle reinterpreting an Nx6N vertical texture stack as an array of textures suitable
/// for a skybox.
fn convert_skyboxes(
    mut conversions: ResMut<SkyboxTextureConversion>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut i = 0;
    loop {
        // Check each texture in the pending queue to see if it is loaded yet.
        let (handle, texture) = match conversions.handles.get(i) {
            Some(handle) => match textures.get_mut(handle) {
                // If it's loaded, take it out of the queue.
                Some(texture) => (conversions.handles.remove(i), texture),
                None => {
                    i += 1;
                    continue;
                }
            },
            None => break,
        };

        debug!("Reinterpreting as Skybox Texture {:?}", handle,);
        texture.reinterpret_stacked_2d_as_array(6);
        texture.texture_view_descriptor.get_or_insert_with(default).dimension = Some(TextureViewDimension::Cube);
    }
}

// /// Constants defining node names in the render graph.
// pub mod node {
//     /// Node for the `SkyboxMaterial`.
//     pub const SKYBOX_MATERIAL: &str = "skybox_material";
// }
//
// /// Add the render graph and pipeline for the skybox to the world.
// fn add_skybox_graph(world: &mut World) {
//     {
//         let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
//         graph.add_system_node(
//             node::SKYBOX_MATERIAL,
//             AssetRenderResourcesNode::<SkyboxMaterial>::new(true),
//         );
//         graph
//             .add_node_edge(node::SKYBOX_MATERIAL, base::node::MAIN_PASS)
//             .unwrap();
//     }
//
//     let pipeline = build_skybox_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());
//     let mut pipelines = world
//         .get_resource_mut::<Assets<PipelineDescriptor>>()
//         .unwrap();
//     pipelines.set_untracked(SKYBOX_PIPELINE_HANDLE, pipeline);
// }

/// Handle to use to reference the skybox vertex shader.
pub const SKYBOX_VERT_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 16037920303847147810);
/// Handle to use to reference the skybox fragment shader.
pub const SKYBOX_FRAG_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8266127799467747040);

fn add_skybox_mesh(meshes: &mut Assets<Mesh>) {
    // Skybox mesh needs to be large enough not to get caught in the camera's near-clip plane (but
    // can otherwise be any value).
    meshes.set_untracked(SKYBOX_MESH_HANDLE, Mesh::from(shape::Cube { size: 1.0 }));
}

/// Handle to use to reference the skybox mesh.
pub const SKYBOX_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 7423141153313829192);

// /// Build the render pipeline for the skybox vertex and fragment shaders.
// fn build_skybox_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
//     PipelineDescriptor {
//         depth_stencil: Some(DepthStencilState {
//             format: TextureFormat::Depth32Float,
//             depth_write_enabled: true,
//             // Depth test needs to use LessEqual because it is forcing all points of the skybox to
//             // maximum depth.
//             depth_compare: CompareFunction::LessEqual,
//             stencil: StencilState {
//                 front: StencilFaceState::IGNORE,
//                 back: StencilFaceState::IGNORE,
//                 read_mask: 0,
//                 write_mask: 0,
//             },
//             bias: DepthBiasState {
//                 constant: 0,
//                 slope_scale: 0.0,
//                 clamp: 0.0,
//             },
//             clamp_depth: false,
//         }),
//         color_target_states: vec![ColorTargetState {
//             format: TextureFormat::default(),
//             color_blend: BlendState {
//                 src_factor: BlendFactor::SrcAlpha,
//                 dst_factor: BlendFactor::OneMinusSrcAlpha,
//                 operation: BlendOperation::Add,
//             },
//             alpha_blend: BlendState {
//                 src_factor: BlendFactor::One,
//                 dst_factor: BlendFactor::One,
//                 operation: BlendOperation::Add,
//             },
//             write_mask: ColorWrite::ALL,
//         }],
//         primitive: PrimitiveState {
//             topology: PrimitiveTopology::TriangleList,
//             strip_index_format: None,
//             front_face: FrontFace::Ccw,
//             cull_mode: CullMode::Front,
//             polygon_mode: PolygonMode::Fill,
//         },
//         ..PipelineDescriptor::new(ShaderStages {
//             vertex: shaders.add(Shader::from_glsl(
//                 ShaderStage::Vertex,
//                 include_str!("skybox.vert"),
//             )),
//             fragment: Some(shaders.add(Shader::from_glsl(
//                 ShaderStage::Fragment,
//                 include_str!("skybox.frag"),
//             ))),
//         })
//     }
// }
