
use gfx;
use cgmath::{Deg, Matrix4, vec3, Vector3, Rad};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_constant_struct!(ForwardLocals {
	eye_position: [f32; 4] = "u_EyePosition",
	num_lights: i32 = "u_NumLights",
});

gfx_constant_struct!(LightSourceInfo {
	pos: [f32; 4] = "pos",
	ambiant: [f32; 4] = "ambiant",
	diffuse: [f32; 4] = "diffuse",
	specular: [f32; 4] = "specular",
});

gfx_vertex_struct!( Vertex {
    pos: [f32; 4] = "v_pos",
	color: [f32; 4] = "v_color",
	normal: [f32; 4] = "v_normal",
});

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 3], normal: [f32; 3]) -> Vertex {
        Vertex {
			pos: [pos[0], pos[1], pos[2], 1.0],
			color: [color[0], color[1], color[2], 1.0],
			normal: [normal[0], normal[1], normal[2], 1.0]
		}
    }

    pub fn new_default(pos: [f32; 3]) -> Vertex {
		Vertex::new(
			pos,
			[1.0; 3],
			[0.0; 3]
		)
    }
}

gfx_defines!{
	pipeline pipe {
		vbuf: gfx::VertexBuffer<Vertex> = (),
		ps_locals: gfx::ConstantBuffer<ForwardLocals> = "Locals",
		light_sources_info: gfx::ConstantBuffer<LightSourceInfo> = "b_Lights",
		out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
		out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
		mvp: gfx::Global<[[f32; 4]; 4]> = "u_MVP",
		view_model: gfx::Global<[[f32; 4]; 4]> = "u_ViewModel",
	}
}
