use actix_files::{Files, NamedFile};
use actix_web::http::StatusCode;
use actix_web::{get, guard, middleware, rt, web, App, HttpResponse, HttpServer, Result};
use clap::ArgMatches;
use log::info;
use std::path::Path;

struct AppState {
    root_dir: String,
}

/// favicon handler
#[get("/favicon")]
async fn favicon(data: web::Data<AppState>) -> Result<NamedFile> {
    let mut file_path = Path::new(&data.root_dir).join("favicon.svg");

    if !file_path.exists() {
        file_path = Path::new(&data.root_dir).join("favicon.ico");
    }

    Ok(NamedFile::open(file_path)?)
}

/// 404 handler
async fn p404(data: web::Data<AppState>) -> Result<NamedFile> {
    // let file_path = Path::new(&data.root_dir).join("404.html");
    let file_path = Path::new(&data.root_dir).join("index.html");
    Ok(NamedFile::open(file_path)?.set_status_code(StatusCode::NOT_FOUND))
}

#[derive(Clone)]
struct ServerOptions {
    host: String,
    port: String,
    root_dir: String,
}

impl ServerOptions {
    fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn run(args: &ArgMatches) -> std::io::Result<()> {
    let mut sys = rt::System::new("server");

    let options = ServerOptions {
        host: args.value_of("host").unwrap().to_string(),
        port: args.value_of("port").unwrap().to_string(),
        root_dir: args.value_of("dir").unwrap().to_string(),
    };

    let addr = options.get_addr();

    let srv = HttpServer::new(move || {
        let root_dir = &options.root_dir;
        info!("Serving {}", root_dir);

        App::new()
            .data(AppState {
                root_dir: String::from(root_dir),
            })
            .wrap(middleware::Compress::default())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
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
