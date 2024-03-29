mod context;
mod error;
mod imgui;
mod layer;
mod log;

pub mod event;

use event::Event;
use std::{
	iter::once,
	ops::Deref,
	sync::Arc,
	time::{Duration, Instant},
};
use tap::Pipe;
use wgpu::{
	Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
	InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue,
	RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, Surface,
	SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::{
	dpi::LogicalSize,
	event::{Event as WinitEvent, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

#[allow(unused)]
pub(crate) use crate::log::{core_debug, core_error, core_info, core_log, core_trace, core_warn};

pub use crate::{
	context::{EventContext, LayerContext},
	error::Error,
	imgui::ImGuiLayer,
	layer::{Layer, LayerStack},
	log::{log, LogLevel},
};
pub use winit::dpi::{PhysicalPosition as Position, PhysicalSize as Size};

pub struct Application {
	size: Size<u32>,
	config: SurfaceConfiguration,
	queue: Queue,
	device: Device,
	surface: Surface,
	window: Arc<Window>,
	event_loop: Option<EventLoop<()>>,
}

impl Application {
	pub fn new(width: u32, height: u32) -> Self {
		let event_loop = EventLoop::new();

		let window = WindowBuilder::new()
			.with_title("Hazel Engine")
			.with_inner_size(LogicalSize::new(width, height))
			.build(&event_loop)
			.unwrap()
			.pipe(Arc::new);

		let size = window.inner_size();

		let instance = Instance::new(InstanceDescriptor::default());

		let surface = unsafe { instance.create_surface(window.deref()).unwrap() };

		let adapter = instance
			.request_adapter(&RequestAdapterOptions {
				power_preference: PowerPreference::default(),
				force_fallback_adapter: false,
				compatible_surface: Some(&surface),
			})
			.pipe(pollster::block_on)
			.unwrap();

		let (device, queue) = adapter
			.request_device(
				&DeviceDescriptor {
					label: None,
					features: Features::empty(),
					limits: Limits::default(),
				},
				None,
			)
			.pipe(pollster::block_on)
			.unwrap();

		let surface_caps = surface.get_capabilities(&adapter);

		let surface_format = surface_caps
			.formats
			.iter()
			.copied()
			.find(TextureFormat::is_srgb)
			.unwrap_or(surface_caps.formats[0]);

		let config = SurfaceConfiguration {
			usage: TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
		};

		Self {
			size,
			config,
			queue,
			device,
			surface,
			window,
			event_loop: Some(event_loop),
		}
	}

	fn run(
		mut self,
		mut layer_stack: LayerStack,
		mut event_handler: impl FnMut(&mut EventContext, Event) + 'static,
	) -> Result<(), crate::Error> {
		match self.event_loop {
			None => Err(Error::Core),
			Some(_) => {
				let mut last_update = Instant::now();

				self.event_loop
					.take()
					.unwrap()
					.run(move |mut winit_event, _, control_flow| {
						let now = Instant::now();

						match winit_event {
							WinitEvent::RedrawRequested(_) => {
								//TODO rendering is overlapping as if two framebuffers are rendered for every frame.
								// how can we queue multiple render passes into a single output?
								match self.render() {
									Ok(_) => {},
									Err(SurfaceError::Lost) => self.resize(self.size),
									Err(SurfaceError::OutOfMemory) => {
										*control_flow = ControlFlow::Exit
									},
									Err(error) => eprintln!("{error:?}"),
								}

								for i in 0..layer_stack.len() {
									let (_, ref mut layer) = layer_stack[i];

									let mut context = LayerContext::new(
										now - last_update,
										&mut self,
										Some(control_flow),
									);

									layer.on_update(&mut context);
								}
							},

							WinitEvent::MainEventsCleared => {
								self.window.request_redraw();
							},

							_ => {
								if let WinitEvent::WindowEvent {
									event:
										WindowEvent::CursorMoved {
											ref mut position, ..
										},
									..
								} = winit_event
								{
									let scale = self.window.scale_factor();
									position.x /= scale;
									position.y /= scale;
								}

								if let Ok(event) = Event::try_from(winit_event) {
									let mut context = EventContext::new(
										now - last_update,
										&mut self,
										&mut layer_stack,
										Some(control_flow),
									);

									event_handler(&mut context, event);
								}
							},
						}

						last_update = now;
					})
			},
		}
	}

	pub fn resize(&mut self, size: Size<u32>) {
		self.config.width = size.width;
		self.config.height = size.height;
		self.size = size;
		self.surface.configure(&self.device, &self.config);
	}

	pub fn render(&mut self) -> Result<(), SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output
			.texture
			.create_view(&TextureViewDescriptor::default());
		let mut encoder = self
			.device
			.create_command_encoder(&CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		{
			let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear(Color {
							r: 1.0,
							g: 0.0,
							b: 1.0,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});
		}

		self.queue.submit(once(encoder.finish()));

		output.present();

		Ok(())
	}
}

pub trait Core {
	fn on_window_close(&self, context: &mut EventContext);
	fn on_window_resize(&self, context: &mut EventContext, size: Size<u32>);
}

pub fn run(core: impl Core + 'static, mut configure: impl FnMut(&mut EventContext)) {
	let mut application = Application::new(1280, 720);
	let mut layer_stack = LayerStack::new();
	configure(&mut EventContext::new(
		Duration::ZERO,
		&mut application,
		&mut layer_stack,
		None,
	));

	application
		.run(layer_stack, move |context, event| match event {
			Event::WindowClose => core.on_window_close(context),

			Event::WindowResize { size } => core.on_window_resize(context, size),

			_ => {
				for i in 0..context.layer_stack.len() {
					let (_, ref mut layer) = context.layer_stack[i];

					if layer.on_event(&mut context.layer_context, &event) {
						break;
					}
				}
			},
		})
		.expect("something went wrong");
}
