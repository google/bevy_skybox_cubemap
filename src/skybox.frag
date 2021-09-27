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

layout(set = 1, binding = 0) uniform SkyboxMaterial_color {
    vec4 color;
};
#ifdef SKYBOXMATERIAL_TEXTURE
layout(set = 1, binding = 1) uniform texture2DArray SkyboxMaterial_texture;
layout(set = 1, binding = 2) uniform sampler SkyboxMaterial_texture_sampler;
#endif

layout(location = 0) out vec4 o_Target;

// This is a handwritten cubemap sampler. We should use the shader language's builtin cubemap
// sampling, but that doesn't work right now because Bevy has a bug binding cubemaps currently.
// So instead we just write our own cubemap sampler and use an array texture, which does work
// correctly.
// TODO: Just use a normal cubemap once those work in bevy.
vec3 sampleCubeHacky(const vec3 ray) {
    vec3 rayAbs = abs(ray);
    float maxAdjust;
    float faceIndex;
    vec2 uv;
    if (rayAbs.z >= rayAbs.x && rayAbs.z >= rayAbs.y) {
        faceIndex = ray.z < 0.0 ? 5.0 : 4.0;
        maxAdjust = 0.5 / rayAbs.z;
        uv = vec2(ray.x * -sign(ray.z), -ray.y);
    } else if (rayAbs.y >= rayAbs.x) {
        faceIndex = ray.y < 0.0 ? 3.0 : 2.0;
        maxAdjust = 0.5 / ray.y;
        uv = vec2(ray.x, ray.z * -sign(ray.y)) * sign(ray.y);
    } else {
        faceIndex = ray.x < 0.0 ? 1.0 : 0.0;
        maxAdjust = 0.5 / ray.x;
        uv = vec2(ray.z, ray.y * -sign(ray.x));
    }
    return vec3(uv * maxAdjust + 0.5, faceIndex);
}

void main() {
#ifdef SKYBOXMATERIAL_TEXTURE
    vec3 uvIndex = sampleCubeHacky(TexCoords);
    o_Target = texture(
        sampler2DArray(SkyboxMaterial_texture, SkyboxMaterial_texture_sampler),
        uvIndex
    ) * color;

    // This is how this should work.
    // o_Target = texture(
    //     samplerCube(SkyboxMaterial_texture, SkyboxMaterial_texture_sampler),
    //     TexCoords
    // ) * color;
#else
    o_Target = color;
#endif
}
