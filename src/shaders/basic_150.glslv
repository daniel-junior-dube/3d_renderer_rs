#version 150 core

uniform Locals {
	vec4 u_EyePosition;
	int u_NumLights; // active number of lights
};

attribute vec3 v_pos;
attribute vec3 v_color;
attribute vec3 v_normal;

uniform mat4 u_MVP;
uniform mat4 u_ViewModel;

varying vec3 f_color;

void main() {
	gl_Position = u_MVP * vec4(v_pos, 1.0);
	if ((u_ViewModel * vec4(v_pos, 1.0)).y > u_EyePosition.y) {
		f_color = vec3(0, 1, 1);
	} else {
		f_color = v_color;
	}
}
