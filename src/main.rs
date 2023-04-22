use std::{collections::HashMap, env::var, error::Error, net::SocketAddr, str::FromStr, sync::Arc};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{AppendHeaders, IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use mu_rust_common::setup_tracing;
use mu_rust_service_common::{extract_session_headers::ExtractSession, SERVICE_HOST, SERVICE_PORT};
use mu_rust_sparql_client::{SparqlClient, SparqlResponse};

async fn hello(axum::extract::Path(name): axum::extract::Path<String>) -> String {
    format!("hello {name} from rust-template")
}

async fn query(
    ExtractSession(session): ExtractSession,
    sparql_client: Extension<Arc<SparqlClient>>,
    _req: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    let query = sparql_client
        .make_query_from_template(r#""#, &HashMap::new())
        .map_err(|e| server_error(e))?;
    let (mu_auth_response_headers, response) = sparql_client
        .query(query, session)
        .await
        .map_err(|e| server_error(e))?;

    let headers = AppendHeaders(mu_auth_response_headers);
    Ok((headers, Json(response)).into_response())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_tracing()?; // setup logging

    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));

    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));

    let addr = SocketAddr::from_str(&format!("{host}:{port}"))?;

    let sparql_client = Arc::new(mu_rust_sparql_client::SparqlClient::new(Default::default())?);

    let app = Router::new()
        .route("/upload", post(query))
        .route("/download/:name", get(hello))
        .layer(Extension(sparql_client));

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn server_error(error: Box<dyn Error>) -> StatusCode {
    tracing::error!("{error}");
    StatusCode::INTERNAL_SERVER_ERROR
}
