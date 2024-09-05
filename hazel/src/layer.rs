use std::iter::Rev;
use std::ops::DerefMut;
use std::slice;
use winit::event_loop::ActiveEventLoop;
use event::Event;
use crate::event;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct LayerId(usize);

pub trait Layer {
	fn name(&self) -> &str;
	fn on_event(&mut self, _event_loop: &ActiveEventLoop, _event: &Event) -> bool { false }
}

pub struct LayerStack {
	ids: Vec<LayerId>,
	data: Vec<Box<dyn Layer>>,
	layer_insert: usize, // one past the last overlay
	next_layer_id: usize,
}

pub struct IterMut<'data> {
	inner: Rev<slice::IterMut<'data, Box<dyn Layer>>>,
}

impl<'data> Iterator for IterMut<'data> {
	type Item = &'data mut dyn Layer;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|it| { it.deref_mut() })
	}
}

impl LayerStack {
	pub(crate) fn new() -> LayerStack {
		LayerStack { ids: Vec::new(), data: Vec::new(), layer_insert: 0, next_layer_id: 1 }
	}

	pub fn push_layer(&mut self, layer: impl Layer + 'static) -> LayerId {
		let layer_id = LayerId(self.next_layer_id);

		self.ids.insert(self.layer_insert, layer_id);
		self.data.insert(self.layer_insert, Box::new(layer));

		self.next_layer_id += 1;
		self.layer_insert += 1;

		layer_id
	}

	pub fn push_overlay(&mut self, overlay: impl Layer + 'static) -> LayerId {
		let layer_id = LayerId(self.next_layer_id);

		self.ids.push(layer_id);
		self.data.push(Box::new(overlay));

		self.next_layer_id += 1;

		layer_id
	}

	pub fn pop_layer(&mut self, layer_id: LayerId) -> Option<Box<dyn Layer>> {
		self.ids.iter().position(|it| *it == layer_id)
			.map(|index| {
				self.layer_insert -= 1;
				self.data.remove(index)
			})
	}

	pub fn pop_overlay(&mut self, layer_id: LayerId) -> Option<Box<dyn Layer>> {
		self.ids.iter().position(|it| *it == layer_id)
			.map(|index| self.data.remove(index))
	}

	#[must_use]
	pub fn iter_mut(&mut self) -> IterMut<'_> {
		IterMut { inner: self.data.iter_mut().rev() }
	}
}

impl<'data> IntoIterator for &'data mut LayerStack {
	type Item = &'data mut dyn Layer;
	type IntoIter = IterMut<'data>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
