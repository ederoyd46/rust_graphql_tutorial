use actix_web::web::Buf;
use juniper::http::GraphQLRequest;
use lambda_http::{service_fn, Request};
use lambda_runtime::Error;
use serde_json::Value;
use std::sync::Arc;
mod schema;
use crate::schema::{create_schema, Schema};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = Arc::new(create_schema());
    log::info!("Startup");
    lambda_http::run(service_fn(|event| lambda_handler(event, schema.clone()))).await?;
    log::info!("Shutting down");
    Ok(())
}

async fn lambda_handler(event: Request, schema: Arc<Schema>) -> Result<Value, Error> {
    let data: GraphQLRequest = serde_json::from_reader(event.body().reader())?;
    let response_data = data.execute(&schema, &()).await;
    Ok(serde_json::to_value(&response_data).unwrap())
}
