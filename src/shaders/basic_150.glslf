#version 150 core
#define MAX_LIGHTS 250

const float screenGamma = 2.2; // Assume the monitor is calibrated to the sRGB color space
const vec4 ambientColor = vec4(0.1, 0.1, 0.1, 1.0);
const vec4 object_diffuse = vec4(0.5, 0.5, 0.5, 1.0);
const vec4 object_specular = vec4(0.0, 0.0, 0.0, 1.0);

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

varying vec4 f_VertexPos;
varying vec4 f_Color;
varying vec4 f_Normal;
varying vec3 f_UV;

void main() {
	vec4 N = f_Normal;
	vec4 O = normalize(u_EyePosition -  f_VertexPos);
	vec4 intensity = ambientColor;
	for (int i = 0; i < u_NumLights && i < MAX_LIGHTS; ++i) {
		Light light = u_Lights[i];
		vec4 L = normalize(light.pos - f_VertexPos);
		vec4 reflectDir = reflect(-L, f_Normal);
    	float spec = pow(max(dot(O, reflectDir), 0.0), 1);

		float distanceToLight = length(light.pos - f_VertexPos);
		float attenuation = 1.0 / (1.0 + (0.05) * pow(distanceToLight, 2));

		float lambertian = max(dot(f_Normal, L), 0.0);
		if (lambertian > 0.0) {
			//gl_FragColor = vec4(f_Color.x, f_Color.y, f_Color.z, 1.0);
		}
		vec4 ambiant = light.ambiant * object_diffuse; // TODO: Change 'object_diffuse' to 'vec3(texture(material.diffuse, TexCoords))';
		vec4 diffuse = light.diffuse * lambertian * object_diffuse;
		vec4 specular = light.specular * spec * object_specular;

		intensity += (ambiant + diffuse + specular) * attenuation;
	}

	gl_FragColor = pow(intensity, vec4(1.0/screenGamma));
}
