mod context;
mod error;
mod layer;
mod log;

pub mod event;

use std::{iter::once, ops::Deref, sync::Arc};
use tap::{Pipe, Tap};
use wgpu::{
	Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
	InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue,
	RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, Surface,
	SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::{
	dpi::LogicalSize,
	event::Event,
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

pub use crate::{
	context::Context,
	error::Error,
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
		layer_stack: LayerStack,
		mut event_handler: impl FnMut(&mut Context, &LayerStack, event::Event) + 'static,
	) -> Result<(), crate::Error> {
		match self.event_loop {
			None => Err(Error::Core),
			Some(_) => self
				.event_loop
				.take()
				.unwrap()
				.run(move |winit_event, _, control_flow| match winit_event {
					Event::RedrawRequested(_) => {
						match self.render() {
							Ok(_) => {}
							Err(SurfaceError::Lost) => self.resize(self.size),
							Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
							Err(error) => eprintln!("{error:?}"),
						}

						let context = Context::new(&mut self, control_flow);

						layer_stack.iter().for_each(|(_, layer)| {
							layer.on_update(&context);
						});

						self.on_update();
					}

					Event::MainEventsCleared => {
						self.window.request_redraw();
					}

					_ => {
						if let Ok(event) = event::Event::try_from(winit_event) {
							let mut context = Context::new(&mut self, control_flow);
							event_handler(&mut context, &layer_stack, event);
						}
					}
				}),
		}
	}

	pub fn resize(&mut self, size: Size<u32>) {
		self.config.width = size.width;
		self.config.height = size.height;
		self.size = size;
		self.surface.configure(&self.device, &self.config);
	}

	pub fn on_update(&mut self) {}

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
	fn on_window_close(&self, context: &mut Context);
	fn on_window_resize(&self, context: &mut Context, size: Size<u32>);
}

pub fn run(core: impl Core + 'static, configure_application: impl FnMut(&mut LayerStack)) {
	let application = Application::new(1280, 720);
	let layer_stack = LayerStack::new().tap_mut(configure_application);

	application
		.run(
			layer_stack,
			move |context, layer_stack, event| match event {
				event::Event::WindowClose => core.on_window_close(context),

				event::Event::WindowResize { size } => core.on_window_resize(context, size),

				_ => {
					for (_, layer) in layer_stack.iter() {
						if layer.on_event(&event) {
							break;
						}
					}
				}
			},
		)
		.expect("something went wrong");
}
