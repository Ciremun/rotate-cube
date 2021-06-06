#version 330 core

layout(location = 0) in vec3 vertexPosition_modelspace;

uniform mat4 MVP;

void main() {
    // gl_Position.xyz = vertexPosition_modelspace * vec3(0.3, 0.3, 0.0);
    // gl_Position.w = 1.0;
    gl_Position = MVP * vec4(vertexPosition_modelspace, 1);
}