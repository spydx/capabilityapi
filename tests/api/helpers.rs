use capabilityapi::configuration::get_configuration;
use capabilityapi::database::Database;
use capabilityapi::startup::Application;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub database: Database,
}

pub async fn initiate_app() -> TestApp {
    let configuration = {
        let mut config = get_configuration().expect("Failed to read config");
        config.application.port = 0;
        config
    };

    

    let databasepool = Database::build(&configuration)
        .await
        .expect("Failed to get database");
    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");

    let application_port = application.port();
    let application_address = format!("http://localhost:{}", application_port);
    let _ = tokio::spawn(application.run());

    TestApp {
        address: application_address,
        database: databasepool,
        port: application_port,
    }
}
