mod context;
mod error;
mod layer;
mod log;

pub mod event;

use std::{iter::once, sync::Arc};
use tap::{Pipe, Tap};
use wgpu::{
	Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
	InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue,
	RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
	SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

pub use crate::{
	context::Context,
	error::Error,
	layer::{Layer, LayerStack},
	log::{log, LogLevel},
};
pub use winit::dpi::{PhysicalPosition as Position, PhysicalSize as Size};

pub struct Application<'window> {
	size: Size<u32>,
	config: SurfaceConfiguration,
	queue: Queue,
	device: Device,
	surface: Surface<'window>,
	window: Arc<Window>,
	event_loop: Option<EventLoop<()>>,
}

impl<'window> Application<'window> {
	pub async fn new() -> Self {
		let event_loop = EventLoop::new().unwrap();

		let window = WindowBuilder::new()
			.with_title("Hazel Engine")
			.with_inner_size(LogicalSize::new(1280, 720))
			.build(&event_loop)
			.unwrap()
			.pipe(Arc::new);

		let size = window.inner_size();

		let instance = Instance::new(InstanceDescriptor::default());

		let surface = instance.create_surface(window.clone()).unwrap();

		let adapter = instance
			.request_adapter(&RequestAdapterOptions {
				power_preference: PowerPreference::default(),
				force_fallback_adapter: false,
				compatible_surface: Some(&surface),
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(
				&DeviceDescriptor {
					label: None,
					required_features: Features::empty(),
					required_limits: Limits::default(),
				},
				None,
			)
			.await
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
			desired_maximum_frame_latency: 2,
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
				.run(move |winit_event, event_loop| match winit_event {
					Event::WindowEvent {
						event: WindowEvent::RedrawRequested,
						..
					} => {
						match self.render() {
							Ok(_) => {}
							Err(SurfaceError::Lost) => self.resize(self.size),
							Err(SurfaceError::OutOfMemory) => event_loop.exit(),
							Err(error) => eprintln!("{error:?}"),
						}

						let context = Context::new(&mut self, event_loop);

						layer_stack.iter().for_each(|(_, layer)| {
							layer.on_update(&context);
						});

						self.on_update();
					}

					Event::AboutToWait => {
						self.window.request_redraw();
					}

					_ => {
						if let Ok(event) = event::Event::try_from(winit_event) {
							let mut context = Context::new(&mut self, event_loop);
							event_handler(&mut context, &layer_stack, event);
						}
					}
				})
				.map_err(|_| Error::Core),
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
						store: StoreOp::Store,
					},
				})],
				depth_stencil_attachment: None,
				timestamp_writes: None,
				occlusion_query_set: None,
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

pub async fn run(
	core: impl Core + 'static,
	configure_application: impl FnMut(&mut LayerStack),
) -> Result<(), crate::Error> {
	let application = Application::new().await;
	let layer_stack = LayerStack::new().tap_mut(configure_application);

	application.run(
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
}
