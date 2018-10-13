
use cgmath::{Point3, vec3};
use std::path::Path;

pub mod camera;
pub mod entity;
pub mod light_source;
pub mod object;

use self::camera::Camera;
use self::object::Object3D;
use self::light_source::LightSource;
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
	pub light_sources: Vec<LightSource>
}
impl Scene {
	pub fn new() -> Self {
		Scene {
			clear_color: [0.0, 0.0, 0.0, 1.0],
			objects: vec![],
			camera: Camera::new(Point3::new(0.0, 2.0, 0.0), Point3::new(0.0, 0.0, -4.0), vec3(0.0, 1.0, 0.0), 16.0 / 9.0),
			light_sources: vec![],
		}
	}

	pub fn new_test_scene() -> Self {
		let mut scene = Scene {
			clear_color: [0.0, 0.0, 0.0, 1.0],
			objects: vec![],
			camera: Camera::new(Point3::new(0.0, 2.0, 0.0), Point3::new(0.0, 0.0, -4.0), vec3(0.0, 1.0, 0.0), 16.0 / 9.0),
			light_sources: vec![],
		};

		{
			let mut cube = Object3D::new_cube();
			cube.translate(0.0, 0.0, -4.0);
			scene.objects.push(cube);
		}
		{
			let mut objects = Object3D::from_obj(Path::new("./data/suzanne.obj"));
			let mut suzanne = objects.pop().unwrap();
			suzanne.translate(0.0, 0.0, -10.0);
			scene.objects.push(suzanne);
		}
		{
			let mut light = LightSource::new();
			light.translate(0.0, 10.0, -10.0);
			light.diffuse[0] = 0.0;
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
}