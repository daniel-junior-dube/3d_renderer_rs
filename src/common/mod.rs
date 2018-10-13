
use gfx;

use scene::light_source::LightSource;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_constant_struct!(ForwardPsLocals {
	num_lights: i32 = "u_NumLights",
	eye_position: [f32; 4] = "u_EyePosition",
});

gfx_constant_struct!(LightSourceInfo {
	pos: [f32; 4] = "pos",
	ambiant: [f32; 4] = "ambiant",
	diffuse: [f32; 4] = "diffuse",
	specular: [f32; 4] = "specular",
});

gfx_defines!{
	vertex Vertex {
		pos: [f32; 3] = "coord3d",
		color: [f32; 3] = "v_color",
	}

	pipeline pipe {
		vbuf: gfx::VertexBuffer<Vertex> = (),
		ps_locals: gfx::ConstantBuffer<ForwardPsLocals> = "PsLocals",
		light_sources_info: gfx::ConstantBuffer<LightSourceInfo> = "b_Lights",
		out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
		out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
		projection: gfx::Global<[[f32; 4]; 4]> = "projection",
		model_view: gfx::Global<[[f32; 4]; 4]> = "modelview",
	}
}