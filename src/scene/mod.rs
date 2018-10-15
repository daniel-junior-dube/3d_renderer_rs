
use std::path::Path;
use cgmath::{Point3, vec3};
use tobj;

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
	pub materials: Vec<Material>
}
impl Scene {
	pub fn new(clear_color: [f32; 4], objects: Vec<Object3D>, camera: Camera, light_sources: Vec<LightSource>, materials: Vec<Material>) -> Self {
		Scene {
			clear_color,
			objects,
			camera,
			light_sources,
			materials,
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
		{
			let mut cube = Object3D::new_cube();
			cube.translate(0.0, 0.0, -4.0);
			scene.objects.push(cube);
		}
		{
			let mut light = LightSource::new();
			light.translate(0.0, 0.5, 0.0);
			scene.light_sources.push(light);
		}
		{
			let mut light = LightSource::new();
			light.translate(0.0, 10.0, -10.0);
			light.diffuse[1] = 0.0;
			scene.light_sources.push(light);
		}
		scene
	}

	pub fn import_obj(&mut self, path: &Path) {
		let tobj_data = tobj::load_obj(&path);
		let (tobj_models, tobj_materials) = tobj_data.unwrap();
		let mut objects = Object3D::from_tobj_models(&tobj_models);
		let mut materials = Material::from_tobj_materials(&tobj_materials);

		if self.materials.is_empty() {
			self.materials = materials;
		} else {
			self.materials.append(&mut materials);

			// ? Adjust the object's material ids since their ids just got offset
			objects.iter_mut().for_each(|object| {
				if let Some(ref mut material_id) = object.material_id {
					*material_id = *material_id + self.materials.len();
				}
			});
		}

		self.objects.append(&mut objects);
	}
}