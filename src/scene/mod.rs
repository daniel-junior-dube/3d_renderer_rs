
use std::collections::HashMap;
use std::path::Path;
use cgmath::{Point3, vec3};
use tobj;
use image;

pub mod camera;
pub mod entity;
pub mod light_source;
pub mod material;
pub mod object;

use self::camera::Camera;
use self::object::Object3D;
use self::light_source::LightSource;
use self::material::Material;
use self::entity::Entity3D;

/*
..####....####...######..##..##..######.
.##......##..##..##......###.##..##.....
..####...##......####....##.###..####...
.....##..##..##..##......##..##..##.....
..####....####...######..##..##..######.
........................................
*/

pub struct Scene {
	pub clear_color: [f32; 4],
	pub objects: Vec<Object3D>,
	pub camera: Camera,
	pub light_sources: Vec<LightSource>,
	pub materials: Vec<Material>,
	pub texture_map: HashMap<String, image::RgbaImage>,
}
impl Scene {
	pub fn new(clear_color: [f32; 4], objects: Vec<Object3D>, camera: Camera, light_sources: Vec<LightSource>, materials: Vec<Material>) -> Self {
		Scene {
			clear_color,
			objects,
			camera,
			light_sources,
			materials,
			texture_map: HashMap::new()
		}
	}

	pub fn new_test_scene() -> Self {
		let mut scene = Scene::new(
			[0.0, 0.0, 0.0, 1.0],
			vec![],
			Camera::new(Point3::new(0.0, 2.0, 0.0), Point3::new(0.0, 0.0, -10.0), vec3(0.0, 1.0, 0.0), 16.0 / 9.0),
			vec![],
			vec![],
		);
		scene.import_obj(Path::new("./data/venus.obj"));
		scene.light_sources.push(
			LightSource::new_translated(
				vec3(0.0, 0.5, 0.0)
			)
		);
		scene.light_sources.push(
			LightSource::new_translated(
				vec3(0.0, 10.0, -10.0)
			)
		);
		scene
	}

	pub fn load_and_add_textures(&mut self, material: &Material) {
		if !material.ambient_texture.is_empty() && !self.texture_map.contains_key(&material.ambient_texture) {
			self.texture_map.insert(
				material.ambient_texture.clone(),
				image::open(&material.ambient_texture).unwrap().to_rgba()
			);
		}
		if !material.diffuse_texture.is_empty() && !self.texture_map.contains_key(&material.diffuse_texture) {
			self.texture_map.insert(
				material.diffuse_texture.clone(),
				image::open(&material.diffuse_texture).unwrap().to_rgba()
			);
		}
		if !material.specular_texture.is_empty() && !self.texture_map.contains_key(&material.specular_texture) {
			self.texture_map.insert(
				material.specular_texture.clone(),
				image::open(&material.specular_texture).unwrap().to_rgba()
			);
		}
		if !material.normal_texture.is_empty() && !self.texture_map.contains_key(&material.normal_texture) {
			self.texture_map.insert(
				material.normal_texture.clone(),
				image::open(&material.normal_texture).unwrap().to_rgba()
			);
		}
		if !material.dissolve_texture.is_empty() && !self.texture_map.contains_key(&material.dissolve_texture) {
			self.texture_map.insert(
				material.dissolve_texture.clone(),
				image::open(&material.dissolve_texture).unwrap().to_rgba()
			);
		}
	}

	pub fn import_obj(&mut self, path: &Path) {
		let tobj_data = tobj::load_obj(&path);
		let (tobj_models, tobj_materials) = tobj_data.unwrap();
		let mut objects = Object3D::from_tobj_models(&tobj_models);
		let mut materials = Material::from_tobj_materials(&tobj_materials);

		// ? Load textures from each materials
		materials.iter().for_each(|material| self.load_and_add_textures(&material));

		// ? If self.materials is empty, no need to increment the objects's material ids
		if !self.materials.is_empty() {

			// ? Adjust the object's material ids since their ids just got offset
			objects.iter_mut().for_each(|object| {
				if let Some(ref mut material_id) = object.material_id {
					*material_id = *material_id + self.materials.len();
				}
			});
		}
		self.materials.append(&mut materials);
		self.objects.append(&mut objects);
	}
}