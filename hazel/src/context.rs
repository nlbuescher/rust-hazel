use std::time::Duration;
use crate::{layer::LayerId, Application, Layer, LayerStack, Size};
use winit::event_loop::ControlFlow;

pub struct LayerContext<'a> {
	delta_time: Duration,
	pub(crate) application: &'a mut Application,
	pub(crate) control_flow: Option<&'a mut ControlFlow>,
}

impl<'a> LayerContext<'a> {
	pub fn new(
		delta_time: Duration,
		application: &'a mut Application,
		control_flow: Option<&'a mut ControlFlow>,
	) -> Self {
		Self {
			delta_time,
			application,
			control_flow,
		}
	}

	pub fn delta_time(&self) -> Duration {
		self.delta_time
	}
}

pub struct EventContext<'a> {
	pub(crate) layer_context: LayerContext<'a>,
	pub(crate) layer_stack: &'a mut LayerStack,
}

impl<'a> EventContext<'a> {
	pub(crate) fn new(
		delta_time: Duration,
		application: &'a mut Application,
		layer_stack: &'a mut LayerStack,
		control_flow: Option<&'a mut ControlFlow>,
	) -> Self {
		Self {
			layer_context: LayerContext::new(delta_time, application, control_flow),
			layer_stack,
		}
	}

	pub fn push_layer(&mut self, mut layer: Box<dyn Layer>) -> LayerId {
		layer.on_attach(&mut self.layer_context);
		self.layer_stack.push_layer(layer)
	}

	pub fn push_overlay(&mut self, mut overlay: Box<dyn Layer>) -> LayerId {
		overlay.on_attach(&mut self.layer_context);
		self.layer_stack.push_overlay(overlay)
	}

	pub fn pop_layer(&mut self, mut layer: Box<dyn Layer>) -> LayerId {
		layer.on_detach(&mut self.layer_context);
		self.layer_stack.push_layer(layer)
	}

	pub fn pop_overlay(&mut self, mut overlay: Box<dyn Layer>) -> LayerId {
		overlay.on_detach(&mut self.layer_context);
		self.layer_stack.push_overlay(overlay)
	}

	pub fn exit(&mut self) {
		if let Some(ref mut control_flow) = self.layer_context.control_flow {
			**control_flow = ControlFlow::Exit;
		}
	}

	pub fn resize(&mut self, size: Size<u32>) {
		self.layer_context.application.resize(size);
	}
}
