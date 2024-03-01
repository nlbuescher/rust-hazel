use crate::{Layer, LayerContext};
use imgui::{FontConfig, FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::iter::once;
use wgpu::{
	Color, CommandEncoderDescriptor, Operations, RenderPassColorAttachment, RenderPassDescriptor,
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
		//TODO abstract submitting passes into the rendering pipeline
		if let (Some(ref mut renderer), Some(ref view)) =
			(&mut self.renderer, &context.application.view)
		{
			let imgui = &mut self.imgui;

			imgui.io_mut().update_delta_time(context.delta_time());

			self.platform
				.prepare_frame(imgui.io_mut(), &context.application.window)
				.expect("Failed to prepare frame");

			let ui = imgui.frame();
			ui.show_demo_window(&mut self.show);

			self.platform
				.prepare_render(ui, &context.application.window);

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
		}
	}
}
