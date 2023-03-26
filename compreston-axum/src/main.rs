
use axum::{
    extract::{DefaultBodyLimit, Multipart},
    response::{Html, IntoResponse}, 
    routing::get,
    Router, body::Bytes,
    Json, 
    http::StatusCode,
};
use std::net::SocketAddr;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::{Serialize, Deserialize};


#[tokio::main]
async fn main() {
    // This adds a logger to our system, allowing us to take some logs. That's cool
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "compreston-axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our application with some routes
    let app = Router::new()
        .route("/", get(show_form).post(accept_form))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250*1024*1024, /* 250mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/success", get(show_success));


    //  Run it with hyyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <label> 
                        Upload file:
                        <input type="file" name="file">
                    </label>
                    
                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}


// This section creates the structs that will be used in the website
async fn accept_form(mut multipart: Multipart) -> impl IntoResponse { 
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of {} ({} : {} ) is {} bytes",
            name, 
            file_name, 
            content_type,
            data.len()
        );
    // Strategy here is to upload these to a DB (ideally, a local one), then to use it when we get there. 
    // The problem with returning here is that we expect something regardless of how many times the loop runs,
    // but there's no way to do that kind of checking. 
    }
    (StatusCode::ACCEPTED, Json("This... prints?"))
}
async fn show_success(submission: Bytes) {
    submission.len();
} 

