struct Sandbox {}

impl Sandbox {
	pub fn new() -> Self {
		Self {}
	}
}

impl hazel::Core for Sandbox {
	fn on_window_close(&self, context: &mut hazel::Context) {
		context.exit()
	}

	fn on_window_resize(&self, context: &mut hazel::Context, width: u32, height: u32) {
		context.resize(width, height);
	}
}

#[pollster::main]
async fn main() -> Result<(), hazel::Error> {
	hazel::run(Sandbox::new()).await
}
