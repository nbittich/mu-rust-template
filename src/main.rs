use std::{collections::HashMap, env::var, error::Error, net::SocketAddr, str::FromStr, sync::Arc};

use axum::{
    body::Body,
    extract::State,
    http::{header::CONTENT_TYPE, Request, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use mu_rust_common::{setup_tracing, SPARQL_RESULT_CONTENT_TYPE};
use mu_rust_service_common::{extract_session_headers::ExtractSession, SERVICE_HOST, SERVICE_PORT};
use mu_rust_sparql_client::{HeaderValue, SparqlClient};

async fn hello(axum::extract::Path(name): axum::extract::Path<String>) -> String {
    format!("hello {name} from rust-template")
}

async fn query(
    ExtractSession(session): ExtractSession,
    sparql_client: State<Arc<SparqlClient>>,
    _req: Request<Body>,
) -> Result<impl IntoResponse, StatusCode> {
    // build query from template
    let query = sparql_client
        .make_query_from_template(
            include_str!("templates/query1.sparql"),
            &HashMap::from([(
                "type",
                "http://data.vlaanderen.be/ns/besluit#Bestuurseenheid".into(),
            )]),
        )
        .map_err(|e| server_error(e))?;
    // execute non-sudo query (use query_sudo for sudo queries)
    let (mut response_headers, response) = sparql_client
        .query(query, session)
        .await
        .map_err(|e| server_error(e))?;
    // enrich response headers
    response_headers.push((
        CONTENT_TYPE,
        HeaderValue::from_static(SPARQL_RESULT_CONTENT_TYPE),
    ));
    // send
    let headers = AppendHeaders(response_headers);
    Ok((StatusCode::OK, headers, Json(response)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_tracing()?; // setup logging

    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("0.0.0.0"));

    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("80"));

    let addr = SocketAddr::from_str(&format!("{host}:{port}"))?;

    let sparql_client = Arc::new(mu_rust_sparql_client::SparqlClient::new(Default::default())?);

    let app = Router::new()
        .route("/query", post(query))
        .route("/hello/:name", get(hello))
        .with_state(sparql_client);

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
