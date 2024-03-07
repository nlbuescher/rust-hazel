use crate::{
	event::{Event, KeyCode, MouseButton}, Layer, LayerContext, Size
};
use imgui::{FontConfig, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::iter::once;
use wgpu::{
	Color, CommandEncoderDescriptor, Operations, RenderPassColorAttachment, RenderPassDescriptor, TextureViewDescriptor,
};

pub struct ImGuiLayer {
	imgui: imgui::Context,
	platform: WinitPlatform,
	renderer: Option<Renderer>,
	clear_color: Color,
	show: bool,
}

impl ImGuiLayer {
	pub fn new() -> Self {
		let mut imgui = imgui::Context::create();
		let platform = WinitPlatform::init(&mut imgui);

		Self {
			imgui,
			platform,
			renderer: None,
			clear_color: Color {
				r: 1.0,
				g: 0.0,
				b: 1.0,
				a: 1.0,
			},
			show: true,
		}
	}
}

impl Layer for ImGuiLayer {
	fn get_name(&self) -> &str {
		"ImGuiLayer"
	}

	fn on_attach(&mut self, context: &mut LayerContext) {
		let imgui = &mut self.imgui;
		imgui.style_mut().use_dark_colors();

		{
			let io = imgui.io_mut();
			io.backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
			io.backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;

			io.key_map[imgui::Key::Tab as usize] = KeyCode::Tab as u32;
			io.key_map[imgui::Key::LeftArrow as usize] = KeyCode::Left as u32;
			io.key_map[imgui::Key::RightArrow as usize] = KeyCode::Right as u32;
			io.key_map[imgui::Key::UpArrow as usize] = KeyCode::Up as u32;
			io.key_map[imgui::Key::DownArrow as usize] = KeyCode::Down as u32;
			io.key_map[imgui::Key::PageUp as usize] = KeyCode::PageUp as u32;
			io.key_map[imgui::Key::PageDown as usize] = KeyCode::PageDown as u32;
			io.key_map[imgui::Key::Home as usize] = KeyCode::Home as u32;
			io.key_map[imgui::Key::End as usize] = KeyCode::End as u32;
			io.key_map[imgui::Key::Insert as usize] = KeyCode::Insert as u32;
			io.key_map[imgui::Key::Delete as usize] = KeyCode::Delete as u32;
			io.key_map[imgui::Key::Backspace as usize] = KeyCode::Back as u32;
			io.key_map[imgui::Key::Space as usize] = KeyCode::Space as u32;
			io.key_map[imgui::Key::Enter as usize] = KeyCode::Return as u32;
			io.key_map[imgui::Key::Escape as usize] = KeyCode::Escape as u32;
			io.key_map[imgui::Key::A as usize] = KeyCode::A as u32;
			io.key_map[imgui::Key::C as usize] = KeyCode::C as u32;
			io.key_map[imgui::Key::V as usize] = KeyCode::V as u32;
			io.key_map[imgui::Key::X as usize] = KeyCode::X as u32;
			io.key_map[imgui::Key::Y as usize] = KeyCode::Y as u32;
			io.key_map[imgui::Key::Z as usize] = KeyCode::Z as u32;
		}

		let hidpi_factor = context.application.window.scale_factor();

		self.platform.attach_window(
			imgui.io_mut(),
			&context.application.window,
			HiDpiMode::Default,
		);
		imgui.set_ini_filename(None);

		let font_size = (13.0 * hidpi_factor) as f32;
		imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

		imgui.fonts().add_font(&[FontSource::DefaultFontData {
			config: Some(FontConfig {
				oversample_h: 1,
				pixel_snap_h: true,
				size_pixels: font_size,
				..Default::default()
			}),
		}]);

		let renderer_config = RendererConfig {
			texture_format: context.application.config.format,
			..Default::default()
		};

		self.renderer = Some(Renderer::new(
			imgui,
			&context.application.device,
			&context.application.queue,
			renderer_config,
		));
	}

	fn on_update(&mut self, context: &mut LayerContext) {
		if let Some(ref mut renderer) = self.renderer {
			let imgui = &mut self.imgui;

			imgui.io_mut().update_delta_time(context.delta_time());

			self.platform
				.prepare_frame(imgui.io_mut(), &context.application.window)
				.expect("Failed to prepare frame");

			let ui = imgui.frame();
			ui.show_demo_window(&mut self.show);

			self.platform
				.prepare_render(ui, &context.application.window);

			let output = context.application.surface.get_current_texture().expect("NO");
			let view = output
				.texture
				.create_view(&TextureViewDescriptor::default());		

			let mut encoder = context
				.application
				.device
				.create_command_encoder(&CommandEncoderDescriptor { label: None });

			{
				let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
					label: None,
					color_attachments: &[Some(RenderPassColorAttachment {
						view: &view,
						resolve_target: None,
						ops: Operations {
							load: wgpu::LoadOp::Clear(self.clear_color),
							store: true,
						},
					})],
					depth_stencil_attachment: None,
				});

				renderer
					.render(
						imgui.render(),
						&context.application.queue,
						&context.application.device,
						&mut render_pass,
					)
					.expect("Renderer failed");
			}

			context.application.queue.submit(once(encoder.finish()));
			output.present();
		}
	}

	fn on_event(&mut self, context: &mut LayerContext, event: &Event) -> bool {
		let io = self.imgui.io_mut();

		match *event {
			Event::WindowResize {
				size: Size { width, height },
			} => {
				io.display_size[0] = width as f32;
				io.display_size[1] = height as f32;
				io.display_framebuffer_scale[0] = 1.0;
				io.display_framebuffer_scale[1] = 1.0;
				context.application.resize(Size::new(width, height));
			},

			Event::WindowScaleChange(_) => {
				//TODO how to reconfigure scale after moving to a different monitor?
			},

			Event::KeyPressed(key) => {
				io.keys_down[key as usize] = true;

				io.key_ctrl = io.keys_down[KeyCode::LControl as usize]
					|| io.keys_down[KeyCode::RControl as usize];
				io.key_shift = io.keys_down[KeyCode::LShift as usize]
					|| io.keys_down[KeyCode::RShift as usize];
				io.key_alt =
					io.keys_down[KeyCode::LAlt as usize] || io.keys_down[KeyCode::RAlt as usize];
				io.key_super =
					io.keys_down[KeyCode::LWin as usize] || io.keys_down[KeyCode::RWin as usize];
			},

			Event::KeyReleased(key) => {
				io.keys_down[key as usize] = false;

				io.key_ctrl = io.keys_down[KeyCode::LControl as usize]
					|| io.keys_down[KeyCode::RControl as usize];
				io.key_shift = io.keys_down[KeyCode::LShift as usize]
					|| io.keys_down[KeyCode::RShift as usize];
				io.key_alt =
					io.keys_down[KeyCode::LAlt as usize] || io.keys_down[KeyCode::RAlt as usize];
				io.key_super =
					io.keys_down[KeyCode::LWin as usize] || io.keys_down[KeyCode::RWin as usize];
			},

			Event::KeyTyped(character) => {
				self.imgui.io_mut().add_input_character(character);
			},

			Event::MouseButtonPressed(button) => {
				let imgui_button = match button {
					MouseButton::Left => Some(imgui::MouseButton::Left),
					MouseButton::Right => Some(imgui::MouseButton::Right),
					MouseButton::Middle => Some(imgui::MouseButton::Middle),
					MouseButton::Other(_) => None,
				};

				if let Some(imgui_button) = imgui_button {
					self.imgui.io_mut().mouse_down[imgui_button as usize] = true;
				}
			},

			Event::MouseButtonReleased(button) => {
				let imgui_button = match button {
					MouseButton::Left => Some(imgui::MouseButton::Left),
					MouseButton::Right => Some(imgui::MouseButton::Right),
					MouseButton::Middle => Some(imgui::MouseButton::Middle),
					MouseButton::Other(_) => None,
				};

				if let Some(imgui_button) = imgui_button {
					self.imgui.io_mut().mouse_down[imgui_button as usize] = false;
				}
			},

			Event::MouseMoved { position } => {
				self.imgui.io_mut().mouse_pos[0] = position.x;
				self.imgui.io_mut().mouse_pos[1] = position.y;
			},

			Event::MouseScrolled { offset } => {
				self.imgui.io_mut().mouse_wheel_h = offset.x;
				self.imgui.io_mut().mouse_wheel = offset.y;
			},

			_ => {},
		}

		false
	}
}
