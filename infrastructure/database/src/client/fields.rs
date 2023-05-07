use crate::entities::{fields, fields::Entity as Fields};
use anyhow::Result;
use chrono::{FixedOffset, Utc};
use domain::error::DomainError;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, IntoActiveModel,
    QueryOrder,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresFieldQuery<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> PostgresFieldQuery<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn find_by_id(&self, id: String) -> Result<Option<fields::Model>, DomainError> {
        Fields::find_by_id(id)
            .one(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }

    pub async fn find_all(&self) -> Result<Vec<fields::Model>, DomainError> {
        Fields::find()
            .order_by_asc(fields::Column::CreatedAt)
            .all(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }
}

#[derive(Debug)]
pub struct PostgresFieldCommand<'a> {
    txn: &'a DatabaseTransaction,
}

impl<'a> PostgresFieldCommand<'a> {
    pub fn new(txn: &'a DatabaseTransaction) -> Self {
        Self { txn }
    }

    pub async fn create(
        &self,
        data_label: String,
        label: String,
    ) -> Result<fields::Model, DomainError> {
        let result = fields::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            data_label: Set(data_label),
            label: Set(label),
            created_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())),
            updated_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())),
        }
        .insert(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }

    pub async fn update(&self, id: String, label: String) -> Result<fields::Model, DomainError> {
        let target = Fields::find_by_id(id.to_string())
            .one(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .ok_or(DomainError::NotFound(id))?;

        let result = fields::ActiveModel {
            label: Set(label),
            updated_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())),
            ..target.into_active_model()
        }
        .update(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }

    pub async fn delete(&self, id: String) -> Result<(), DomainError> {
        let _ = Fields::delete_by_id(id)
            .exec(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
