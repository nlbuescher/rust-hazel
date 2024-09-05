pub mod event;
pub mod layer;
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
	event_loop::EventLoop,
	window::{Window, WindowId},
};
pub use winit::{event::MouseButton, event_loop::ActiveEventLoop, keyboard::Key};

#[allow(unused)]
pub(crate) use crate::log::{core_debug, core_error, core_info, core_trace, core_warn};
use crate::{event::Event, layer::LayerStack};

pub trait Application {
	fn on_key_pressed(&mut self, _event_loop: &ActiveEventLoop, _key: &Key, _is_repeat: bool) {}
	fn on_key_released(&mut self, _event_loop: &ActiveEventLoop, _key: &Key) {}
	fn on_mouse_button_pressed(&self, _event_loop: &ActiveEventLoop, _button: &MouseButton) {}
	fn on_mouse_button_released(&self, _event_loop: &ActiveEventLoop, _button: &MouseButton) {}
	fn on_mouse_moved(&self, _event_loop: &ActiveEventLoop, _x: f32, _y: f32) {}
	fn on_mouse_scrolled(&self, _event_loop: &ActiveEventLoop, _x_offset: f32, _y_offset: f32) {}
	fn on_window_close(&self, event_loop: &ActiveEventLoop) {
		event_loop.exit();
	}
	fn on_window_resize(&self, _event_loop: &ActiveEventLoop, _width: u32, _height: u32) {}
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
	layer_stack: LayerStack,
	state: Option<State<'app>>,
}

impl<'app, App: Application> Context<'app, App> {
	fn new(application: App, layer_setup: impl Fn(&mut LayerStack)) -> Self {
		let mut layer_stack = LayerStack::new();
		layer_setup(&mut layer_stack);
		Context { application, layer_stack, state: None }
	}

	fn on_event(&mut self, event_loop: &ActiveEventLoop, event: &Event) {
		match event {
			Event::KeyPressed { key, is_repeat } => {
				self.application.on_key_pressed(event_loop, key, *is_repeat);
			},
			Event::KeyReleased { key } => {
				self.application.on_key_released(event_loop, key);
			},
			Event::MouseButtonPressed(button) => {
				self.application.on_mouse_button_pressed(event_loop, button);
			},
			Event::MouseButtonReleased(button) => {
				self.application.on_mouse_button_released(event_loop, button);
			},
			Event::MouseMoved { x, y } => {
				self.application.on_mouse_moved(event_loop, *x, *y);
			},
			Event::MouseScrolled { x_offset, y_offset } => {
				self.application.on_mouse_scrolled(event_loop, *x_offset, *y_offset);
			},
			Event::WindowClose => {
				self.application.on_window_close(event_loop);
			},
			Event::WindowResize { width, height } => {
				self.application.on_window_resize(event_loop, *width, *height);
			},
		}

		for layer in &mut self.layer_stack {
			if layer.on_event(event_loop, event) {
				break;
			}
		}
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
		winit_event: WindowEvent,
	) {
		if self.state.is_none() {
			return;
		}
		let state = self.state.as_mut().unwrap();

		// handle winit event
		let event = match winit_event {
			WindowEvent::CloseRequested => Event::WindowClose,

			WindowEvent::KeyboardInput { event, .. } => match event.state {
				ElementState::Pressed => {
					Event::KeyPressed { key: event.logical_key, is_repeat: event.repeat }
				},
				ElementState::Released => Event::KeyReleased { key: event.logical_key },
			},

			WindowEvent::MouseInput { state, button, .. } => match state {
				ElementState::Pressed => Event::MouseButtonPressed(button),
				ElementState::Released => Event::MouseButtonReleased(button),
			},

			WindowEvent::MouseWheel { delta, .. } => match delta {
				MouseScrollDelta::LineDelta(x, y) => {
					let line_scale = 2.0; // hard-code a line pixel size of 2
					Event::MouseScrolled { x_offset: x * line_scale, y_offset: y * line_scale }
				},
				MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
					Event::MouseScrolled { x_offset: x as f32, y_offset: y as f32 }
				},
			},

			WindowEvent::CursorMoved { position: PhysicalPosition { x, y }, .. } => {
				Event::MouseMoved { x: x as f32, y: y as f32 }
			},

			WindowEvent::Resized(PhysicalSize { width, height }) => {
				state.config.width = width;
				state.config.height = height;
				state.surface.configure(&state.device, &state.config);
				state.window.request_redraw();

				Event::WindowResize { width, height }
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
				return;
			},

			_ => {
				return;
			},
		};

		self.on_event(event_loop, &event);
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
pub fn run(
	app: impl Application,
	layer_setup: impl Fn(&mut layer::LayerStack),
) -> Result<(), Error> {
	let mut context = Context::new(app, layer_setup);

	EventLoop::new()?.run_app(&mut context)?;

	Ok(())
}
