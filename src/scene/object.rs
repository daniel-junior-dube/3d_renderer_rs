
use cgmath::{Deg, Matrix4, vec3, Vector3, Rad};

use common::*;
use scene::entity::Entity3D;

/*
..####...#####...######..######...####...######..######..#####..
.##..##..##..##......##..##......##..##....##.......##...##..##.
.##..##..#####.......##..####....##........##......###...##..##.
.##..##..##..##..##..##..##......##..##....##........##..##..##.
..####...#####....####...######...####.....##....#####...#####..
................................................................
*/

pub struct Object3D {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
	pub translation: Vector3<f32>,
	pub rotation: Matrix4<f32>,
	pub scale: f32,
}
impl Object3D {
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
		Object3D {
			vertices: vertices,
			indices: indices,
			translation: vec3(0.0, 0.0, 0.0),
			rotation: Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(0.0)),
			scale: 1.0,
		}
	}

	pub fn new_cube() -> Self {
		Object3D::new(
			vec![
				// front
				Vertex { pos: [ -1.0, -1.0, 1.0 ], color: [ 1.0, 0.0, 0.0 ] },
				Vertex { pos: [  1.0, -1.0, 1.0 ], color: [ 0.0, 1.0, 0.0 ] },
				Vertex { pos: [  1.0,  1.0, 1.0 ], color: [ 0.0, 0.0, 1.0 ] },
				Vertex { pos: [ -1.0,  1.0, 1.0 ], color: [ 1.0, 1.0, 1.0 ] },
				// back
				Vertex { pos: [ -1.0, -1.0, -1.0 ], color: [ 1.0, 0.0, 0.0 ] },
				Vertex { pos: [  1.0, -1.0, -1.0 ], color: [ 0.0, 1.0, 0.0 ] },
				Vertex { pos: [  1.0,  1.0, -1.0 ], color: [ 0.0, 0.0, 1.0 ] },
				Vertex { pos: [ -1.0,  1.0, -1.0 ], color: [ 1.0, 1.0, 1.0 ] }
			],
			vec![
				// front
				0, 1, 2,
				2, 3, 0,
				// top
				1, 5, 6,
				6, 2, 1,
				// back
				7, 6, 5,
				5, 4, 7,
				// bottom
				4, 0, 3,
				3, 7, 4,
				// left
				4, 5, 1,
				1, 0, 4,
				// right
				3, 2, 6,
				6, 7, 3
			]
		)
	}

	pub fn model_matrix(&self) -> Matrix4<f32> {
		Matrix4::from_translation(self.translation) * self.rotation * Matrix4::from_scale(self.scale)
	}
}

impl Entity3D for Object3D {
	fn translate(&mut self, x: f32, y: f32, z: f32) {
		self.translation += vec3(x, y, z);
	}

	fn rotate(&mut self, axis: Vector3<f32>, angle: Rad<f32>) {
		self.rotation = self.rotation * Matrix4::from_axis_angle(axis, angle);
	}
}
