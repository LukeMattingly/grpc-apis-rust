#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::pin::Pin;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::PgConnection;
use dotenv::dotenv;
use futures::Stream;
use log::{debug, error, info};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};


use solar_system_info_rpc::solar_system_info::solar_system_info_server::{
    SolarSystemInfo, SolarSystemInfoServer,
};
use solar_system_info_rpc::solar_system_info::{
    Planet, PlanetRequest, PlanetResponse, PlanetsListResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    env_logger::init();

    info!("Starting Solar System info server");

    let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    let pool = create_connection_pool();
    run_migrations(&pool);

    let solar_system_info = SolarSystemInfoService{ pool};
    let svc = SolarSystemInfoServer::new(solar_system_info);

    Server::builder().add_service(svc).serve(addr).await?;
}


