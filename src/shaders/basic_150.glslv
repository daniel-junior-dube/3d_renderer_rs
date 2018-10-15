#version 150 core

uniform Locals {
	vec4 u_EyePosition;
	int u_NumLights; // active number of lights
};

attribute vec4 v_pos;
attribute vec4 v_color;
attribute vec4 v_normal;
attribute vec3 v_uv;

uniform mat4 u_MVP;
uniform mat4 u_ViewModel;

varying vec4 f_VertexPos;
varying vec4 f_Color;
varying vec4 f_Normal;
varying vec3 f_UV;

void main() {
	gl_Position = u_MVP * v_pos;
	f_VertexPos = gl_Position;
	f_Color = v_color;
	f_Normal = v_normal;
	f_UV = v_uv;
}
