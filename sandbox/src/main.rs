struct Sandbox {}

impl Sandbox {
    pub fn new() -> Self {
        Sandbox{}
    }
}

impl hazel::Application for Sandbox {}

pub fn main() {
    hazel::run(Sandbox::new)
}
