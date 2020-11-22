use actix_files::{Files, NamedFile};
use actix_web::http::StatusCode;
use actix_web::{get, guard, middleware, rt, web, App, HttpResponse, HttpServer, Result};
use clap::ArgMatches;
use std::path::Path;

struct AppState {
    root_dir: String,
}

/// favicon handler
#[get("/favicon")]
async fn favicon(data: web::Data<AppState>) -> Result<NamedFile> {
    let file_path = Path::new(&data.root_dir).join("favicon.svg");
    Ok(NamedFile::open(file_path)?)
}

/// 404 handler
async fn p404(data: web::Data<AppState>) -> Result<NamedFile> {
    let file_path = Path::new(&data.root_dir).join("404.html");
    Ok(NamedFile::open(file_path)?.set_status_code(StatusCode::NOT_FOUND))
}

pub fn run(args: &ArgMatches) -> std::io::Result<()> {
    let mut sys = rt::System::new("server");

    let host = args.value_of("host").unwrap();
    let port = args.value_of("port").unwrap();
    let addr = format!("{}:{}", host, port);

    let srv = HttpServer::new(|| {
        let root_dir = "./assets/web";

        App::new()
            .data(AppState {
                root_dir: String::from(root_dir),
            })
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(favicon)
            .service(Files::new("/", &root_dir).index_file("index.html"))
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
    .bind(addr)?
    .run();

    sys.block_on(srv)
}
