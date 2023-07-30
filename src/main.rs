use async_std::sync::OnceCell;
use tide::prelude::*;
use tide_cors::CorsMiddleware;

use realworld_tide_sqlite_backend::*;

static APP: OnceCell<app::App> = OnceCell::new();

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = app::App::new();

    APP.set(app).expect("Cannot create application instance.");

    let mut app = tide::new();
    
    // Enable CORS middleware with your desired configuration
    let cors_middleware = CorsMiddleware::new()
        .allow_methods("GET, POST, PUT, DELETE")
        .allow_origin("http://localhost:3000")
        .allow_credentials(false);
    
    app.at("/").nest({
        let mut api = tide::with_state(APP.get().unwrap().clone());
        // Add your API routes to `api` here
        api
    });

    // Apply CORS middleware to your app
    app.middleware(cors_middleware);

    app.listen("127.0.0.1:8000").await?;

    Ok(())
}
// use once_cell::sync::OnceCell;

// use realworld_tide_sqlite_backend::*;

// static APP: OnceCell<app::App> = OnceCell::new();

// #[async_std::main]
// async fn main() -> std::result::Result<(), crate::errors::BackendError> {
//     let app = app::App::new();

//     APP.set(app).expect("Cannot create application instance.");
    
//     APP.get().unwrap().run().await
// }
