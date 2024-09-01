struct Sandbox {}

impl Sandbox {}

impl hazel::Application for Sandbox {}

pub fn main() { hazel::run(|| Sandbox {}); }
