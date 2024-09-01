pub mod log;

pub(crate) use crate::log::core_warn;

pub trait Application {
	fn run(&self) {
		loop {
			todo!("Program Logic")
		}
	}
}

pub fn run<App: Application>(constructor: impl Fn() -> App) {
	core_warn!("Initialized Log!");
	let a = 5;
	info!("Hello! Var: {a}");

	let application = constructor();
	application.run();
}
