#version 150 core
#define MAX_LIGHTS 250

const float gamma = 1.0/2.2; // Assume the monitor is calibrated to the sRGB color space
const vec3 global_ambient_color = vec3(0.0, 0.0, 0.0);

struct Light {
	vec4 pos; // world position
	vec4 color;
};

uniform sampler2D u_DiffuseTexture;

uniform Locals {
	vec4 u_EyePosition;
	int u_NumLights; // active number of lights
};

uniform b_lights {
	Light u_lights[MAX_LIGHTS];
};

varying vec4 f_vertexpos;
varying vec3 f_color;
varying vec4 f_normal;
varying vec2 f_uv;

void main() {
	vec3 texture_pixel_color = texture(u_DiffuseTexture, f_uv).rgb;
	vec4 O = normalize(u_EyePosition - f_vertexpos);
	vec3 intensity = global_ambient_color;
	for (int i = 0; i < u_NumLights && i < MAX_LIGHTS; ++i) {
		Light light = u_lights[i];
		float distanceToLight = length(light.pos - f_vertexpos);
		float attenuationFactor = 1.0 / (1.0 + (0.05) * pow(distanceToLight, 2));

		vec4 L = normalize(light.pos - f_vertexpos);
		//vec4 reflectDir = reflect(-L, f_normal);
    	//float spec = pow(max(dot(O, reflectDir), 0.0), 1);

		float cosTheta = max(dot(f_normal, L), 0.0);
		if (cosTheta > 0) {
			vec3 diffuse = light.color.rgb * texture_pixel_color * cosTheta;
			//vec4 specular = light.specular * spec * object_specular;

			intensity += (diffuse) * attenuationFactor;
		}
	}

	vec3 gamma_corrected_intensity = pow(intensity, vec3(gamma));
	gl_FragColor = vec4(gamma_corrected_intensity, 1.0);
}
