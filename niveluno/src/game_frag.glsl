#version 330 core

// Vertex positions, normals and UV coords
in vec3 vp, vn;
in vec2 vt;

uniform sampler2D s;

// count of (the vectors) of lights (2* num_lights)
uniform int light_count;

// Lights [(x,y,z,_), [r,g,b,_], ...]
// Must match MAX_LIGHT_VEC4S in render.rs, 1024 vec4s is the
// GL 3.3 guaranteed 16KiB uniform block size
layout(std140) uniform Lights {
    vec4 lights[1024];
};

// Use a static multiplier to light, instead
// of dynamic lighting
in vec3 out_glow;

out vec4 fragColor;

void main(void) {
    fragColor = texture(s, vt);

    // skip lights
    if (isinf(out_glow.x)) {
        return;
    }

    // override with a color
    if (!isnan(out_glow.x)) {
        fragColor.rgb = out_glow;
        return;
    }

    // Debug: no textures
    // fragColor.rgb = vec3(.5);

    // Calculate all lights
    vec3 vl = vec3(0, 0, 0);
    for (int i = 0; i < light_count; i += 2) {
        vl +=
            // Angle to normal
            max(
                dot(
                    vn, normalize(lights[i].xyz - vp)
                )
            , 0.0)
            * (1.0 / pow(length(lights[i].xyz - vp), 2.0)) // Inverse distance squared
            * lights[i + 1].rgb; // Light color/intensity
    }

    // Debug: full bright lights
    // vl = vec3(2, 2, 2);

    // enforce minimum luminosity
    vl = max(vl, vec3(0.2, 0.2, 0.2));

    vec3 p = pow(vl, vec3(0.75));
    
    fragColor.rgb = floor(
        fragColor.rgb * p // Light, Gamma
        * 16.0 + 0.5
    ) / 16.0; // Reduce final output color for some extra dirty looks
}
