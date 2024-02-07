pub mod event;

use colored::Colorize;
use iso8601_timestamp::Timestamp;
use std::{fmt, iter::once, sync::Arc};
use tap::Pipe;
use wgpu::*;
use winit::{
	dpi::{LogicalSize, PhysicalSize},
	event::{Event, WindowEvent},
	event_loop::{EventLoop, EventLoopWindowTarget},
	window::{Window, WindowBuilder},
};

#[derive(Clone, Copy)]
pub enum Level {
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

static LOG_LEVEL_NAMES: [&str; 5] = ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

impl fmt::Display for Level {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.pad(LOG_LEVEL_NAMES[*self as usize])
	}
}

#[allow(unused)]
pub fn log<Message: AsRef<str>>(level: Level, message: Message) {
	let level_string: String = match level {
		Level::Error => format!("{:<5}", level.to_string().red()),
		Level::Warn => format!("{:<5}", level.to_string().yellow()),
		Level::Info => format!("{:<5}", level.to_string().cyan()),
		Level::Debug => format!("{:<5}", level.to_string().purple()),
		Level::Trace => format!("{:<5}", level.to_string().normal()),
	};
	println!(
		"{} {} [APP] {}",
		Timestamp::now_utc().format(),
		level_string,
		message.as_ref()
	);
}

#[allow(unused)]
fn core_log<Message: AsRef<str>>(level: Level, message: Message) {
	let level_string: String = match level {
		Level::Error => format!("{:<5}", level.to_string().red()),
		Level::Warn => format!("{:<5}", level.to_string().yellow()),
		Level::Info => format!("{:<5}", level.to_string().cyan()),
		Level::Debug => format!("{:<5}", level.to_string().purple()),
		Level::Trace => format!("{:<5}", level.to_string().normal()),
	};
	println!(
		"{} {} [HAZEL] {}",
		Timestamp::now_utc().format(),
		level_string,
		message.as_ref()
	);
}

#[macro_export]
macro_rules! trace {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Trace, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Debug, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Info, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Warn, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Error, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_trace {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            core_log(Level::Trace, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_debug {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            core_log(Level::Debug, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_info {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            core_log(Level::Info, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_warn {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            crate::core_log(Level::Warn, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_error {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            hazel::core_log(Level::Error, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[derive(Debug)]
pub enum Error {
	Core,
}

pub struct Context<'a, 'window> {
	application: &'a mut Application<'window>,
	event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a, 'window> Context<'a, 'window> {
	fn new(
		application: &'a mut Application<'window>,
		event_loop: &'a EventLoopWindowTarget<()>,
	) -> Self {
		Context {
			application,
			event_loop,
		}
	}

	pub fn exit(&self) {
		self.event_loop.exit();
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.application.resize(width, height);
	}
}

struct Application<'window> {
	size: PhysicalSize<u32>,
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
			.find(|it| it.is_srgb())
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

	pub fn run(
		mut self,
		mut event_handler: impl FnMut(&mut Context, event::Event),
	) -> Result<(), crate::Error> {
		match self.event_loop {
			None => Err(Error::Core),
			Some(_) => self
				.event_loop
				.take()
				.unwrap()
				.run(move |winit_event, event_loop| {
					let mut context = Context::new(&mut self, event_loop);

					match winit_event {
						Event::WindowEvent {
							window_id: _,
							event: WindowEvent::RedrawRequested,
						} => {
							self.update();
							match self.render() {
								Ok(_) => {}
								Err(SurfaceError::Lost) => {
									self.resize(self.size.width, self.size.height)
								}
								Err(SurfaceError::OutOfMemory) => event_loop.exit(),
								Err(error) => eprintln!("{error:?}"),
							}
						}

						Event::AboutToWait => {
							self.window.request_redraw();
						}

						_ => {
							if let Ok(event) = event::Event::try_from(winit_event) {
								event_handler(&mut context, event);
							}
						}
					}
				})
				.map_err(|_| crate::Error::Core),
		}
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.size = PhysicalSize::new(width, height);
		self.config.width = width;
		self.config.height = height;
		self.surface.configure(&self.device, &self.config);
	}

	pub fn update(&self) {}

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
	fn on_window_resize(&self, context: &mut Context, width: u32, height: u32);
}

pub async fn run(core: impl Core) -> Result<(), crate::Error> {
	let application = Application::new().await;
	application.run(move |context, event| match event {
		event::Event::WindowClose => {
			core.on_window_close(context);
		}

		event::Event::WindowResize { width, height } => {
			core.on_window_resize(context, width, height)
		}

		_ => {}
	})
}
