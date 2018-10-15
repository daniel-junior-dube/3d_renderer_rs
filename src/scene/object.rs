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
	pub material_id: Option<usize>,
	pub translation: Vector3<f32>,
	pub rotation: Matrix4<f32>,
	pub scale: f32,
}
impl Object3D {
	pub fn new(name: String, vertices: Vec<[f32; 3]>, indices: Vec<u32>, uvs: Vec<[f32; 2]>, normals: Vec<[f32; 3]>, material_id: Option<usize>) -> Self {
		let mut object_vertices = vec![];
		for i in indices {
			let index = i as usize;
			let vertex = &vertices[index];
			let uv = if let Some(uv) = uvs.get(index) {
				*uv
			} else {
				[0.0, 0.0]
			};
			let color = [1.0, 1.0, 1.0];
			let normal = &normals[index];
			object_vertices.push(
				Vertex::new(*vertex, color, *normal, uv)
			);
		}
		Object3D {
			name,
			vertices: object_vertices,
			material_id,
			translation: vec3(0.0, 0.0, 0.0),
			rotation: Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0), Deg(0.0)),
			scale: 1.0,
		}
	}

	pub fn from_tobj_model(model: &tobj::Model) -> Self {
		let mesh = &model.mesh;
		let name = model.name.clone();
		let material_id = mesh.material_id;
		let indices = mesh.indices.clone();
		let mut vertices = vec![];
		for vertex_index in 0..mesh.positions.len() / 3 {
			let first_coord_index = 3 * vertex_index;
			let x = mesh.positions[first_coord_index];
			let y = mesh.positions[first_coord_index + 1];
			let z = mesh.positions[first_coord_index + 2];
			vertices.push([x, y, z]);
		}
		let mut uvs = vec![];
		let mut normals = vec![];
		for i in indices.iter() {
			let normal_index = (i * 3) as usize;
			if normal_index < mesh.normals.len() {
				normals.push(
					[
						mesh.normals[normal_index],
						mesh.normals[normal_index + 1],
						mesh.normals[normal_index + 2],
					]
				);
			}
			let texture_index = (i * 2) as usize;
			if texture_index < mesh.texcoords.len() {
				uvs.push(
					[
						mesh.texcoords[texture_index],
						mesh.texcoords[texture_index + 1]
					]
				);
			}
		}
		Object3D::new(name, vertices, indices, uvs, normals, material_id)
	}

	pub fn from_tobj_models(models: &Vec<tobj::Model>) -> Vec<Self> {
		models.iter().map(|model| Object3D::from_tobj_model(model)).collect()
	}

	pub fn new_cube() -> Self {
		let mut vertices = vec![
			// front
			[ -1.0, -1.0, 1.0 ],
			[  1.0, -1.0, 1.0 ],
			[  1.0,  1.0, 1.0 ],
			[ -1.0,  1.0, 1.0 ],
			// back
			[ -1.0, -1.0, -1.0 ],
			[  1.0, -1.0, -1.0 ],
			[  1.0,  1.0, -1.0 ],
			[ -1.0,  1.0, -1.0 ],
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
		let normals = Object3D::evaluate_vertex_normals(&mut vertices, &indices);
		Object3D::new(String::from("cube"), vertices, indices, vec![], normals, None)
	}

	pub fn model_matrix(&self) -> Matrix4<f32> {
		Matrix4::from_translation(self.translation) * self.rotation * Matrix4::from_scale(self.scale)
	}

	fn evaluate_vertex_normals(vertices: &mut Vec<[f32; 3]>, indices: &Vec<u32>) -> Vec<[f32; 3]> {
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
				&vec3(v1[0], v1[1], v1[2]),
				&vec3(v2[0], v2[1], v2[2])
			);
			normals[index_v1] += normal;
			normals[index_v2] += normal;
			normals[index_v3] += normal;
		}

		// ? Assign and normalize each normal for the corresponding vertices
		let normals: Vec<[f32; 3]> = normals.iter().map(|normal| {
			let normalized_normal = normal.normalize();
			[normalized_normal.x, normalized_normal.y, normalized_normal.z]
		}).collect();
		normals
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
