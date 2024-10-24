use glium::{framebuffer::SimpleFrameBuffer, glutin::{dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, ContextBuilder}, index::PrimitiveType, program::ProgramCreationInput, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction, UniformsStorage}, vertex::Attribute, Display, DrawParameters, IndexBuffer, Rect, Surface, Texture2d, Vertex, VertexBuffer, VertexFormat};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2(pub f32, pub f32);
impl Vertex for Vec2 {
	fn build_bindings() -> VertexFormat {
		std::borrow::Cow::Owned(vec![(std::borrow::Cow::Borrowed("position"), 0, -1, <(f32, f32)>::get_type(), false)])
	}
}


pub fn load_shader_program(display: &Display, vert_shader: &str, frag_shader: &str) -> glium::Program {
    glium::Program::new(display, ProgramCreationInput::SourceCode {
        vertex_shader: &std::fs::read_to_string(format!("src/shaders/{vert_shader}.vert")).unwrap(),
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: &std::fs::read_to_string(format!("src/shaders/{frag_shader}.frag")).unwrap(),
        transform_feedback_varyings: None,
        outputs_srgb: false,
        uses_point_size: false,
    }).unwrap()
}


static DEFAULT_VERTICES: [Vec2; 4] = [Vec2(-1.0, -1.0), Vec2(1.0, -1.0), Vec2(1.0, 1.0), Vec2(-1.0, 1.0)];
static DEFAULT_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];


const SIM_SIZE: u32 = 256;


fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new()
		.with_inner_size(PhysicalSize::new(1024.0, 1024.0));
	let cb = ContextBuilder::new();
	let display = Display::new(wb, cb, &event_loop).unwrap();
	let PhysicalSize { mut width, mut height } = display.gl_window().window().inner_size();
	
	let default_program = load_shader_program(&display, "default", "default");
	let mut program = load_shader_program(&display, "default", "main");
	
	let default_vertex_buffer = VertexBuffer::new(&display, &DEFAULT_VERTICES).unwrap();
	let default_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &DEFAULT_INDICES).unwrap();
	
	
	let state_texture_a = Texture2d::empty(&display, SIM_SIZE, SIM_SIZE).unwrap();
	let state_texture_b = Texture2d::empty(&display, SIM_SIZE, SIM_SIZE).unwrap();
	let mut a_current = true;
	
	
	let mut mouse_l_down = false;
	let mut mouse_r_down = false;
	let mut mouse_m_down = false;
	let mut ctrl_pressed = false;
	let mut previous_mouse_position: Option<PhysicalPosition<f64>> = None;
	
	
	event_loop.run(move |ev, _, control_flow| {
		match ev {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::KeyboardInput { input, .. } => {
					if let Some(code) = input.virtual_keycode {
						let state = match input.state {
							ElementState::Pressed => true,
							ElementState::Released => false
						};
						match code {
							
							VirtualKeyCode::LControl | VirtualKeyCode::RControl => ctrl_pressed = state,
							
							VirtualKeyCode::R => if state {
								if ctrl_pressed {
									program = load_shader_program(&display, "default", "main");
								} else {
									SimpleFrameBuffer::new(&display, &state_texture_a).unwrap().clear_color(0.0, 0.0, 0.0, 0.0);
									SimpleFrameBuffer::new(&display, &state_texture_b).unwrap().clear_color(0.0, 0.0, 0.0, 0.0);
									a_current = true;
								}
							}
							
							_ => ()
						}
						
					}
				}
				WindowEvent::MouseInput { state, button, .. } => {
					let pressed = match state {
						ElementState::Pressed => true,
						ElementState::Released => false,
					};
					
					match button {
						MouseButton::Left => mouse_l_down = pressed,
						MouseButton::Right => mouse_r_down = pressed,
						MouseButton::Middle => mouse_m_down = pressed,
						_ => ()
					}
					
					if mouse_l_down || mouse_r_down || mouse_m_down {
						let color = (
							if mouse_l_down {1.0} else {0.0},
							if mouse_r_down {1.0} else {0.0},
							if mouse_m_down {1.0} else {0.0},
						);
						
						if let Some(position) = previous_mouse_position {
							(if a_current {&state_texture_a} else {&state_texture_b}).write(Rect {
								left: (position.x / width as f64 * SIM_SIZE as f64 - 5.0).clamp(0.0, SIM_SIZE as f64 - 11.0) as u32,
								bottom: ((1.0 - position.y / height as f64) * SIM_SIZE as f64 - 5.0).clamp(0.0, SIM_SIZE as f64 - 11.0) as u32,
								width: 11,
								height: 11,
							}, (0..=10).map(|x| (0..=10).map(|y| if (x-5)*(x-5) + (y-5)*(y-5) <= 30 {color} else {(0.0, 0.0, 0.0)}).collect()).collect::<Vec<_>>());
						}
					}
				}
				WindowEvent::CursorMoved { position, .. } => {
					if mouse_l_down || mouse_r_down || mouse_m_down {
						let color = (
							if mouse_l_down {1.0} else {0.0},
							if mouse_r_down {1.0} else {0.0},
							if mouse_m_down {1.0} else {0.0},
						);
						
						if let Some(position) = previous_mouse_position {
							(if a_current {&state_texture_a} else {&state_texture_b}).write(Rect {
								left: (position.x / width as f64 * SIM_SIZE as f64 - 5.0).clamp(0.0, SIM_SIZE as f64 - 11.0) as u32,
								bottom: ((1.0 - position.y / height as f64) * SIM_SIZE as f64 - 5.0).clamp(0.0, SIM_SIZE as f64 - 11.0) as u32,
								width: 11,
								height: 11,
							}, (0..=10).map(|x| (0..=10).map(|y| if (x-5)*(x-5) + (y-5)*(y-5) <= 30 {color} else {(0.0, 0.0, 0.0)}).collect()).collect::<Vec<_>>());
						}
					}
					
					previous_mouse_position = Some(position);
				}
				WindowEvent::Resized(new_size) => {
					width = new_size.width;
					height = new_size.height;
					//main_texture = Texture2d::empty(&display, width, height).unwrap();
				}
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
				}
				_ => ()
			}
			Event::RedrawEventsCleared => {
				display.gl_window().window().request_redraw();
			}
			Event::RedrawRequested(_) => {
				
				let (current_state, next_state) = match a_current {
					true => (&state_texture_a, &state_texture_b),
					false => (&state_texture_b, &state_texture_a)
				};
				
				let mut target = SimpleFrameBuffer::new(&display, next_state).unwrap();
				
				target.draw(&default_vertex_buffer, &default_index_buffer, &program, &UniformsStorage::
					 new("sim_size", SIM_SIZE as i32)
					.add("state", Sampler(current_state, SamplerBehavior {
						wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
						minify_filter: MinifySamplerFilter::Nearest,
						magnify_filter: MagnifySamplerFilter::Nearest,
						depth_texture_comparison: None,
						max_anisotropy: 1,
					}))
				, &DrawParameters::default()).unwrap();
				
				
				
				let mut target = display.draw();
				target.clear_color(0.0, 0.0, 0.0, 0.0);
				
				target.draw(&default_vertex_buffer, &default_index_buffer, &default_program, &UniformsStorage::
					 new("screen_texture", next_state)
				, &DrawParameters::default()).unwrap();
				
				target.finish().unwrap();
				
				
				a_current = !a_current;
			}
			_ => ()
		}
	});
}
