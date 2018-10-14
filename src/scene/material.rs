
use common::*;

use tobj::Material as TObjMaterial;

/*
.######..##......##......##..##..##...##..######..##..##...####...######..######...####...##..##.
...##....##......##......##..##..###.###....##....###.##..##..##....##......##....##..##..###.##.
...##....##......##......##..##..##.#.##....##....##.###..######....##......##....##..##..##.###.
...##....##......##......##..##..##...##....##....##..##..##..##....##......##....##..##..##..##.
.######..######..######...####...##...##..######..##..##..##..##....##....######...####...##..##.
.................................................................................................
.##...##...####...#####...######..##.....
.###.###..##..##..##..##..##......##.....
.##.#.##..##..##..##..##..####....##.....
.##...##..##..##..##..##..##......##.....
.##...##...####...#####...######..######.
.........................................
*/

// ? See: http://paulbourke.net/dataformats/mtl/
pub enum IlluminationModel {
    ConstantColor = 0, // color = Kd
    Model1 = 1, // color = KaIa + Kd { SUM j=1..ls, (N * Lj)Ij }
    Model2 = 2, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks { SUM j=1..ls, ((H*Hj)^Ns)Ij }
    Model3 = 3, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks ({ SUM j=1..ls, ((H*Hj)^Ns)Ij } + Ir) | Ir = (intensity of reflection map) + (ray trace)
    Model4 = 4, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks ({ SUM j=1..ls, ((H*Hj)^Ns)Ij } + Ir)
    Model5 = 5, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks ({ SUM j=1..ls, ((H*Hj)^Ns)Ij Fr(Lj*Hj,Ks,Ns)Ij} + Fr(N*V,Ks,Ns)Ir})
    Model6 = 6, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks ({ SUM j=1..ls, ((H*Hj)^Ns)Ij } + Ir) + (1.0 - Ks) TfIt
    Model7 = 7, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks ({ SUM j=1..ls, ((H*Hj)^Ns)Ij Fr(Lj*Hj,Ks,Ns)Ij} + Fr(N*V,Ks,Ns)Ir}) + (1.0 - Kx)Ft (N*V,(1.0-Ks),Ns)TfIt
    Model8 = 8, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks ({ SUM j=1..ls, ((H*Hj)^Ns)Ij } + Ir) | Ir = (intensity of reflection map)
    Model9 = 9, // color = KaIa + Kd { SUM j=1..ls, (N*Lj)Ij } + Ks ({ SUM j=1..ls, ((H*Hj)^Ns)Ij } + Ir) | Ir = (intensity of reflection map)
    Model10 = 10, // ...
}

/*
.##...##...####...######..######..#####...######...####...##.....
.###.###..##..##....##....##......##..##....##....##..##..##.....
.##.#.##..######....##....####....#####.....##....######..##.....
.##...##..##..##....##....##......##..##....##....##..##..##.....
.##...##..##..##....##....######..##..##..######..##..##..######.
.................................................................
*/

pub struct Material {
	pub name: String,
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub shininess: f32,
    pub dissolve: f32,
    pub optical_density: f32,
    pub ambient_texture: String,
    pub diffuse_texture: String,
    pub specular_texture: String,
    pub normal_texture: String,
    pub dissolve_texture: String,
    pub illumination_model: Option<u8>,
}

impl Material {
    pub fn new(name: String, ambient: [f32; 3], diffuse: [f32; 3], specular: [f32; 3], shininess: f32, dissolve: f32, optical_density: f32,  ambient_texture: String, diffuse_texture: String, specular_texture: String, normal_texture: String, dissolve_texture: String) -> Self {
        Material {
            name,
            ambient: [ambient[0], ambient[1], ambient[2], 1.0],
            diffuse: [diffuse[0], diffuse[1], diffuse[2], 1.0],
            specular: [specular[0], specular[1], specular[2], 1.0],
            shininess: shininess,
            dissolve: dissolve,
            optical_density,
            ambient_texture,
            diffuse_texture,
            specular_texture,
            normal_texture,
            dissolve_texture,
            illumination_model: None,
        }
    }

    pub fn from_tobj_material(tobj_material: &TObjMaterial) -> Self {
        Material::new(
            tobj_material.name.clone(),
            tobj_material.ambient,
            tobj_material.diffuse,
            tobj_material.specular,
            tobj_material.shininess,
            tobj_material.dissolve,
            tobj_material.optical_density,
            tobj_material.ambient_texture.clone(),
            tobj_material.diffuse_texture.clone(),
            tobj_material.specular_texture.clone(),
            tobj_material.normal_texture.clone(),
            tobj_material.dissolve_texture.clone(),
        )
    }

    pub fn from_tobj_materials(tobj_materials: &Vec<TObjMaterial>) -> Vec<Self> {
        tobj_materials.iter().map(|material| Material::from_tobj_material(material)).collect()
    }
}