use axum::{
    http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router
};
use listenfd::ListenFd;
use rs_system::{self, service::users::UserServiceImpl};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let num = rs_system::add(1, 2);
    println!("Hello, world!{num} years old.");
    
    let app = Router::new()
        .route("/", get(handler))
        .route("/listenfd", get(listenfd_handdler))

    ;
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

async fn listenfd_handdler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}