use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, get, guard, head, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::prelude::*;

#[derive(Debug, clap::Parser)]
pub(crate) struct Serve {
    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

struct AppState {
    datashed: Datashed,
}

#[head("/health-check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/index.ipc")]
async fn index(
    state: web::Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let path = &state.datashed.base_dir().join("index.ipc");
    Ok(NamedFile::open(path)?)
}

impl Serve {
    pub(crate) async fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();

        let Some(config) = datashed.config()?.server else {
            bail!("missing server config");
        };

        let mut builder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls())
                .unwrap();
        builder.set_certificate_chain_file(config.cert).unwrap();
        builder
            .set_private_key_file(config.key, SslFiletype::PEM)
            .unwrap();

        let data = web::Data::new(AppState { datashed });

        let _ = HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .wrap(Logger::new("%r %s %b"))
                .service(
                    Files::new("/data", data_dir.clone())
                        .method_guard(guard::Get())
                        .prefer_utf8(true),
                )
                .service(health_check)
                .service(index)
        })
        .bind_openssl((config.address, config.port), builder)?
        .workers(config.workers.unwrap_or(4))
        .run()
        .await;

        Ok(SUCCESS)
    }
}
