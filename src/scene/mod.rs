
use cgmath::{Point3, vec3};

pub mod camera;
pub mod entity;
pub mod light_source;
pub mod object;

use self::camera::Camera;
use self::object::Object3D;
use self::light_source::LightSource;

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
}