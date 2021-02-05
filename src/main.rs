use std::env;

use actix_http::{body::Body, http::StatusCode, Response};
use actix_web::{
    dev::ServiceResponse,
    error::ErrorInternalServerError,
    get,
    middleware::{
        errhandlers::{ErrorHandlerResponse, ErrorHandlers},
        Compress, Logger,
    },
    web::{scope, Data},
    App, Error, HttpResponse, HttpServer, Result,
};
use env_logger::Env;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use tera::{Context, Tera};

use bag_of_tricks::character::Character;

#[get("/")]
async fn index(tmpl: Data<Tera>) -> Result<HttpResponse, Error> {
    let mut rng = Pcg64::from_entropy();
    let mut ctx = Context::new();
    ctx.insert("character", &format!("{}", Character::gen(&mut rng)));
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
#[allow(clippy::clippy::unnecessary_wraps)]
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let tera = request.app_data::<Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut ctx = Context::new();
            ctx.insert("error", error);
            ctx.insert("status_code", res.status().as_str());
            let body = tera.render("error.html", &ctx);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(tera)
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(index)
            .service(scope("").wrap(error_handlers()))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
