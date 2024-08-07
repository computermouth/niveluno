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

// Model position (x, y, z)
uniform vec3 model_pos;

// Model rotation (yaw, pitch)
uniform vec2 model_rot;

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

void main(void) {
    f_unlit = float(unlit);

    // Rotation Matrices for model rotation
    mat4 mry = ry(model_rot.x);
    mat4 mrz = rz(model_rot.y);

    // Mix vertex positions, rotate and add the model position
    vp = (mry * mrz * vec4(mix(p, p2, blend), 1.0)).xyz + model_pos;

    // Mix normals
    vn = (mry * mrz * vec4(mix(n, n2, blend), 1.0)).xyz;

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
