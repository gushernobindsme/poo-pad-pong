use crate::entities::{objects, objects::Entity as Objects};
use anyhow::Result;
use domain::error::DomainError;
use sea_orm::ActiveValue::Set;
use sea_orm::JsonValue as Json;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, IntoActiveModel,
    QueryOrder, TryIntoModel,
};

#[derive(Debug, Clone)]
pub struct PostgresObjectQuery<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> PostgresObjectQuery<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn find_by_id(
        &self,
        id: String,
    ) -> std::result::Result<Option<objects::Model>, DomainError> {
        Objects::find_by_id(id)
            .one(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }

    pub async fn find_all(&self) -> Result<Vec<objects::Model>, DomainError> {
        Objects::find()
            .order_by_asc(objects::Column::CreatedAt)
            .all(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }
}

#[derive(Debug)]
pub struct PostgresObjectCommand<'a> {
    txn: &'a DatabaseTransaction,
}

impl<'a> PostgresObjectCommand<'a> {
    pub fn new(txn: &'a DatabaseTransaction) -> Self {
        Self { txn }
    }

    pub async fn create(&self, attributes: Json) -> Result<objects::Model, DomainError> {
        let result = objects::ActiveModel {
            attributes: Set(attributes),
            ..Default::default()
        }
        .save(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        result
            .try_into_model()
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }

    pub async fn update(
        &self,
        id: String,
        attributes: Json,
    ) -> Result<objects::Model, DomainError> {
        let target = Objects::find_by_id(id.to_string())
            .one(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .ok_or(DomainError::NotFound(id))?;

        let result = objects::ActiveModel {
            attributes: Set(attributes),
            ..target.into_active_model()
        }
        .update(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        result
            .try_into_model()
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }

    pub async fn delete(&self, id: String) -> Result<(), DomainError> {
        let _ = Objects::delete_by_id(id)
            .exec(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
