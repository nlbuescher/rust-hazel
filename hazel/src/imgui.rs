use crate::{Context, Layer};

struct ImGuiLayer {
}

impl ImGuiLayer {
	pub fn new() -> Self {
		unimplemented!()
	}
}

impl Layer for ImGuiLayer {
	fn get_name(&self) -> &str {
		"ImGuiLayer"
	}

	fn on_attach(&self, _: &Context) {}
}
