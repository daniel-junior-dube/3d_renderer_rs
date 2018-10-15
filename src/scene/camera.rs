
use cgmath::{perspective, Deg, Matrix4, Point3, vec3, Rad, Vector3};

use scene::entity::Entity3D;

/*
..####....####...##...##..######..#####....####..
.##..##..##..##..###.###..##......##..##..##..##.
.##......######..##.#.##..####....#####...######.
.##..##..##..##..##...##..##......##..##..##..##.
..####...##..##..##...##..######..##..##..##..##.
.................................................
*/

pub struct Camera {
	pub eye: Point3<f32>,
	pub center: Point3<f32>,
	pub up: Vector3<f32>,
	pub view: Matrix4<f32>,
	pub projection: Matrix4<f32>,
}
impl Camera {
	pub fn new(eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>, aspect_ratio: f32) -> Self {
		Camera {
			eye,
			center,
			up,
			view: Matrix4::look_at(eye, center, up),
			projection: perspective(Deg(45.0), aspect_ratio, 0.1, 100.0),
		}
	}

	pub fn vp_matrix(&self) -> Matrix4<f32> {
		self.projection * self.view
	}
}

impl Entity3D for Camera {
	fn translate(&mut self, x: f32, y: f32, z: f32) {
		self.view = self.view * Matrix4::from_translation(vec3(x, y, z));
	}

	fn rotate(&mut self, axis: Vector3<f32>, angle: Rad<f32>) {
		self.view = self.view * Matrix4::from_axis_angle(axis, angle);
	}
}