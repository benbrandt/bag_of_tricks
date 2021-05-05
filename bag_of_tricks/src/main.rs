#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::env;

use character::Character;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use tera::Tera;
use tide_compress::CompressMiddleware;
use tide_tera::{context, TideTeraExt};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    env::set_var("RUST_BACKTRACE", "1");
    let _guard = sentry::init((
        "https://ffdf2fc8b5ff48c4a1e5240f9679abd7@o251876.ingest.sentry.io/5628414",
        sentry::ClientOptions::default(),
    ));

    let port: String = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let mut app = tide::with_state(Tera::new("bag_of_tricks/templates/**/*")?);
    app.with(CompressMiddleware::new());
    app.at("/").get(|req: tide::Request<Tera>| async move {
        let mut rng = Pcg64::from_entropy();
        let tera = req.state();
        tera.render_response(
            "index.html",
            &context! { "character" => format!("{}", Character::gen(&mut rng)) },
        )
    });
    app.listen(format!("127.0.0.1:{}", port)).await?;
    Ok(())
}
