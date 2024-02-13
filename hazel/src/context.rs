use crate::{Application, Size};
use winit::event_loop::EventLoopWindowTarget;

pub struct Context<'a, 'window> {
	pub(crate) application: &'a mut Application<'window>,
	pub(crate) event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a, 'window> Context<'a, 'window> {
	pub(crate) fn new(
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

	pub fn resize(&mut self, size: Size<u32>) {
		self.application.resize(size);
	}
}
