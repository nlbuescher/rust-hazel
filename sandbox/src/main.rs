use hazel::{event::Event, EventContext, Layer, LayerContext, Size};

struct Sandbox {}

impl Sandbox {
	pub fn new() -> Self {
		Self {}
	}
}

impl hazel::Core for Sandbox {
	fn on_window_close(&self, context: &mut EventContext) {
		context.exit()
	}

	fn on_window_resize(&self, context: &mut EventContext, size: Size<u32>) {
		context.resize(size);
	}
}

struct ExampleLayer {}

impl ExampleLayer {
	pub fn new() -> Self {
		Self {}
	}
}

impl Layer for ExampleLayer {
	fn get_name(&self) -> &str {
		"Example Layer"
	}

	fn on_update(&mut self, _: &mut LayerContext) {
		hazel::info!("ExampleLayer::on_update");
	}

	fn on_event(&self, event: &Event) -> bool {
		hazel::info!("{event}");
		true
	}
}

fn main() {
	hazel::info!("TEST");
	hazel::run(Sandbox::new(), |context| {
		context.push_layer(Box::new(ExampleLayer::new()));
		context.push_layer(Box::new(hazel::ImGuiLayer::new()));
	});
}
