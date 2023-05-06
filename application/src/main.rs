use database::create_database_connection;
use grpc::fields::FieldsServerImpl;
use std::env;
use tonic::transport::Server;

use grpc::api::fields_server::FieldsServer;
use grpc::api::objects_server::ObjectsServer;
use grpc::objects::ObjectsServerImpl;
use repository::fields::FieldRepositoryImpl;
use repository::objects::ObjectRepositoryImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // establish database connection
    let connection = create_database_connection(database_url).await?;
    let field_repository = FieldRepositoryImpl::new(connection.clone());
    let object_repository = ObjectRepositoryImpl::new(connection);

    let field_server = FieldsServerImpl::new(field_repository);
    let object_server = ObjectsServerImpl::new(object_repository);
    Server::builder()
        .add_service(FieldsServer::new(field_server))
        .add_service(ObjectsServer::new(object_server))
        .serve(addr)
        .await?;

    Ok(())
}
