extern crate cgmath;
#[macro_use]
extern crate gfx;
#[macro_use]
extern crate gfx_macros;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;
extern crate image;
extern crate tobj;

use cgmath::{Deg, Matrix4, Rad, vec3};
use gfx::Device;
use gfx::buffer::Role;
use gfx::traits::FactoryExt;
use glutin::{GlContext};
use glutin::dpi::LogicalSize;
use time::precise_time_s;

mod scene;
mod common;

use common::*;
use scene::Scene;
use scene::entity::Entity3D;

/*
..####....####...#####...######.
.##..##..##..##..##..##..##.....
.##......##..##..#####...####...
.##..##..##..##..##..##..##.....
..####....####...##..##..######.
................................
*/

#[derive(Eq, PartialEq)]
enum CoreState {
	Waiting,
	Running,
	Stopping
}

struct Core {
	state: CoreState,
	scene: Scene,
}
impl Core {
	pub fn new() -> Self {
		Core {
			state: CoreState::Waiting,
			scene: Scene::new_test_scene(),
		}
	}

	pub fn stop(&mut self) {
		self.state = CoreState::Stopping;
	}

	pub fn handle_events(&mut self, event: &glutin::Event) {
		match event {
			glutin::Event::WindowEvent { event, .. } => {
				match event {
					glutin::WindowEvent::CloseRequested => {
						self.stop();
					}
					glutin::WindowEvent::KeyboardInput { input, .. } => {
						if let Some(keycode) = input.virtual_keycode {
							match keycode {
								glutin::VirtualKeyCode::Escape => {
									return;
								},
								glutin::VirtualKeyCode::Up => {
									self.scene.camera.translate(0.0, 0.0, 0.25);
									if let Some(object) = self.scene.objects.get_mut(1) {
										object.rotate(vec3(0.0, 1.0, 0.0), Rad::from(Deg(5.0)));
									}
								},
								glutin::VirtualKeyCode::Down => {
									self.scene.camera.translate(0.0, 0.0, -0.25);
								},
								glutin::VirtualKeyCode::Left => {
									self.scene.camera.translate(0.25, 0.0, 0.0);
								},
								glutin::VirtualKeyCode::Right => {
									self.scene.camera.translate(-0.25, 0.0, 0.0);
								},
								glutin::VirtualKeyCode::E => {
									self.scene.camera.rotate(vec3(0.0, 1.0, 0.0), Rad::from(Deg(5.0)));
								},
								glutin::VirtualKeyCode::Q => {
									self.scene.camera.rotate(vec3(0.0, 1.0, 0.0), Rad::from(Deg(-5.0)));
								},
								_ => {
									// DO NOTHING
								}
							}
						}
					},
					glutin::WindowEvent::Resized(_logical_size) => {

					}
					_ => {
						println!("Unknown glutin::WindowEvent received: {:?}", event);
					},
				}
			},
			_ => {
				println!("Unknown glutin::Event received: {:?}", event);
			}
		}
	}
}

/*
.##..##..######..######..##.......####..
.##..##....##......##....##......##.....
.##..##....##......##....##.......####..
.##..##....##......##....##..........##.
..####.....##....######..######...####..
........................................
*/

fn load_image_resource_view<F, R>(factory: &mut F, rgba_image: &image::RgbaImage) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where F: gfx::Factory<R>, R: gfx::Resources
{
    let (width, height) = rgba_image.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
	let mipmap = gfx::texture::Mipmap::Provided;
	let (_, view) = factory.create_texture_immutable_u8::<ColorFormat>(kind, mipmap, &[&rgba_image]).unwrap();
    view
}

/*
.##...##...####...######..##..##.
.###.###..##..##....##....###.##.
.##.#.##..######....##....##.###.
.##...##..##..##....##....##..##.
.##...##..##..##..######..##..##.
.................................
*/

fn main() {
	let mut core = Core::new();
	let mut events_loop = glutin::EventsLoop::new();

	let width = 800.0;
	let height = 600.0;
	let gl_builder = glutin::ContextBuilder::new().with_vsync(true);
	let builder = glutin::WindowBuilder::new()
		.with_title("Renderer".to_string())
		.with_dimensions(LogicalSize::new(width, height));
	let (window, mut device, mut factory, main_color, main_depth) =
		gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, gl_builder, &events_loop);

	// Despite requesting 640x480, verify the height and width.
	//let LogicalSize { width, height } = window.get_inner_size().unwrap();

	let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

	let pipeline_state = factory
		.create_pipeline_simple(
			include_bytes!("./shaders/basic_150.glslv"),
			include_bytes!("./shaders/basic_150.glslf"),
			pipe::new(),
		)
		.unwrap();

	let empty_vertices: Vec<Vertex> = vec![];
	let empty_indices: Vec<u32> = vec![];
	let (empty_buffer, ..) = factory.create_vertex_buffer_with_slice(
		&empty_vertices.as_slice(), empty_indices.as_slice()
	);

	let no_texture_rgba_image = image::open("data/NO_TEXTURE.png").unwrap().to_rgba();
	let texture = load_image_resource_view(&mut factory, &no_texture_rgba_image);
    let sampler = factory.create_sampler_linear();

	let mut data = pipe::Data {
		vbuf: empty_buffer,
		diffuse_texture: (texture, sampler),
		ps_locals: factory.create_constant_buffer(1),
		out: main_color,
		out_depth: main_depth,
		mvp: Matrix4::from_scale(1.0).into(),
		view_model: Matrix4::from_scale(1.0).into(),
		light_sources_info: factory.create_constant_buffer(250), // 250 = MAX_NUM_LIGHTS
	};

	while core.state != CoreState::Stopping {
		events_loop.poll_events(|event| {
			core.handle_events(&event);
		});

		encoder.clear(&data.out, core.scene.clear_color);
		encoder.clear_depth(&data.out_depth, 1.0);

		// ? Update local buffer (num lights)
		let locals = ForwardLocals {
			eye_position: [core.scene.camera.eye.x, core.scene.camera.eye.y, core.scene.camera.eye.z, 1.0],
			num_lights: core.scene.light_sources.len() as i32,
		};
		encoder.update_buffer(&data.ps_locals, &[locals], 0).unwrap();

		// ? Update light data buffer
		let light_params: Vec<_> = core.scene.light_sources.iter().map(
			|light| LightSourceInfo::new(
				light.translation.into(),
				light.color,
			)
		).collect();
		encoder.update_buffer(&data.light_sources_info, &light_params, 0).unwrap();

		// ? Draw object
		for object in &core.scene.objects {
			let view_model_matrix = core.scene.camera.view * object.model_matrix();
			/* let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
				&object.vertices.as_slice(),
				object.indices.as_slice()
			); */
			let vertex_buffer = factory.create_vertex_buffer(
				&object.vertices.as_slice()
			);
			let slice = gfx::Slice {
				start: 0,
				end: vertex_buffer.len() as u32,
				base_vertex: 0,
				instances: None,
				buffer: gfx::IndexBuffer::Auto
			};
			data.vbuf = vertex_buffer;
			data.mvp = (core.scene.camera.projection * view_model_matrix).into();
			data.view_model = view_model_matrix.into();
			if let Some(object_material_id) = object.material_id {
				if let Some(object_material) = core.scene.materials.get(object_material_id) {
					if let Some(object_diffure_texture) = core.scene.texture_map.get(&object_material.diffuse_texture) {
						data.diffuse_texture.0 = load_image_resource_view(&mut factory, object_diffure_texture);
					}
				}
			}
			encoder.draw(&slice, &pipeline_state, &data);
		}

		encoder.flush(&mut device);
		window.swap_buffers().unwrap();
		device.cleanup();
	}
}

