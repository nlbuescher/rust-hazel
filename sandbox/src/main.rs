use hazel::{event::Event, Context, Layer, Size};

struct Sandbox {}

impl Sandbox {
	pub fn new() -> Self {
		Self {}
	}
}

impl hazel::Core for Sandbox {
	fn on_window_close(&self, context: &mut Context) {
		context.exit()
	}

	fn on_window_resize(&self, context: &mut Context, size: Size<u32>) {
		context.resize(size);
	}
}

struct ExampleLayer {}

impl Layer for ExampleLayer {
	fn get_name(&self) -> &str {
		"Example Layer"
	}

	fn on_update(&self, _: &Context) {
		hazel::info!("ExampleLayer::on_update");
	}

	fn on_event(&self, event: &Event) -> bool {
		hazel::info!("{event}");
		true
	}
}

fn main() {
	hazel::info!("TEST");
	hazel::run(Sandbox::new(), |layer_stack| {
		layer_stack.push_layer(Box::new(ExampleLayer {}));
	});
}
