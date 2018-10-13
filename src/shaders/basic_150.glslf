#version 150 core
#define MAX_LIGHTS 250

uniform Locals {
	vec4 u_EyePosition;
	int u_NumLights; // active number of lights
};

struct Light {
	vec4 pos;	// world position
	vec4 ambiant;
	vec4 diffuse;
	vec4 specular;
};

uniform b_Lights {
	Light u_Lights[MAX_LIGHTS];
};

varying vec3 f_color;

void main() {
	//vec4 N = 
	vec4 O = u_EyePosition;
	for (int i = 0; i < u_NumLights && i < MAX_LIGHTS; ++i) {
		Light light = u_Lights[i];
		//vec4 L = 
		//float theta = 
		//float cos_theta = 

		//float alpha = 
		//float cos_alpha = 

		// Do something with the light..
		//gl_FragColor = vec4(light.diffuse.x, light.diffuse.y, light.diffuse.z, 1.0);
	}
	gl_FragColor = vec4(f_color.x, f_color.y, f_color.z, 1.0);
}
