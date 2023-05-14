use database::create_database_connection;
use google_cloud_default::WithAuthExt;
use google_cloud_gax::grpc::Status;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::SubscriptionConfig;
use prost::Message;
use repository::keys::KeyRepositoryImpl;
use repository::objects::ObjectRepositoryImpl;
use repository::rules::RuleRepositoryImpl;
use std::env;
use subscriber::keys::KeysHandler;
use subscriber::pubsub_schema;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Status> {
    let topic_id = env::var("PUBSUB_TOPIC_ID").expect("PUBSUB_TOPIC_ID must be set");
    let subscription_id =
        env::var("PUBSUB_SUBSCRIPTION_ID").expect("PUBSUB_SUBSCRIPTION_ID must be set");

    // create pub/sub client
    let client_config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(client_config).await.unwrap();

    // create subscription client
    let subscription = client.subscription(&subscription_id);
    if !subscription.exists(None).await? {
        let topic = client.topic(&topic_id);
        let config = SubscriptionConfig {
            enable_message_ordering: true,
            ..Default::default()
        };
        subscription
            .create(topic.fully_qualified_name(), config, None)
            .await?;
    }

    // Receive message
    subscription
        .receive(
            |message, _cancel| async move {
                // establish database connection
                let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
                let connection = create_database_connection(database_url).await.unwrap();

                let object_repository = ObjectRepositoryImpl::new(connection.clone());
                let rule_repository = RuleRepositoryImpl::new(connection.clone());
                let key_repository = KeyRepositoryImpl::new(connection);

                let key_handler =
                    KeysHandler::new(rule_repository, object_repository, key_repository);

                // Handle data.
                let request =
                    pubsub_schema::SyncKeys::decode(message.message.data.as_slice()).unwrap();
                key_handler.main(request).await.unwrap();

                // Ack or Nack message.
                let _ = message.ack().await;
            },
            CancellationToken::new(),
            None,
        )
        .await?;

    Ok(())
}
