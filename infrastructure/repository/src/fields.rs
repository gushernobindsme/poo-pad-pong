use database::client::fields::{PostgresFieldCommand, PostgresFieldQuery};
use database::entities::fields;
use domain::error::DomainError;
use domain::fields::{Field, FieldRepository};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{DatabaseConnection, TransactionTrait};

#[derive(Debug, Clone)]
pub struct FieldRepositoryImpl {
    conn: DatabaseConnection,
}

impl FieldRepositoryImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl FieldRepository for FieldRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Field>, DomainError> {
        let client = PostgresFieldQuery::new(&self.conn);
        let response = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(response.into_iter().map(Into::into).collect())
    }

    async fn create(&self, data_label: String, label: String) -> Result<Field, DomainError> {
        let response = self
            .conn
            .transaction::<_, fields::Model, DomainError>(|txn| {
                Box::pin(async move {
                    let client = PostgresFieldCommand::new(txn);
                    client.create(data_label, label).await
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(response.into())
    }

    async fn update(&self, id: String, label: String) -> Result<Field, DomainError> {
        let response = self
            .conn
            .transaction::<_, fields::Model, DomainError>(|txn| {
                Box::pin(async move {
                    let client = PostgresFieldCommand::new(txn);
                    client.update(id, label).await
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(response.into())
    }
}
