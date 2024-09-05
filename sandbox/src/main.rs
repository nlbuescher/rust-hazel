use hazel::{trace, ActiveEventLoop};

struct Sandbox {}

impl Sandbox {}

impl hazel::Application for Sandbox {}

struct ExampleLayer {}

impl hazel::layer::Layer for ExampleLayer {
	fn name(&self) -> &str { "Example" }
	fn on_event(&mut self, _event_loop: &ActiveEventLoop, event: &hazel::event::Event) -> bool {
		trace!("{event:?}");
		
		false
	}
}

/// # Errors
pub fn main() -> Result<(), hazel::Error> {
	hazel::run(Sandbox {}, |layer_stack| {
		layer_stack.push_layer(ExampleLayer {});
	})
}
