
use gfx;
use cgmath::{Deg, Matrix4, vec3, Vector3, Rad};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_constant_struct!(ForwardLocals {
	eye_position: [f32; 4] = "u_EyePosition",
	num_lights: i32 = "u_NumLights",
});

#[derive(ConstantBuffer, Copy, Clone)]
pub struct LightSourceInfo {
	pos: [f32; 4],
	color: [f32; 4],
}

impl LightSourceInfo {
	pub fn new(pos: [f32; 3], color: [f32; 3]) -> Self {
		LightSourceInfo {
			pos: [pos[0], pos[1], pos[2], 1.0],
			color: [color[0], color[1], color[2], 1.0],
		}
	}
}

#[derive(VertexData)]
pub struct Vertex {
    v_pos: [f32; 4],
	v_color: [f32; 3],
	v_normal: [f32; 4],
	v_uv: [f32; 2],
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex {
			v_pos: [pos[0], pos[1], pos[2], 1.0],
			v_color: color,
			v_normal: [normal[0], normal[1], normal[2], 1.0],
			v_uv: uv,
		}
    }

    pub fn new_default(pos: [f32; 3]) -> Vertex {
		Vertex::new(
			pos,
			[1.0; 3],
			[0.0; 3],
			[0.0; 2]
		)
    }
}

gfx_defines!{
	pipeline pipe {
		vbuf: gfx::VertexBuffer<Vertex> = (),
		diffuse_texture: gfx::TextureSampler<[f32; 4]> = "u_DiffuseTexture",
		ps_locals: gfx::ConstantBuffer<ForwardLocals> = "Locals",
		light_sources_info: gfx::ConstantBuffer<LightSourceInfo> = "b_lights",
		out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
		out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
		mvp: gfx::Global<[[f32; 4]; 4]> = "u_MVP",
		view_model: gfx::Global<[[f32; 4]; 4]> = "u_ViewModel",
	}
}
