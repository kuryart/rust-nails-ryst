mod config;
mod errors;

use crate::errors::CustomError;
// 👇 update axum imports
// use axum::{
//     extract::{Extension,Path},
//     http::{header,HeaderValue,StatusCode},
//     response::Html,
//     response::IntoResponse,
//     response::Redirect,
//     response::Response,
//     routing::get,
//     routing::post,
//     Form,
//     Router,
//     body::{self, Body, Empty}
// };
use axum::{
    extract::Extension,
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    response::Redirect,
    response::Response,
    routing::get,
    routing::post,
    Form,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
// 👇 new import
use validator::Validate;
// use axum::http::{header, HeaderValue, Response, StatusCode};
// use assets::templates::statics::StaticFile;
// use grpc_api::api::api_server::UsersServer;
// use grpc_api::api::users_server::UsersServer;

// use tonic::transport::Server;
// use tower::{make::Shared, steer::Steer, BoxError, ServiceExt};
// use http::{header::CONTENT_TYPE, Request};


#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/sign_up", post(accept_form))
        // .route("/static/*path", get(static_path))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));
        // .boxed_clone();

    // // Handle gRPC API requests
    // let grpc = Server::builder()
    //     .add_service(TraceServer::new(api::trace_grpc_service::TraceService {
    //         pool,
    //     }))
    //     .into_service()
    //     .map_response(|r| r.map(axum::body::boxed))
    //     .boxed_clone();

    // // Create a service that can respond to Web and gRPC
    // let http_grpc = Steer::new(vec![app, grpc], |req: &Request<Body>, _svcs: &[_]| {
    //     if req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes()) != Some(b"application/grpc") {
    //         0
    //     } else {
    //         1
    //     }
    // });

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
        // .serve(Shared::new(http_grpc))
        // .await.unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    // We now return HTML
    Ok(Html(ui_components::users::users(users)))
}

#[derive(Deserialize, Validate)]
struct SignUp {
    #[validate(email)] // 👈 add validate annotation
    email: String,
}

async fn accept_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,

    // 👇 change `Redirect` to `Response`
) -> Result<Response, CustomError> {
    // 👇 add our error handling
    if form.validate().is_err() {
        return Ok((StatusCode::BAD_REQUEST, "Bad request \n").into_response());
    }

    let client = pool.get().await?;

    let email = form.email;
    // TODO - accept a password and hash it
    let hashed_password = String::from("aaaa");
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str(), &hashed_password.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/").into_response()) // 👈 add `.into_response()`
}

// async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
//     let path = path.trim_start_matches('/');

//     if let Some(data) = StaticFile::get(path) {
//         Response::builder()
//             .status(StatusCode::OK)
//             .header(
//                 header::CONTENT_TYPE,
//                 HeaderValue::from_str(data.mime.as_ref()).unwrap(),
//             )
//             .body(body::boxed(Body::from(data.content)))
//             .unwrap()
//     } else {
//         Response::builder()
//             .status(StatusCode::NOT_FOUND)
//             .body(body::boxed(Empty::new()))
//             .unwrap()
//     }
// }