#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 0, binding = 1) uniform CameraView {
    mat4 View;
};

layout(location = 0) out vec3 TexCoords;

void main() {
    // ViewProj is Proj * inverse(View). We want to get Proj * inverse(untranslatedView). However,
    // the only bindings available are ProjView and View. So we first get untranslatedView by
    // removing the translation from the View matrix (by clearing the last column), then multiply
    // ProjView * View to undo the earlier Proj * inverse(View). Then we multiply by the
    // untranslated view to get a projection matrix that has the camera's rotation but not position.
    mat4 untranslatedView = View;
    untranslatedView[3] = vec4(0.0, 0.0, 0.0, 1.0);

    mat4 untranslatedProj = ViewProj * View * inverse(untranslatedView);

    vec4 pos = untranslatedProj * vec4(Vertex_Position, 1.0);
    // Use w as z to force the point as far back a possible for depth testing purposes. This makes
    // sure it never draws in front of anything else.
    gl_Position = pos.xyww;

    // Since we're sampling a cubemap, texcoords is just the vertex coordinate.
    TexCoords = Vertex_Position;
}
