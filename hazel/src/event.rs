use winit::{
	dpi::PhysicalSize,
	event::{Event as WinitEvent, WindowEvent},
};

pub enum Event {
	WindowClose,
	WindowResize { width: u32, height: u32 },
	WindowFocus,
	WindowLostFocus,
	WindowMoved,

	AppTick,
	AppUpdate,
	AppRender,

	KeyPressed { key: u32, repeat_count: u32 },
	KeyReleased { key: u32 },

	MouseButtonPressed { button: u32 },
	MouseButtonReleased { button: u32 },
	MouseMoved { x: f32, y: f32 },
	MouseScrolled { x_offset: f32, y_offset: f32 },
}

impl TryFrom<WinitEvent<()>> for Event {
	type Error = crate::Error;

	fn try_from(winit_event: WinitEvent<()>) -> Result<Self, Self::Error> {
		let hazel_event = match winit_event {
			WinitEvent::WindowEvent {
				window_id: _,
				event: WindowEvent::Resized(PhysicalSize { width, height }),
			} => Event::WindowResize { width, height },

			WinitEvent::WindowEvent {
				window_id: _,
				event: WindowEvent::CloseRequested,
			} => Event::WindowClose,

			_ => return Err(crate::Error::Core),
		};

		Ok(hazel_event)
	}
}
