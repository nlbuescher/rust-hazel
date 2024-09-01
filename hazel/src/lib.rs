pub trait Application {
	fn run(&self) {
		loop {
			todo!("Program Logic")
		}
	}
}

pub fn run<App: Application>(constructor: impl Fn() -> App) {
	let application = constructor();
	application.run();
}
