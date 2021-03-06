
use cgmath::{Rad, Deg, Matrix4, vec3, Vector3};

/*
.######..##..##..######..######..######..##..##..######..#####..
.##......###.##....##......##......##.....####......##...##..##.
.####....##.###....##......##......##......##......###...##..##.
.##......##..##....##......##......##......##........##..##..##.
.######..##..##....##....######....##......##....#####...#####..
................................................................
*/

pub trait Entity3D {
	fn translate(&mut self, x: f32, y: f32, z: f32);
	fn rotate(&mut self, axis: Vector3<f32>, angle: Rad<f32>);
}