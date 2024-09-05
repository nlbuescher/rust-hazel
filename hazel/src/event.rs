#[derive(Debug)]
pub enum Event {
	KeyPressed { key: crate::Key, is_repeat: bool },
	KeyReleased { key: crate::Key },
	MouseButtonPressed(crate::MouseButton),
	MouseButtonReleased(crate::MouseButton),
	MouseMoved { x: f32, y: f32 },
	MouseScrolled { x_offset: f32, y_offset: f32 },
	WindowClose,
	WindowResize { width: u32, height: u32 },
}
