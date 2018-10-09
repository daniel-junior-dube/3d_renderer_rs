extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;

use cgmath::{perspective, Deg, Matrix4, Point3, Rad, vec3, Vector3};
use gfx::Device;
use gfx::traits::FactoryExt;
use glutin::{GlContext};
use glutin::dpi::LogicalSize;
use time::precise_time_s;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
	vertex Vertex {
		pos: [f32; 3] = "coord3d",
		color: [f32; 3] = "v_color",
	}

	pipeline pipe {
		vbuf: gfx::VertexBuffer<Vertex> = (),
		out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
		out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
		mvp: gfx::Global<[[f32; 4]; 4]> = "mvp",
	}
}

#[derive(Eq, PartialEq)]
enum CoreState {
	Waiting,
	Running,
	Stopping
}

struct Core {
	state: CoreState,
}
impl Core {
	pub fn new() -> Self {
		Core {
			state: CoreState::Waiting,
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
					glutin::WindowEvent::KeyboardInput {
						input:
							glutin::KeyboardInput {
								virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
								..
							},
						..
					} => return,
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

struct Object3D {
	vertices: Vec<Vertex>,
	indices: Vec<u16>,
	translation: Vector3<f32>,
	rotation: Matrix4<f32>,
	scale: f32,
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

struct Camera {
	view: Matrix4<f32>,
	projection: Matrix4<f32>,
}
impl Camera {
	pub fn new(aspect_ratio: f32) -> Self {
		Camera {
			view: Matrix4::look_at(
				Point3::new(0.0, 2.0, 0.0),
				Point3::new(0.0, 0.0, -4.0),
				vec3(0.0, 1.0, 0.0),
			),
			projection: perspective(Deg(45.0), aspect_ratio, 0.1, 10.0),
		}
	}
}

// ? WIP
struct LightSource {
	ambiant: [f32; 4],
	diffuse: [f32; 4],
	specular: [f32; 4],
}
impl LightSource {
	pub fn new() -> Self {
		LightSource {
			ambiant: [0.2, 0.2, 0.2, 1.0],
			diffuse: [0.8, 0.8, 0.8, 1.0],
			specular: [1.0, 1.0, 1.0, 1.0],
		}
	}
}

struct Scene {
	clear_color: [f32; 4],
	objects: Vec<Object3D>,
	camera: Camera,
}
impl Scene {
	pub fn new() -> Self {
		Scene {
			clear_color: [0.0, 0.0, 0.0, 1.0],
			objects: vec![],
			camera: Camera::new(16.0 / 9.0)
		}
	}
}

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
			include_bytes!("triangle_120.glslv"),
			include_bytes!("triangle_120.glslf"),
			pipe::new(),
		)
		.unwrap();

	let mut scene = Scene::new();
	{
		let mut cube = Object3D::new_cube();
		cube.translation = vec3(0.0, 0.0, -4.0);
		scene.objects.push(cube);
	}
	{
		let mut cube = Object3D::new_cube();
		cube.translation = vec3(0.0, 0.0, -10.0);
		scene.objects.push(cube);
	}

	let empty_vertices: Vec<Vertex> = vec![];
	let empty_indices: Vec<u16> = vec![];
	let (empty_buffer, ..) = factory.create_vertex_buffer_with_slice(
		&empty_vertices.as_slice(), empty_indices.as_slice()
	);

	let mut data = pipe::Data {
		vbuf: empty_buffer,
		out: main_color,
		out_depth: main_depth,
		mvp: Matrix4::from_scale(1.0).into(),
	};

	while core.state != CoreState::Stopping {
		events_loop.poll_events(|event| {
			core.handle_events(&event);
		});

		let angle = precise_time_s() * 45.0; // 45 degrees per second
		let anim = Matrix4::from_angle_y(Rad::from(Deg(angle as f32)));

		encoder.clear(&data.out, scene.clear_color);
		encoder.clear_depth(&data.out_depth, 1.0);

		for object in &scene.objects {
			let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
				&object.vertices.as_slice(),
				object.indices.as_slice()
			);
			data.vbuf = vertex_buffer;
			data.mvp = (scene.camera.projection * scene.camera.view * object.model_matrix() * anim).into();
			encoder.draw(&slice, &pipeline_state, &data);
		}

		encoder.flush(&mut device);
		window.swap_buffers().unwrap();
		device.cleanup();
	}
}

