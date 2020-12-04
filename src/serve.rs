use actix_files::{Files, NamedFile};
use actix_web::http::StatusCode;
use actix_web::{guard, middleware, rt, web, App, HttpResponse, HttpServer, Result};
use log::info;
use std::path::Path;

struct AppState {
    root_dir: String,
}

/// 404 handler
async fn p404(data: web::Data<AppState>) -> Result<NamedFile> {
    // let file_path = Path::new(&data.root_dir).join("404.html");
    let file_path = Path::new(&data.root_dir).join("index.html");
    Ok(NamedFile::open(file_path)?.set_status_code(StatusCode::NOT_FOUND))
}

#[derive(Clone)]
pub struct ServerOptions {
    pub host: String,
    pub port: String,
    pub root_dir: String,
    pub open: bool,
}

impl ServerOptions {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    pub fn get_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

pub fn run_server(options: ServerOptions) -> std::io::Result<()> {
    let mut sys = rt::System::new("server");

    let addr = options.get_addr();
    let url = options.get_url();
    let open = options.open;

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

    if open {
        webbrowser::open(url.as_str())?;
    }

    sys.block_on(srv)
}
