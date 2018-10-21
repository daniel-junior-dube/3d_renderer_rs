#version 150 core

uniform Locals {
	vec4 u_EyePosition;
	int u_NumLights; // active number of lights
};

attribute vec4 v_pos;
attribute vec3 v_color;
attribute vec4 v_normal;
attribute vec2 v_uv;

uniform mat4 u_MVP;
uniform mat4 u_ViewModel;

varying vec4 f_vertexpos;
varying vec3 f_color;
varying vec4 f_normal;
varying vec2 f_uv;

void main() {
	gl_Position = u_MVP * v_pos;
	f_vertexpos = u_ViewModel * v_pos;
	f_color = v_color;
	f_normal = v_normal;
	f_uv = v_uv;
}
