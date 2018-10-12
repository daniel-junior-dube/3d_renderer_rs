#version 150 core
#define MAX_LIGHTS 250

uniform PsLocals {
	// active number of lights
	int u_NumLights;
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
	for (int i = 0; i < u_NumLights && i < MAX_LIGHTS; ++i) {
		Light light = u_Lights[i];
		// Do something with the light..
		gl_FragColor = vec4(light.diffuse.x, light.diffuse.y, light.diffuse.z, 1.0);
	}
	//gl_FragColor = vec4(f_color.x, f_color.y, f_color.z, 1.0);
}
