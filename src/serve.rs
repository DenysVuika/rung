use actix_files::{Files, NamedFile};
use actix_web::http::StatusCode;
use actix_web::{get, guard, middleware, rt, web, App, HttpResponse, HttpServer, Result};
use clap::ArgMatches;

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("./assets/web/favicon.svg")?)
}

/// 404 handler
async fn p404() -> Result<NamedFile> {
    Ok(NamedFile::open("./assets/web/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

pub fn run(args: &ArgMatches) -> std::io::Result<()> {
    let mut sys = rt::System::new("server");

    let srv = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(favicon)
            .service(Files::new("/", "./assets/web").index_file("index.html"))
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run();

    sys.block_on(srv)
}
