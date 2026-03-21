#version 330 core
in vec3 p;
uniform vec4 camera_pos;
uniform vec2 mouse;

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

void main() {
    gl_Position = mat4(
        1, 0, 0, 0,
        0, camera_pos.w, 0, 0,
        0, 0, 1, 1,
        0, 0, -1, 0
    ) * rx(-mouse.y) * ry(-mouse.x) * vec4(p - camera_pos.xyz, 1.0);
}
