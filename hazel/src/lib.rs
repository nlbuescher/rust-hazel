pub mod log;

use std::sync::Arc;

use pollster::FutureExt;
use tap::Pipe;
use wgpu::{
	Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance, Limits, LoadOp,
	MemoryHints, Operations, PowerPreference, Queue, RenderPassColorAttachment,
	RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration,
	TextureViewDescriptor,
};
use winit::{
	application::ApplicationHandler,
	dpi::{PhysicalPosition, PhysicalSize},
	error::EventLoopError,
	event::{ElementState, MouseScrollDelta, WindowEvent},
	event_loop::{ActiveEventLoop, EventLoop},
	window::{Window, WindowId},
};
pub use winit::{event::MouseButton, keyboard::Key};

#[allow(unused)]
pub(crate) use crate::log::{core_debug, core_error, core_info, core_trace, core_warn};

pub trait Application {
	fn on_key_pressed(&self, _key: Key, _is_repeat: bool) {}
	fn on_key_released(&self, _key: Key) {}
	fn on_mouse_moved(&self, _x: f32, _y: f32) {}
	fn on_mouse_scrolled(&self, _x_offset: f32, _y_offset: f32) {}
	fn on_mouse_button_pressed(&self, _button: MouseButton) {}
	fn on_mouse_button_released(&self, _button: MouseButton) {}
	fn on_window_close(&self, event_loop: &ActiveEventLoop) {
		event_loop.exit();
	}
	fn on_window_resize(&self, _width: u32, _height: u32) {}
}

struct State<'app> {
	window: Arc<Window>,
	surface: Surface<'app>,
	device: Device,
	queue: Queue,
	config: SurfaceConfiguration,
}

pub struct Context<'app, App: Application> {
	application: App,
	state: Option<State<'app>>,
}

impl<'app, App: Application> Context<'app, App> {
	fn new(application: App) -> Self {
		Context { application, state: None }
	}
}

impl<'app, App: Application> ApplicationHandler for Context<'app, App> {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window = event_loop
			.create_window(Window::default_attributes())
			.expect("Could not create window!")
			.pipe(Arc::new);

		let instance = Instance::default();
		let surface = instance.create_surface(window.clone()).expect("Could not create surface!");
		let adapter = instance
			.request_adapter(&RequestAdapterOptions {
				power_preference: PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.block_on()
			.expect("Could not find adapter!");
		let (device, queue) = adapter
			.request_device(
				&DeviceDescriptor {
					label: None,
					required_features: Features::empty(),
					required_limits: Limits::default(),
					memory_hints: MemoryHints::default(),
				},
				None,
			)
			.block_on()
			.expect("Could not create device!");

		let config = {
			let scale_factor = window.scale_factor();
			let (frame_width, frame_height) =
				window.inner_size().pipe(|PhysicalSize { width, height }| {
					((width as f64 * scale_factor) as u32, (height as f64 * scale_factor) as u32)
				});
			surface
				.get_default_config(&adapter, frame_width, frame_height)
				.expect("Could not get default config!")
		};
		surface.configure(&device, &config);

		self.state.replace(State { window, surface, device, queue, config });
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		if self.state.is_none() {
			return;
		}
		let state = self.state.as_mut().unwrap();
		match event {
			WindowEvent::CloseRequested => {
				self.application.on_window_close(event_loop);
			},

			WindowEvent::KeyboardInput { event, .. } => match event.state {
				ElementState::Pressed => {
					self.application.on_key_pressed(event.logical_key, event.repeat);
				},
				ElementState::Released => self.application.on_key_released(event.logical_key),
			},

			WindowEvent::MouseInput { state, button, .. } => match state {
				ElementState::Pressed => self.application.on_mouse_button_pressed(button),
				ElementState::Released => self.application.on_mouse_button_released(button),
			},

			WindowEvent::MouseWheel { delta, .. } => match delta {
				MouseScrollDelta::LineDelta(x, y) => {
					let line_scale = 2.0; // hard-code a line pixel size of 2
					self.application.on_mouse_scrolled(x * line_scale, y * line_scale);
				},
				MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
					self.application.on_mouse_scrolled(x as f32, y as f32);
				},
			},

			WindowEvent::CursorMoved { position: PhysicalPosition { x, y }, .. } => {
				self.application.on_mouse_moved(x as f32, y as f32);
			},

			WindowEvent::RedrawRequested => {
				let frame =
					state.surface.get_current_texture().expect("Could not get next texture");
				let view = frame.texture.create_view(&TextureViewDescriptor::default());
				let mut encoder =
					state.device.create_command_encoder(&CommandEncoderDescriptor { label: None });
				{
					let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
						label: None,
						color_attachments: &[Some(RenderPassColorAttachment {
							view: &view,
							resolve_target: None,
							ops: Operations {
								load: LoadOp::Clear(Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 }),
								store: StoreOp::Store,
							},
						})],
						depth_stencil_attachment: None,
						timestamp_writes: None,
						occlusion_query_set: None,
					});
				}
				state.queue.submit(Some(encoder.finish()));
				frame.present();
			},

			WindowEvent::Resized(PhysicalSize { width, height }) => {
				state.config.width = width;
				state.config.height = height;
				state.surface.configure(&state.device, &state.config);
				state.window.request_redraw();

				self.application.on_window_resize(width, height);
			},

			_ => {},
		}
	}
}

#[derive(Debug)]
pub enum Error {
	Unknown(String),
}

impl From<EventLoopError> for Error {
	fn from(value: EventLoopError) -> Self {
		Self::Unknown(format!("{value}"))
	}
}

/// # Errors
pub fn run<App: Application>(app_factory: impl Fn() -> App) -> Result<(), Error> {
	let mut context = Context::new(app_factory());

	EventLoop::new()?.run_app(&mut context)?;

	Ok(())
}
