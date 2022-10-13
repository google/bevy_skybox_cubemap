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

#version 450
layout(location = 0) in vec3 TexCoords;

layout(set = 2, binding = 0) uniform SkyboxMaterial_color {
    vec4 color;
};
//#ifdef SKYBOXMATERIAL_TEXTURE
// layout(set = 2, binding = 1) uniform textureCube SkyboxMaterial_texture;
// layout(set = 2, binding = 2) uniform sampler SkyboxMaterial_texture_sampler;
//#endif

layout(location = 0) out vec4 o_Target;

void main() {
//#ifdef SKYBOXMATERIAL_TEXTURE
    // vec3 uvIndex = sampleCubeHacky(TexCoords);
    // o_Target = texture(
    //     sampler2DArray(SkyboxMaterial_texture, SkyboxMaterial_texture_sampler),
    //     uvIndex
    // ) * color;

    // o_Target = texture(
    //     samplerCube(SkyboxMaterial_texture, SkyboxMaterial_texture_sampler),
    //     TexCoords
    // ) * color;
//#else
    o_Target = color;
//#endif
}
