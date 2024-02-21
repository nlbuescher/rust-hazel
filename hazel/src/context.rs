use crate::{Application, Size};
use winit::event_loop::ControlFlow;

pub struct Context<'a> {
	pub(crate) application: &'a mut Application,
	pub(crate) control_flow: &'a mut ControlFlow,
}

impl<'a> Context<'a> {
	pub(crate) fn new(
		application: &'a mut Application,
		control_flow: &'a mut ControlFlow,
	) -> Self {
		Context {
			application,
			control_flow,
		}
	}

	pub fn exit(&mut self) {
		*self.control_flow = ControlFlow::Exit;
	}

	pub fn resize(&mut self, size: Size<u32>) {
		self.application.resize(size);
	}
}
