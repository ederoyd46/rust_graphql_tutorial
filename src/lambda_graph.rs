use actix_web::web::Buf;
use juniper::http::{GraphQLRequest, GraphQLResponse};
use lambda_http::{service_fn, Request};
use lambda_runtime::{Error, LambdaEvent};
use std::sync::Arc;

mod schema;
use crate::schema::create_schema;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Startup");
    lambda_http::run(service_fn(lambda_handler)).await?;
    log::info!("Started");
    Ok(())
}

async fn lambda_handler(event: Request) -> Result<String, Error> {
    let body = event.body();
    log::info!("payload {:?}", body);
    let schema = Arc::new(create_schema());
    let data: GraphQLRequest = serde_json::from_reader(body.reader())?;
    let response_data = data.execute(&schema, &()).await;
    log::info!("response data {:?}", &response_data);
    Ok(serde_json::to_string(&response_data).unwrap())
}
