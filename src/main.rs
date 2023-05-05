#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));
    server = match listenfd.take_tcp_listener(0) {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("please set host in .env");
            let port = env::var("PORT").expect("please set port in .env");
            server.bind(format!("{}:{}", host, port));
        }
    };   
    server.run().await 
}
