use hazel::{Context, Size};

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

#[pollster::main]
async fn main() -> Result<(), hazel::Error> {
	hazel::info!("TEST");
	hazel::run(Sandbox::new()).await
}
