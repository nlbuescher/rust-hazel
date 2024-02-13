use crate::{event::Event, Context};
use std::slice::Iter;
use tap::{Pipe, Tap};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug, Hash)]
pub struct LayerId(pub usize);

pub trait Layer {
	fn get_name(&self) -> &str {
		"Layer"
	}
	fn on_attach(&self) {}
	fn on_detach(&self) {}
	fn on_update(&self, _: &Context) {}
	fn on_event(&self, _: &Event) -> bool {
		false
	}
}

#[derive(Default)]
pub struct LayerStack {
	layers: Vec<(LayerId, Box<dyn Layer>)>,
	layer_insert: usize,
}

impl LayerStack {
	pub fn new() -> Self {
        Self { ..Default::default() }
	}

	pub fn iter(&self) -> Iter<'_, (LayerId, Box<dyn Layer>)> {
		self.layers.iter()
	}

	pub fn push_layer(&mut self, layer: Box<dyn Layer>) -> LayerId {
		let layer_id = self.layers.last().map_or(0, |it| it.0 .0 + 1).pipe(LayerId);

		layer.on_attach();

		self.layers.insert(self.layer_insert, (layer_id, layer));
		self.layer_insert += 1;

		layer_id
	}

	pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) -> LayerId {
		let layer_id = self.layers.last().map_or(0, |it| it.0 .0 + 1).pipe(LayerId);

		overlay.on_attach();

		self.layers.push((layer_id, overlay));

		layer_id
	}

	pub fn pop_layer(&mut self, layer_id: LayerId) -> Option<Box<dyn Layer>> {
		let entry = self
			.layers
			.iter()
			.enumerate()
			.find(|(_, it)| it.0 == layer_id);

		match entry {
			None => None,
			Some((index, _)) => self
				.layers
				.remove(index)
				.1
				.tap(|it| {
					self.layer_insert -= 1;
					it.on_detach();
				})
				.pipe(Some),
		}
	}

	pub fn pop_overlay(&mut self, layer_id: LayerId) -> Option<Box<dyn Layer>> {
		let entry = self
			.layers
			.iter()
			.enumerate()
			.find(|(_, it)| it.0 == layer_id);

		match entry {
			None => None,
			Some((index, _)) => self
				.layers
				.remove(index)
				.1
				.tap(|it| it.on_detach())
				.pipe(Some),
		}
	}
}
