use crate::{Position, Size};
use std::fmt::{Display, Formatter};
use winit::{
	dpi::{PhysicalPosition, PhysicalSize},
	event::{
		ElementState, Event as WinitEvent, KeyboardInput, MouseButton as WinitMouseButton,
		MouseScrollDelta, VirtualKeyCode, WindowEvent,
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
	Left,
	Right,
	Middle,
	Other(u16),
}

impl Display for MouseButton {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Left => write!(fmt, "Left"),
			Self::Right => write!(fmt, "Right"),
			Self::Middle => write!(fmt, "Middle"),
			Self::Other(button) => write!(fmt, "Other({button})"),
		}
	}
}

impl From<winit::event::MouseButton> for MouseButton {
	fn from(value: winit::event::MouseButton) -> Self {
		match value {
			WinitMouseButton::Left => Self::Left,
			WinitMouseButton::Right => Self::Right,
			WinitMouseButton::Middle => Self::Middle,
			WinitMouseButton::Other(button) => Self::Other(button),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyCode {
	/// The '1' key over the letters.
	Key1,
	/// The '2' key over the letters.
	Key2,
	/// The '3' key over the letters.
	Key3,
	/// The '4' key over the letters.
	Key4,
	/// The '5' key over the letters.
	Key5,
	/// The '6' key over the letters.
	Key6,
	/// The '7' key over the letters.
	Key7,
	/// The '8' key over the letters.
	Key8,
	/// The '9' key over the letters.
	Key9,
	/// The '0' key over the 'O' and 'P' keys.
	Key0,

	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,

	/// The Escape key, next to F1.
	Escape,

	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,

	/// Print Screen/SysRq.
	Snapshot,
	/// Scroll Lock.
	Scroll,
	/// Pause/Break key, next to Scroll lock.
	Pause,

	/// `Insert`, next to Backspace.
	Insert,
	Home,
	Delete,
	End,
	PageDown,
	PageUp,

	Left,
	Up,
	Right,
	Down,

	/// The Backspace key, right over Enter.
	// TODO: rename
	Back,
	/// The Enter key.
	Return,
	/// The space bar.
	Space,

	/// The "Compose" key on Linux.
	Compose,

	Caret,

	Numlock,
	Numpad0,
	Numpad1,
	Numpad2,
	Numpad3,
	Numpad4,
	Numpad5,
	Numpad6,
	Numpad7,
	Numpad8,
	Numpad9,
	NumpadAdd,
	NumpadDivide,
	NumpadDecimal,
	NumpadComma,
	NumpadEnter,
	NumpadEquals,
	NumpadMultiply,
	NumpadSubtract,

	AbntC1,
	AbntC2,
	Apostrophe,
	Apps,
	Asterisk,
	At,
	Ax,
	Backslash,
	Calculator,
	Capital,
	Colon,
	Comma,
	Convert,
	Equals,
	Grave,
	Kana,
	Kanji,
	LAlt,
	LBracket,
	LControl,
	LShift,
	LWin,
	Mail,
	MediaSelect,
	MediaStop,
	Minus,
	Mute,
	MyComputer,
	// also called "Next"
	NavigateForward,
	// also called "Prior"
	NavigateBackward,
	NextTrack,
	NoConvert,
	OEM102,
	Period,
	PlayPause,
	Plus,
	Power,
	PrevTrack,
	RAlt,
	RBracket,
	RControl,
	RShift,
	RWin,
	Semicolon,
	Slash,
	Sleep,
	Stop,
	Sysrq,
	Tab,
	Underline,
	Unlabeled,
	VolumeDown,
	VolumeUp,
	Wake,
	WebBack,
	WebFavorites,
	WebForward,
	WebHome,
	WebRefresh,
	WebSearch,
	WebStop,
	Yen,
	Copy,
	Paste,
	Cut,
}

impl Display for KeyCode {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Key1 => write!(f, "Key1"),
			Self::Key2 => write!(f, "Key2"),
			Self::Key3 => write!(f, "Key3"),
			Self::Key4 => write!(f, "Key4"),
			Self::Key5 => write!(f, "Key5"),
			Self::Key6 => write!(f, "Key6"),
			Self::Key7 => write!(f, "Key7"),
			Self::Key8 => write!(f, "Key8"),
			Self::Key9 => write!(f, "Key9"),
			Self::Key0 => write!(f, "Key0"),
			Self::A => write!(f, "A"),
			Self::B => write!(f, "B"),
			Self::C => write!(f, "C"),
			Self::D => write!(f, "D"),
			Self::E => write!(f, "E"),
			Self::F => write!(f, "F"),
			Self::G => write!(f, "G"),
			Self::H => write!(f, "H"),
			Self::I => write!(f, "I"),
			Self::J => write!(f, "J"),
			Self::K => write!(f, "K"),
			Self::L => write!(f, "L"),
			Self::M => write!(f, "M"),
			Self::N => write!(f, "N"),
			Self::O => write!(f, "O"),
			Self::P => write!(f, "P"),
			Self::Q => write!(f, "Q"),
			Self::R => write!(f, "R"),
			Self::S => write!(f, "S"),
			Self::T => write!(f, "T"),
			Self::U => write!(f, "U"),
			Self::V => write!(f, "V"),
			Self::W => write!(f, "W"),
			Self::X => write!(f, "X"),
			Self::Y => write!(f, "Y"),
			Self::Z => write!(f, "Z"),
			Self::Escape => write!(f, "Escape"),
			Self::F1 => write!(f, "F1"),
			Self::F2 => write!(f, "F2"),
			Self::F3 => write!(f, "F3"),
			Self::F4 => write!(f, "F4"),
			Self::F5 => write!(f, "F5"),
			Self::F6 => write!(f, "F6"),
			Self::F7 => write!(f, "F7"),
			Self::F8 => write!(f, "F8"),
			Self::F9 => write!(f, "F9"),
			Self::F10 => write!(f, "F10"),
			Self::F11 => write!(f, "F11"),
			Self::F12 => write!(f, "F12"),
			Self::F13 => write!(f, "F13"),
			Self::F14 => write!(f, "F14"),
			Self::F15 => write!(f, "F15"),
			Self::F16 => write!(f, "F16"),
			Self::F17 => write!(f, "F17"),
			Self::F18 => write!(f, "F18"),
			Self::F19 => write!(f, "F19"),
			Self::F20 => write!(f, "F20"),
			Self::F21 => write!(f, "F21"),
			Self::F22 => write!(f, "F22"),
			Self::F23 => write!(f, "F23"),
			Self::F24 => write!(f, "F24"),
			Self::Snapshot => write!(f, "Snapshot"),
			Self::Scroll => write!(f, "Scroll"),
			Self::Pause => write!(f, "Pause"),
			Self::Insert => write!(f, "Insert"),
			Self::Home => write!(f, "Home"),
			Self::Delete => write!(f, "Delete"),
			Self::End => write!(f, "End"),
			Self::PageDown => write!(f, "PageDown"),
			Self::PageUp => write!(f, "PageUp"),
			Self::Left => write!(f, "Left"),
			Self::Up => write!(f, "Up"),
			Self::Right => write!(f, "Right"),
			Self::Down => write!(f, "Down"),
			Self::Back => write!(f, "Back"),
			Self::Return => write!(f, "Return"),
			Self::Space => write!(f, "Space"),
			Self::Compose => write!(f, "Compose"),
			Self::Caret => write!(f, "Caret"),
			Self::Numlock => write!(f, "Numlock"),
			Self::Numpad0 => write!(f, "Numpad0"),
			Self::Numpad1 => write!(f, "Numpad1"),
			Self::Numpad2 => write!(f, "Numpad2"),
			Self::Numpad3 => write!(f, "Numpad3"),
			Self::Numpad4 => write!(f, "Numpad4"),
			Self::Numpad5 => write!(f, "Numpad5"),
			Self::Numpad6 => write!(f, "Numpad6"),
			Self::Numpad7 => write!(f, "Numpad7"),
			Self::Numpad8 => write!(f, "Numpad8"),
			Self::Numpad9 => write!(f, "Numpad9"),
			Self::NumpadAdd => write!(f, "NumpadAdd"),
			Self::NumpadDivide => write!(f, "NumpadDivide"),
			Self::NumpadDecimal => write!(f, "NumpadDecimal"),
			Self::NumpadComma => write!(f, "NumpadComma"),
			Self::NumpadEnter => write!(f, "NumpadEnter"),
			Self::NumpadEquals => write!(f, "NumpadEquals"),
			Self::NumpadMultiply => write!(f, "NumpadMultiply"),
			Self::NumpadSubtract => write!(f, "NumpadSubtract"),
			Self::AbntC1 => write!(f, "AbntC1"),
			Self::AbntC2 => write!(f, "AbntC2"),
			Self::Apostrophe => write!(f, "Apostrophe"),
			Self::Apps => write!(f, "Apps"),
			Self::Asterisk => write!(f, "Asterisk"),
			Self::At => write!(f, "At"),
			Self::Ax => write!(f, "Ax"),
			Self::Backslash => write!(f, "Backslash"),
			Self::Calculator => write!(f, "Calculator"),
			Self::Capital => write!(f, "Capital"),
			Self::Colon => write!(f, "Colon"),
			Self::Comma => write!(f, "Comma"),
			Self::Convert => write!(f, "Convert"),
			Self::Equals => write!(f, "Equals"),
			Self::Grave => write!(f, "Grave"),
			Self::Kana => write!(f, "Kana"),
			Self::Kanji => write!(f, "Kanji"),
			Self::LAlt => write!(f, "LAlt"),
			Self::LBracket => write!(f, "LBracket"),
			Self::LControl => write!(f, "LControl"),
			Self::LShift => write!(f, "LShift"),
			Self::LWin => write!(f, "LWin"),
			Self::Mail => write!(f, "Mail"),
			Self::MediaSelect => write!(f, "MediaSelect"),
			Self::MediaStop => write!(f, "MediaStop"),
			Self::Minus => write!(f, "Minus"),
			Self::Mute => write!(f, "Mute"),
			Self::MyComputer => write!(f, "MyComputer"),
			Self::NavigateForward => write!(f, "NavigateForward"),
			Self::NavigateBackward => write!(f, "NavigateBackward"),
			Self::NextTrack => write!(f, "NextTrack"),
			Self::NoConvert => write!(f, "NoConvert"),
			Self::OEM102 => write!(f, "OEM102"),
			Self::Period => write!(f, "Period"),
			Self::PlayPause => write!(f, "PlayPause"),
			Self::Plus => write!(f, "Plus"),
			Self::Power => write!(f, "Power"),
			Self::PrevTrack => write!(f, "PrevTrack"),
			Self::RAlt => write!(f, "RAlt"),
			Self::RBracket => write!(f, "RBracket"),
			Self::RControl => write!(f, "RControl"),
			Self::RShift => write!(f, "RShift"),
			Self::RWin => write!(f, "RWin"),
			Self::Semicolon => write!(f, "Semicolon"),
			Self::Slash => write!(f, "Slash"),
			Self::Sleep => write!(f, "Sleep"),
			Self::Stop => write!(f, "Stop"),
			Self::Sysrq => write!(f, "Sysrq"),
			Self::Tab => write!(f, "Tab"),
			Self::Underline => write!(f, "Underline"),
			Self::Unlabeled => write!(f, "Unlabeled"),
			Self::VolumeDown => write!(f, "VolumeDown"),
			Self::VolumeUp => write!(f, "VolumeUp"),
			Self::Wake => write!(f, "Wake"),
			Self::WebBack => write!(f, "WebBack"),
			Self::WebFavorites => write!(f, "WebFavorites"),
			Self::WebForward => write!(f, "WebForward"),
			Self::WebHome => write!(f, "WebHome"),
			Self::WebRefresh => write!(f, "WebRefresh"),
			Self::WebSearch => write!(f, "WebSearch"),
			Self::WebStop => write!(f, "WebStop"),
			Self::Yen => write!(f, "Yen"),
			Self::Copy => write!(f, "Copy"),
			Self::Paste => write!(f, "Paste"),
			Self::Cut => write!(f, "Cut"),
		}
	}
}

impl From<VirtualKeyCode> for KeyCode {
	fn from(value: VirtualKeyCode) -> Self {
		match value {
			VirtualKeyCode::Key1 => KeyCode::Key1,
			VirtualKeyCode::Key2 => KeyCode::Key2,
			VirtualKeyCode::Key3 => KeyCode::Key3,
			VirtualKeyCode::Key4 => KeyCode::Key4,
			VirtualKeyCode::Key5 => KeyCode::Key5,
			VirtualKeyCode::Key6 => KeyCode::Key6,
			VirtualKeyCode::Key7 => KeyCode::Key7,
			VirtualKeyCode::Key8 => KeyCode::Key8,
			VirtualKeyCode::Key9 => KeyCode::Key9,
			VirtualKeyCode::Key0 => KeyCode::Key0,
			VirtualKeyCode::A => KeyCode::A,
			VirtualKeyCode::B => KeyCode::B,
			VirtualKeyCode::C => KeyCode::C,
			VirtualKeyCode::D => KeyCode::D,
			VirtualKeyCode::E => KeyCode::E,
			VirtualKeyCode::F => KeyCode::F,
			VirtualKeyCode::G => KeyCode::G,
			VirtualKeyCode::H => KeyCode::H,
			VirtualKeyCode::I => KeyCode::I,
			VirtualKeyCode::J => KeyCode::J,
			VirtualKeyCode::K => KeyCode::K,
			VirtualKeyCode::L => KeyCode::L,
			VirtualKeyCode::M => KeyCode::M,
			VirtualKeyCode::N => KeyCode::N,
			VirtualKeyCode::O => KeyCode::O,
			VirtualKeyCode::P => KeyCode::P,
			VirtualKeyCode::Q => KeyCode::Q,
			VirtualKeyCode::R => KeyCode::R,
			VirtualKeyCode::S => KeyCode::S,
			VirtualKeyCode::T => KeyCode::T,
			VirtualKeyCode::U => KeyCode::U,
			VirtualKeyCode::V => KeyCode::V,
			VirtualKeyCode::W => KeyCode::W,
			VirtualKeyCode::X => KeyCode::X,
			VirtualKeyCode::Y => KeyCode::Y,
			VirtualKeyCode::Z => KeyCode::Z,
			VirtualKeyCode::Escape => KeyCode::Escape,
			VirtualKeyCode::F1 => KeyCode::F1,
			VirtualKeyCode::F2 => KeyCode::F2,
			VirtualKeyCode::F3 => KeyCode::F3,
			VirtualKeyCode::F4 => KeyCode::F4,
			VirtualKeyCode::F5 => KeyCode::F5,
			VirtualKeyCode::F6 => KeyCode::F6,
			VirtualKeyCode::F7 => KeyCode::F7,
			VirtualKeyCode::F8 => KeyCode::F8,
			VirtualKeyCode::F9 => KeyCode::F9,
			VirtualKeyCode::F10 => KeyCode::F10,
			VirtualKeyCode::F11 => KeyCode::F11,
			VirtualKeyCode::F12 => KeyCode::F12,
			VirtualKeyCode::F13 => KeyCode::F13,
			VirtualKeyCode::F14 => KeyCode::F14,
			VirtualKeyCode::F15 => KeyCode::F15,
			VirtualKeyCode::F16 => KeyCode::F16,
			VirtualKeyCode::F17 => KeyCode::F17,
			VirtualKeyCode::F18 => KeyCode::F18,
			VirtualKeyCode::F19 => KeyCode::F19,
			VirtualKeyCode::F20 => KeyCode::F20,
			VirtualKeyCode::F21 => KeyCode::F21,
			VirtualKeyCode::F22 => KeyCode::F22,
			VirtualKeyCode::F23 => KeyCode::F23,
			VirtualKeyCode::F24 => KeyCode::F24,
			VirtualKeyCode::Snapshot => KeyCode::Snapshot,
			VirtualKeyCode::Scroll => KeyCode::Scroll,
			VirtualKeyCode::Pause => KeyCode::Pause,
			VirtualKeyCode::Insert => KeyCode::Insert,
			VirtualKeyCode::Home => KeyCode::Home,
			VirtualKeyCode::Delete => KeyCode::Delete,
			VirtualKeyCode::End => KeyCode::End,
			VirtualKeyCode::PageDown => KeyCode::PageDown,
			VirtualKeyCode::PageUp => KeyCode::PageUp,
			VirtualKeyCode::Left => KeyCode::Left,
			VirtualKeyCode::Up => KeyCode::Up,
			VirtualKeyCode::Right => KeyCode::Right,
			VirtualKeyCode::Down => KeyCode::Down,
			VirtualKeyCode::Back => KeyCode::Back,
			VirtualKeyCode::Return => KeyCode::Return,
			VirtualKeyCode::Space => KeyCode::Space,
			VirtualKeyCode::Compose => KeyCode::Compose,
			VirtualKeyCode::Caret => KeyCode::Caret,
			VirtualKeyCode::Numlock => KeyCode::Numlock,
			VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
			VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
			VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
			VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
			VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
			VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
			VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
			VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
			VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
			VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
			VirtualKeyCode::NumpadAdd => KeyCode::NumpadAdd,
			VirtualKeyCode::NumpadDivide => KeyCode::NumpadDivide,
			VirtualKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
			VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
			VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
			VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
			VirtualKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
			VirtualKeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
			VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
			VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
			VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
			VirtualKeyCode::Apps => KeyCode::Apps,
			VirtualKeyCode::Asterisk => KeyCode::Asterisk,
			VirtualKeyCode::At => KeyCode::At,
			VirtualKeyCode::Ax => KeyCode::Ax,
			VirtualKeyCode::Backslash => KeyCode::Backslash,
			VirtualKeyCode::Calculator => KeyCode::Calculator,
			VirtualKeyCode::Capital => KeyCode::Capital,
			VirtualKeyCode::Colon => KeyCode::Colon,
			VirtualKeyCode::Comma => KeyCode::Comma,
			VirtualKeyCode::Convert => KeyCode::Convert,
			VirtualKeyCode::Equals => KeyCode::Equals,
			VirtualKeyCode::Grave => KeyCode::Grave,
			VirtualKeyCode::Kana => KeyCode::Kana,
			VirtualKeyCode::Kanji => KeyCode::Kanji,
			VirtualKeyCode::LAlt => KeyCode::LAlt,
			VirtualKeyCode::LBracket => KeyCode::LBracket,
			VirtualKeyCode::LControl => KeyCode::LControl,
			VirtualKeyCode::LShift => KeyCode::LShift,
			VirtualKeyCode::LWin => KeyCode::LWin,
			VirtualKeyCode::Mail => KeyCode::Mail,
			VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
			VirtualKeyCode::MediaStop => KeyCode::MediaStop,
			VirtualKeyCode::Minus => KeyCode::Minus,
			VirtualKeyCode::Mute => KeyCode::Mute,
			VirtualKeyCode::MyComputer => KeyCode::MyComputer,
			VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
			VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
			VirtualKeyCode::NextTrack => KeyCode::NextTrack,
			VirtualKeyCode::NoConvert => KeyCode::NoConvert,
			VirtualKeyCode::OEM102 => KeyCode::OEM102,
			VirtualKeyCode::Period => KeyCode::Period,
			VirtualKeyCode::PlayPause => KeyCode::PlayPause,
			VirtualKeyCode::Plus => KeyCode::Plus,
			VirtualKeyCode::Power => KeyCode::Power,
			VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
			VirtualKeyCode::RAlt => KeyCode::RAlt,
			VirtualKeyCode::RBracket => KeyCode::RBracket,
			VirtualKeyCode::RControl => KeyCode::RControl,
			VirtualKeyCode::RShift => KeyCode::RShift,
			VirtualKeyCode::RWin => KeyCode::RWin,
			VirtualKeyCode::Semicolon => KeyCode::Semicolon,
			VirtualKeyCode::Slash => KeyCode::Slash,
			VirtualKeyCode::Sleep => KeyCode::Sleep,
			VirtualKeyCode::Stop => KeyCode::Stop,
			VirtualKeyCode::Sysrq => KeyCode::Sysrq,
			VirtualKeyCode::Tab => KeyCode::Tab,
			VirtualKeyCode::Underline => KeyCode::Underline,
			VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
			VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
			VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
			VirtualKeyCode::Wake => KeyCode::Wake,
			VirtualKeyCode::WebBack => KeyCode::WebBack,
			VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
			VirtualKeyCode::WebForward => KeyCode::WebForward,
			VirtualKeyCode::WebHome => KeyCode::WebHome,
			VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
			VirtualKeyCode::WebSearch => KeyCode::WebSearch,
			VirtualKeyCode::WebStop => KeyCode::WebStop,
			VirtualKeyCode::Yen => KeyCode::Yen,
			VirtualKeyCode::Copy => KeyCode::Copy,
			VirtualKeyCode::Paste => KeyCode::Paste,
			VirtualKeyCode::Cut => KeyCode::Cut,
		}
	}
}

pub enum Event {
	WindowClose,
	WindowResize { size: Size<u32> },
	WindowFocus,
	WindowLostFocus,
	WindowMoved { offset: Position<i32> },
	WindowScaleChange(f32),

	AppTick,
	AppUpdate,
	AppRender,

	KeyPressed(KeyCode),
	KeyReleased(KeyCode),
	KeyTyped(char),

	MouseButtonPressed(MouseButton),
	MouseButtonReleased(MouseButton),
	MouseMoved { position: Position<f32> },
	MouseScrolled { offset: Position<f32> },
}

impl TryFrom<WinitEvent<'_, ()>> for Event {
	type Error = crate::Error;

	fn try_from(winit_event: WinitEvent<()>) -> Result<Self, Self::Error> {
		let hazel_event = match winit_event {
			WinitEvent::WindowEvent {
				event: window_event,
				..
			} => match window_event {
				WindowEvent::CloseRequested => Self::WindowClose,

				WindowEvent::Resized(PhysicalSize { width, height }) => Self::WindowResize {
					size: Size { width, height },
				},

				WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
					Self::WindowScaleChange(scale_factor as f32)
				},

				WindowEvent::Focused(is_focused) => {
					if is_focused {
						Self::WindowFocus
					} else {
						Self::WindowLostFocus
					}
				},

				WindowEvent::Moved(PhysicalPosition { x, y }) => Self::WindowMoved {
					offset: Position { x, y },
				},

				WindowEvent::KeyboardInput {
					input:
						KeyboardInput {
							state,
							virtual_keycode: Some(virtual_keycode),
							..
						},
					..
				} => match state {
					ElementState::Pressed => Self::KeyPressed(virtual_keycode.into()),
					ElementState::Released => Self::KeyReleased(virtual_keycode.into()),
				},

				WindowEvent::ReceivedCharacter(character) => Self::KeyTyped(character),

				WindowEvent::MouseInput { state, button, .. } => match state {
					ElementState::Pressed => Self::MouseButtonPressed(button.into()),
					ElementState::Released => Self::MouseButtonReleased(button.into()),
				},

				WindowEvent::CursorMoved {
					position: PhysicalPosition { x, y },
					..
				} => Self::MouseMoved {
					position: Position {
						x: x as f32,
						y: y as f32,
					},
				},

				WindowEvent::MouseWheel { delta, .. } => match delta {
					MouseScrollDelta::LineDelta(x, y) => Self::MouseScrolled {
						offset: Position { x, y },
					},
					MouseScrollDelta::PixelDelta(Position { x, y }) => Self::MouseScrolled {
						offset: Position {
							x: x as f32,
							y: y as f32,
						},
					},
				},

				_ => {
					return Err(crate::Error::Event);
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
			Self::WindowClose => write!(fmt, "WindowCloseEvent"),

			Self::WindowResize { size } => {
				write!(fmt, "WindowResizeEvent: ({}, {})", size.width, size.height)
			},

			Self::WindowFocus => write!(fmt, "WindowFocusEvent"),

			Self::WindowLostFocus => write!(fmt, "WindowLostFocusEvent"),

			Self::WindowMoved { offset } => {
				write!(fmt, "WindowMovedEvent: ({}, {})", offset.x, offset.y)
			},

			Self::WindowScaleChange(scale) => write!(fmt, "WindowScaleChangedEvent({scale})"),

			Self::AppTick => write!(fmt, "AppTickEvent"),

			Self::AppUpdate => write!(fmt, "AppUpdateEvent"),

			Self::AppRender => write!(fmt, "AppRenderEvent"),

			Self::KeyPressed(key) => {
				write!(fmt, "KeyPressedEvent: {key}")
			},

			Self::KeyReleased(key) => write!(fmt, "KeyReleasedEvent: {key}"),

			Self::KeyTyped(character) => write!(fmt, "KeyTypedEvent: {character}"),

			Self::MouseButtonPressed(button) => {
				write!(fmt, "MouseButtonPressedEvent: {button}")
			},

			Self::MouseButtonReleased(button) => {
				write!(fmt, "MouseButtonReleasedEvent: {button}")
			},

			Self::MouseMoved { position } => {
				write!(fmt, "MouseMovedEvent: ({}, {})", position.x, position.y)
			},

			Self::MouseScrolled { offset } => {
				write!(fmt, "MouseScrolledEvent: ({}, {})", offset.x, offset.y)
			},
		}
	}
}
