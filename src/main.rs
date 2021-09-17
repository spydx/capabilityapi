use capabilityapi::configuration::get_configuration;
use capabilityapi::startup::Application;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, Capability world!\n");

    let configuration = get_configuration().expect("Failed to get config");

    let server = Application::build(configuration).await?;

    server.run().await?;
    Ok(())
}
