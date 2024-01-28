#[pollster::main]
async fn main() -> Result<(), hazel::Error> {
	hazel::run().await
}
