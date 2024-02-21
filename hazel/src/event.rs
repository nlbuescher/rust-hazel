use crate::{Position, Size};
use std::fmt::{Display, Formatter};
use winit::{
	dpi::{PhysicalPosition, PhysicalSize},
	event::{Event as WinitEvent, WindowEvent},
};

pub enum Event {
	WindowClose,
	WindowResize { size: Size<u32> },
	WindowFocus,
	WindowLostFocus,
	WindowMoved { offset: Position<i32> },

	AppTick,
	AppUpdate,
	AppRender,

	KeyPressed { key: u32, repeat_count: u32 },
	KeyReleased { key: u32 },

	MouseButtonPressed { button: u32 },
	MouseButtonReleased { button: u32 },
	MouseMoved { position: Position<f32> },
	MouseScrolled { offset: Position<f32> },
}

impl TryFrom<WinitEvent<'_, ()>> for Event {
	type Error = crate::Error;

	fn try_from(winit_event: WinitEvent<()>) -> Result<Self, Self::Error> {
		let hazel_event = match winit_event {
			WinitEvent::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => Event::WindowClose,

			WinitEvent::WindowEvent {
				event: WindowEvent::Resized(PhysicalSize { width, height }),
				..
			} => Event::WindowResize {
				size: Size { width, height },
			},

			WinitEvent::WindowEvent {
				event: WindowEvent::Focused(is_focused),
				..
			} => {
				if is_focused {
					Event::WindowFocus
				} else {
					Event::WindowLostFocus
				}
			}

			WinitEvent::WindowEvent {
				event: WindowEvent::Moved(PhysicalPosition { x, y }),
				..
			} => Event::WindowMoved {
				offset: Position { x, y },
			},

			WinitEvent::WindowEvent {
				event:
					WindowEvent::CursorMoved {
						position: PhysicalPosition { x, y },
						..
					},
				..
			} => Event::MouseMoved {
				position: Position {
					x: x as f32,
					y: y as f32,
				},
			},

			_ => return Err(crate::Error::Event),
		};

		Ok(hazel_event)
	}
}

impl Display for Event {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Event::WindowClose => write!(fmt, "WindowCloseEvent"),

			Event::WindowResize { size } => {
				write!(fmt, "WindowResizeEvent: ({}, {})", size.width, size.height)
			}

			Event::WindowFocus => write!(fmt, "WindowFocusEvent"),

			Event::WindowLostFocus => write!(fmt, "WindowLostFocusEvent"),

			Event::WindowMoved { offset } => {
				write!(fmt, "WindowMovedEvent: ({}, {})", offset.x, offset.y)
			}

			Event::AppTick => write!(fmt, "AppTickEvent"),

			Event::AppUpdate => write!(fmt, "AppUpdateEvent"),

			Event::AppRender => write!(fmt, "AppRenderEvent"),

			Event::KeyPressed { key, repeat_count } => {
				write!(fmt, "KeyPressedEvent: {key} ({repeat_count} repeats)")
			}

			Event::KeyReleased { key } => write!(fmt, "KeyReleasedEvent: {key}"),

			Event::MouseButtonPressed { button } => {
				write!(fmt, "MouseButtonPressedEvent: {button}")
			}

			Event::MouseButtonReleased { button } => {
				write!(fmt, "MouseButtonReleasedEvent: {button}")
			}

			Event::MouseMoved { position } => {
				write!(fmt, "MouseMovedEvent: ({}, {})", position.x, position.y)
			}

			Event::MouseScrolled { offset } => {
				write!(fmt, "MouseScrolledEvent: ({}, {})", offset.x, offset.y)
			}
		}
	}
}
