#version 310 es
layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texcoord;
layout(location = 0) out vec2 v_texcoord;
void main(){
    gl_Position = position;
    v_texcoord = texcoord;
}
