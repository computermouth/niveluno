#version 330 core

uniform vec4 color;
out vec4 fragColor;

void main() {
    // 4x4 bayer dither pattern
    int x = int(gl_FragCoord.x) % 4;
    int y = int(gl_FragCoord.y) % 4;

    int idx = x + y * 4;
    float threshold = float[16](
         0,  8,  2, 10,
        12,  4, 14,  6,
         3, 11,  1,  9,
        15,  7, 13,  5
    )[idx] / 16.0;
    
    if (color.a < threshold)
        discard;
    
    fragColor = vec4(color.rgb, 1.0);
}
