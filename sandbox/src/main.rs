struct Sandbox {}

impl Sandbox {}

impl hazel::Application for Sandbox {}

/// # Errors
pub fn main() -> Result<(), hazel::Error> {
	hazel::run(|| Sandbox {})
}
