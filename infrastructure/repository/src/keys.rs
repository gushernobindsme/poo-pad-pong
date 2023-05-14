use anyhow::Result;
use database::client::keys::PostgresKeyCommand;
use domain::error::DomainError;
use domain::keys::KeyRepository;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{DatabaseConnection, TransactionTrait};

#[derive(Debug, Clone)]
pub struct KeyRepositoryImpl {
    conn: DatabaseConnection,
}

impl KeyRepositoryImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl KeyRepository for KeyRepositoryImpl {
    async fn create_by_rule(
        &self,
        rule_id: String,
        keys: Vec<(String, String)>,
    ) -> Result<(), DomainError> {
        let result = self
            .conn
            .transaction::<_, (), DomainError>(|txn| {
                Box::pin(async move {
                    let client = PostgresKeyCommand::new(txn);
                    client.create_many(rule_id, keys).await?;

                    Ok(())
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }

    async fn update_by_rule(
        &self,
        rule_id: String,
        keys: Vec<(String, String)>,
    ) -> Result<(), DomainError> {
        let result = self
            .conn
            .transaction::<_, (), DomainError>(|txn| {
                Box::pin(async move {
                    let client = PostgresKeyCommand::new(txn);
                    client.delete_by_rule_id(rule_id.clone()).await?;
                    client.create_many(rule_id, keys).await?;

                    Ok(())
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }

    async fn delete_by_rule(&self, rule_id: String) -> Result<(), DomainError> {
        let result = self
            .conn
            .transaction::<_, (), DomainError>(|txn| {
                Box::pin(async move {
                    let client = PostgresKeyCommand::new(txn);
                    client.delete_by_rule_id(rule_id).await?;

                    Ok(())
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }
}
