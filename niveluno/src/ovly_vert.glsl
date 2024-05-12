#version 300 es
in vec4 position;
in vec2 texcoord;
out vec2 v_texcoord;
void main(){
    gl_Position = position;
    v_texcoord = texcoord;
}