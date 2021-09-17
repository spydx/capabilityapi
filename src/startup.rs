use actix_web::Server;

pub struct Application {
    port: u16,
    server: Server,   
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        todo!()
    }
}

pub fn run<T>(
    listner: TcpListner,
    db_pool: Pool<T>,
) -> Result<Server, std::io::Error> {
    todo!()
}