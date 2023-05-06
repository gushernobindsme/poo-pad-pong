use crate::entities::{keys, keys::Entity as Keys};
use anyhow::Result;
use domain::error::DomainError;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, TryIntoModel,
};

#[derive(Debug, Clone)]
pub struct PostgresKeyQuery<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> PostgresKeyQuery<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn find_by_id(
        &self,
        rule_id: String,
        object_id: String,
    ) -> std::result::Result<Option<keys::Model>, DomainError> {
        Keys::find()
            .filter(keys::Column::RuleId.eq(rule_id))
            .filter(keys::Column::ObjectId.eq(object_id))
            .one(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }

    pub async fn find_all(&self) -> Result<Vec<keys::Model>, DomainError> {
        Keys::find()
            .order_by_asc(keys::Column::CreatedAt)
            .all(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }
}

#[derive(Debug)]
pub struct PostgresKeyCommand<'a> {
    txn: &'a DatabaseTransaction,
}

impl<'a> PostgresKeyCommand<'a> {
    pub fn new(txn: &'a DatabaseTransaction) -> Self {
        Self { txn }
    }

    pub async fn create(
        &self,
        rule_id: String,
        object_id: String,
        key: String,
    ) -> Result<keys::Model, DomainError> {
        let result = keys::ActiveModel {
            rule_id: Set(rule_id),
            object_id: Set(object_id),
            key: Set(key),
            ..Default::default()
        }
        .save(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        result
            .try_into_model()
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }

    pub async fn create_many(
        &self,
        rule_id: String,
        keys: Vec<(String, String)>, // (object_id, key)
    ) -> Result<(), DomainError> {
        let keys = keys
            .into_iter()
            .map(|(object_id, key)| keys::ActiveModel {
                rule_id: Set(rule_id.clone()),
                object_id: Set(object_id),
                key: Set(key),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let _ = Keys::insert_many(keys)
            .exec(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }

    pub async fn create_many2(
        &self,
        object_id: String,
        keys: Vec<(String, String)>, // (rule_id, key)
    ) -> Result<(), DomainError> {
        let keys = keys
            .into_iter()
            .map(|(rule_id, key)| keys::ActiveModel {
                rule_id: Set(rule_id),
                object_id: Set(object_id.clone()),
                key: Set(key),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let _ = Keys::insert_many(keys)
            .exec(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }

    pub async fn update(
        &self,
        rule_id: String,
        object_id: String,
        key: String,
    ) -> Result<keys::Model, DomainError> {
        let target = Keys::find()
            .filter(keys::Column::RuleId.eq(rule_id.to_string()))
            .filter(keys::Column::ObjectId.eq(object_id.to_string()))
            .one(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .ok_or(DomainError::NotFound(rule_id))?;

        let result = keys::ActiveModel {
            key: Set(key),
            ..target.into_active_model()
        }
        .update(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        result
            .try_into_model()
            .map_err(|e| DomainError::Unexpected(e.to_string()))
    }

    pub async fn delete_by_rule_id(&self, rule_id: String) -> Result<(), DomainError> {
        let _ = Keys::delete_many()
            .filter(keys::Column::RuleId.eq(rule_id.to_string()))
            .exec(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_by_object_id(&self, object_id: String) -> Result<(), DomainError> {
        let _ = Keys::delete_many()
            .filter(keys::Column::ObjectId.eq(object_id.to_string()))
            .exec(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
