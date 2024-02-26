use crate::{layer::LayerId, Application, Layer, LayerStack, Size};
use winit::event_loop::ControlFlow;

pub struct Context<'a> {
	pub(crate) application: &'a mut Application,
	pub(crate) layer_stack: &'a mut LayerStack,
	pub(crate) control_flow: Option<&'a mut ControlFlow>,
}

impl<'a> Context<'a> {
	pub(crate) fn new(
		application: &'a mut Application,
		layer_stack: &'a mut LayerStack,
		control_flow: Option<&'a mut ControlFlow>,
	) -> Self {
		Self {
			application,
			layer_stack,
			control_flow,
		}
	}

	pub fn push_layer(&mut self, layer: Box<dyn Layer>) -> LayerId {
		layer.on_attach(self);
		self.layer_stack.push_layer(layer)
	}

	pub fn exit(&mut self) {
		if let Some(ref mut control_flow) = self.control_flow {
			**control_flow = ControlFlow::Exit;
		}
	}

	pub fn resize(&mut self, size: Size<u32>) {
		self.application.resize(size);
	}
}
