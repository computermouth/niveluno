#version 300 es

precision highp float;

// Vertex positions, normals and UV coords for the fragment shader
out vec3 vp, vn;
out vec2 vt;

// Input vertex positions & normals and blend vertex positions & normals
in vec3 p, n, p2, n2;

// Input UV coords
in vec2 t;

// Camera position (x, y, z) and aspect ratio (w)
uniform vec4 camera_pos;

// Model scale, rotation, translation(pos) v4 -> matrix
uniform vec4 model_mat_v1;
uniform vec4 model_mat_v2;
uniform vec4 model_mat_v3;
uniform vec4 model_mat_v4;

// Mouse rotation yaw (x), pitch (y)
uniform vec2 mouse;

// Blend factor between the two vertex positions
uniform float blend;

// Flag to turn off lighting in the frag shader
uniform int unlit;
out float f_unlit;

// Generate a rotation Matrix around the x,y,z axis;
// Used for model rotation and camera yaw
mat4 rx(float r) {
    return mat4(
        1, 0, 0, 0,
        0, cos(r), sin(r), 0,
        0, -sin(r), cos(r), 0,
        0, 0, 0, 1
    );
}

mat4 ry(float r) {
    return mat4(
        cos(r), 0, -sin(r), 0,
        0, 1, 0, 0,
        sin(r), 0, cos(r), 0,
        0, 0, 0, 1
    );
}

mat4 rz(float r) {
    return mat4(
        cos(r), sin(r), 0, 0,
        -sin(r), cos(r), 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    );
}

struct mat_comp {
    mat3 rotation;
    vec3 translation;
    vec3 scale;
};

mat_comp decomp_mat(mat4 matrix) {
    mat_comp components;

    components.translation = vec3(matrix[3].xyz);

    components.scale = vec3(
        length(matrix[0].xyz),
        length(matrix[1].xyz),
        length(matrix[2].xyz)
    );

    components.rotation = mat3(
        matrix[0].xyz / components.scale.x,
        matrix[1].xyz / components.scale.y,
        matrix[2].xyz / components.scale.z
    );

    return components;
}

void main(void) {
    f_unlit = float(unlit);

    // scale, rotation, translation matrix
    mat4 model_mat = mat4 (
        model_mat_v1,
        model_mat_v2,
        model_mat_v3,
        model_mat_v4
    );

    mat_comp model = decomp_mat(model_mat);
 
    // Mix vertex positions, rotate using mat3, and add the translation
    vp = model.rotation * (model.scale * mix(p, p2, blend)) + model.translation;

    // Mix normals
    vn = model.rotation * mix(n, n2, blend);

    // UV coords are handed over to the fragment shader as is
    vt = t;

    // Final vertex position is transformed by the projection matrix,
    // rotated around mouse yaw/pitch and offset by the camera position
    // We use a FOV of 90, so the matrix[0] and [5] are conveniently 1.
    // (1 / Math.tan((90/180) * Math.PI / 2) === 1)
    gl_Position = mat4(
        1, 0, 0, 0,
        0, camera_pos.w, 0, 0,
        0, 0, 1, 1,
        0, 0, -2, 0
    ) * // projection
    rx(-mouse.y) * ry(-mouse.x) *
    vec4(vp - camera_pos.xyz, 1.0);
}
