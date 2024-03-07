use std::{ops::{Index, IndexMut}, slice::SliceIndex};

use crate::{event::Event, LayerContext};
use tap::{Pipe, Tap};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug, Hash)]
pub struct LayerId(pub usize);

pub trait Layer {
	fn get_name(&self) -> &str {
		"Layer"
	}
	fn on_attach(&mut self, _: &mut LayerContext) {}
	fn on_detach(&mut self, _: &mut LayerContext) {}
	fn on_update(&mut self, _: &mut LayerContext) {}
	fn on_event(&mut self, _: &mut LayerContext, _: &Event) -> bool {
		false
	}
}

#[derive(Default)]
pub struct LayerStack {
	layers: Vec<(LayerId, Box<dyn Layer>)>,
	layer_insert: usize,
	next_id: usize,
}

impl LayerStack {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	pub fn len(&self) -> usize {
		self.layers.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &(LayerId, Box<dyn Layer>)> {
		self.layers.iter()
	}

	pub fn push_layer(&mut self, layer: Box<dyn Layer>) -> LayerId {
		let layer_id = LayerId(self.next_id);

		self.layers.insert(self.layer_insert, (layer_id, layer));

		self.layer_insert += 1;
		self.next_id += 1;

		layer_id
	}

	pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) -> LayerId {
		let layer_id = LayerId(self.next_id);

		self.layers.push((layer_id, overlay));

		self.next_id += 1;

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
				.tap(|_| self.layer_insert -= 1)
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
			Some((index, _)) => self.layers.remove(index).1.pipe(Some),
		}
	}
}

impl<I: SliceIndex<[(LayerId, Box<dyn Layer>)]>> Index<I> for LayerStack {
	type Output = I::Output;

	fn index(&self, index: I) -> &Self::Output {
		Index::index(&self.layers, index)
	}
}

impl<I: SliceIndex<[(LayerId, Box<dyn Layer>)]>> IndexMut<I> for LayerStack {
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		IndexMut::index_mut(&mut self.layers, index)
	}
}
