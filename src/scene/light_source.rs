
use cgmath::{perspective, Deg, Matrix4, Point3, vec3, Rad, Vector3};
use gfx::shade::ToUniform;
use gfx::shade::core::UniformValue;

use scene::entity::Entity3D;

/*
.##......######...####...##..##..######...####....####...##..##..#####....####...######.
.##........##....##......##..##....##....##......##..##..##..##..##..##..##..##..##.....
.##........##....##.###..######....##.....####...##..##..##..##..#####...##......####...
.##........##....##..##..##..##....##........##..##..##..##..##..##..##..##..##..##.....
.######..######...####...##..##....##.....####....####....####...##..##...####...######.
........................................................................................
*/

// ? WIP
pub struct LightSource {
	pub ambiant: [f32; 4],
	pub diffuse: [f32; 4],
	pub specular: [f32; 4],
	pub translation: Vector3<f32>,
	pub rotation: Matrix4<f32>,
	pub scale: f32,
}

impl LightSource {
	pub fn new() -> Self {
		LightSource {
			ambiant: [1.0, 1.0, 1.0, 1.0],
			diffuse: [1.0, 1.0, 1.0, 1.0],
			specular: [1.0, 1.0, 1.0, 1.0],
			translation: vec3(0.0, 0.0, 0.0),
			rotation: Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(0.0)),
			scale: 1.0,
		}
	}

	pub fn model_matrix(&self) -> Matrix4<f32> {
		Matrix4::from_translation(self.translation) * self.rotation * Matrix4::from_scale(self.scale)
	}
}

impl Entity3D for LightSource {
	fn translate(&mut self, x: f32, y: f32, z: f32) {
		self.translation += vec3(x, y, z);
	}

	fn rotate(&mut self, axis: Vector3<f32>, angle: Rad<f32>) {
		self.rotation = self.rotation * Matrix4::from_axis_angle(axis, angle);
	}
}