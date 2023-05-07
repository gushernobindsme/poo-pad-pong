use database::create_database_connection;
use grpc::fields::FieldsServerImpl;
use std::env;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

use grpc::api::fields_server::FieldsServer;
use grpc::api::objects_server::ObjectsServer;
use grpc::api::rules_server::RulesServer;
use grpc::objects::ObjectsServerImpl;
use grpc::rules::RulesServerImpl;
use grpc::API_DESCRIPTOR_SET;
use repository::fields::FieldRepositoryImpl;
use repository::objects::ObjectRepositoryImpl;
use repository::rules::RuleRepositoryImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let reflection_server = Builder::configure()
        .register_encoded_file_descriptor_set(API_DESCRIPTOR_SET)
        .build()?;

    // establish database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = create_database_connection(database_url).await?;

    let field_repository = FieldRepositoryImpl::new(connection.clone());
    let object_repository = ObjectRepositoryImpl::new(connection.clone());
    let rule_repository = RuleRepositoryImpl::new(connection);

    let field_server = FieldsServerImpl::new(field_repository);
    let object_server = ObjectsServerImpl::new(object_repository);
    let rule_server = RulesServerImpl::new(rule_repository);

    Server::builder()
        .add_service(FieldsServer::new(field_server))
        .add_service(ObjectsServer::new(object_server))
        .add_service(RulesServer::new(rule_server))
        .add_service(reflection_server)
        .serve(addr)
        .await?;

    Ok(())
}
