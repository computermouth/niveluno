#version 300 es

precision highp float;

// Vertex positions, normals and UV coords
in vec3 vp, vn;
in vec2 vt;

uniform sampler2D s;

// count of (the vectors) of lights (2* num_lights)
uniform int light_count;

// Lights [(x,y,z), [r,g,b], ...]
uniform vec3 lights[64];

// flag to turn off lighting
in float f_unlit;

out vec4 fragColor;

void main(void) {
    fragColor = texture(s, vt);

    // Debug: no textures
    // fragColor = vec4(.5);

    // Calculate all lights
    vec3 vl = vec3(0, 0, 0);
    for (int i = 0; i < light_count; i += 2) {
        vl +=
            // Angle to normal
            max(
                dot(
                    vn, normalize(lights[i] - vp)
                )
            , 0.0) *
            (1.0 / pow(length(lights[i] - vp), 2.0)) // Inverse distance squared
            * lights[i + 1]; // Light color/intensity
    }

    // Debug: full bright lights
    // vl = vec3(2, 2, 2);

    vec3 p = pow(vl, vec3(0.75));
    if (f_unlit != 0.0) { p = vec3(1.25); }

    fragColor.rgb = floor(
        fragColor.rgb * p // Light, Gamma
        * 16.0 + 0.5
    ) / 16.0; // Reduce final output color for some extra dirty looks
}
