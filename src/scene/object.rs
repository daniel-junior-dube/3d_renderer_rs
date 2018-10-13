use std::path::Path;

use cgmath::{Deg, Matrix4, vec3, Vector3, Rad};
use cgmath::InnerSpace;
use tobj;

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
	pub name: String,
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
	pub translation: Vector3<f32>,
	pub rotation: Matrix4<f32>,
	pub scale: f32,
}
impl Object3D {
	pub fn new(name: String, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
		Object3D {
			name,
			vertices,
			indices,
			translation: vec3(0.0, 0.0, 0.0),
			rotation: Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(0.0)),
			scale: 1.0,
		}
	}

	pub fn from_obj(path: &Path) -> Vec<Self> {
		let tobj_data = tobj::load_obj(&path);
		let (models, _materials) = tobj_data.unwrap();

		let mut objects = vec![];
		for (i, model) in models.iter().enumerate() {
			let mesh = &model.mesh;

			let name = model.name.clone();
			let indices = mesh.indices.clone();
			let mut vertices = vec![];
			for vertex_index in 0..mesh.positions.len() / 3 {
				let first_coord_index = 3 * vertex_index;
				let x = mesh.positions[first_coord_index];
				let y = mesh.positions[first_coord_index + 1];
				let z = mesh.positions[first_coord_index + 2];
				vertices.push(
					Vertex::new_default([x, y, z])
				);
			}

			objects.push(Object3D::new(name, vertices, indices));
		}
		objects
	}

	pub fn new_cube() -> Self {
		let mut vertices = vec![
			// front
			Vertex::new_default([ -1.0, -1.0, 1.0 ]),
			Vertex::new_default([  1.0, -1.0, 1.0 ]),
			Vertex::new_default([  1.0,  1.0, 1.0 ]),
			Vertex::new_default([ -1.0,  1.0, 1.0 ]),
			// back
			Vertex::new_default([ -1.0, -1.0, -1.0 ]),
			Vertex::new_default([  1.0, -1.0, -1.0 ]),
			Vertex::new_default([  1.0,  1.0, -1.0 ]),
			Vertex::new_default([ -1.0,  1.0, -1.0 ])
		];
		let indices = vec![
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
		];
		Object3D::evaluate_vertex_normals(&mut vertices, &indices);
		Object3D::new(String::from("cube"), vertices, indices)
	}

	pub fn model_matrix(&self) -> Matrix4<f32> {
		Matrix4::from_translation(self.translation) * self.rotation * Matrix4::from_scale(self.scale)
	}

	fn evaluate_vertex_normals(vertices: &mut Vec<Vertex>, indices: &Vec<u32>) {
		// ? Init vec to temporarly store normals at vertices positions
		let mut normals:Vec<Vector3<f32>> = vertices.into_iter().map(|_vertex| vec3(0.0, 0.0, 0.0)).collect();

		// ? Fill the temporary normal data
		let nb_tris = indices.len() / 3;
		for i in 0..nb_tris {
			let real_index = 3 * i as usize;
			let index_v1 = indices[real_index] as usize;
			let index_v2 = indices[real_index + 1] as usize;
			let index_v3 = indices[real_index + 2] as usize;
			let v1 = vertices[index_v1];
			let v2 = vertices[index_v2];
			let normal = Object3D::normal(
				&vec3(v1.pos[0], v1.pos[1], v1.pos[2]),
				&vec3(v2.pos[0], v2.pos[1], v2.pos[2])
			);
			/* if i+1 < nb_tris {
				// TODO: check if chunk of 6 indices is a quad
				{
					// ? Get shared indices
					let mut check = vec![];
					let mut nb_duplicates = indices[real_index..real_index+3].into_iter().fold(0, |acc, elem| {
						if check.contains(elem) {
							acc + 1
						} else {
							check.push(*elem);
							acc
						}
					});

					// ? Found 2 shared indices
					if nb_duplicates == 2 {

					}
				}
				let index_v4 = indices[real_index + 3] as usize;
				let index_v5 = indices[real_index + 4] as usize;
				let index_v6 = indices[real_index + 5] as usize;

			} else {
				normals[index_v1] += normal;
				normals[index_v2] += normal;
				normals[index_v3] += normal;
			}*/
			normals[index_v1] += normal;
			normals[index_v2] += normal;
			normals[index_v3] += normal;
		}

		// ? Assign and normalize each normal for the corresponding vertices
		for (i, Vertex {normal, ..}) in vertices.into_iter().enumerate() {
			let normalized_normal = normals[i].normalize();
			normal[0] = normalized_normal.x;
			normal[1] = normalized_normal.y;
			normal[2] = normalized_normal.z;
		}
	}

	fn normal(v1: &Vector3<f32>, v2: &Vector3<f32>) -> Vector3<f32> {
		let normal = v1.cross(*v2).normalize();
		normal
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
