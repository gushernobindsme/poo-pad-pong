use database::client::fields::PostgresFieldQuery;
use database::client::keys::PostgresKeyCommand;
use database::client::objects::{PostgresObjectCommand, PostgresObjectQuery};
use database::client::rules::PostgresRuleQuery;
use database::{to_object, to_rule};
use domain::error::DomainError;
use domain::objects::{Object, ObjectRepository};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{DatabaseConnection, TransactionTrait};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ObjectRepositoryImpl {
    conn: DatabaseConnection,
}

impl ObjectRepositoryImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl ObjectRepository for ObjectRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Object>, DomainError> {
        let client = PostgresObjectQuery::new(&self.conn);
        let objects = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let client = PostgresFieldQuery::new(&self.conn);
        let fields = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        objects
            .into_iter()
            .map(|objects| to_object(objects, fields.clone()))
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get(&self, id: String) -> Result<Option<Object>, DomainError> {
        let client = PostgresObjectQuery::new(&self.conn);
        let object = client
            .find_by_id(id)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        let client = PostgresFieldQuery::new(&self.conn);
        let fields = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        object.map(|v| to_object(v, fields)).transpose()
    }

    async fn create(&self, attributes: HashMap<String, String>) -> Result<Object, DomainError> {
        let client = PostgresFieldQuery::new(&self.conn);
        let fields = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        // get exits rules
        let client = PostgresRuleQuery::new(&self.conn);
        let rules = client.find_all().await?;

        let result = self
            .conn
            .transaction::<_, Object, DomainError>(|txn| {
                Box::pin(async move {
                    // create object
                    let client = PostgresObjectCommand::new(txn);
                    let object = client.create(json!(attributes)).await?;
                    let result = to_object(object, fields.clone())?;

                    // generate keys
                    let keys = rules
                        .into_iter()
                        .map(|rule| {
                            let rule = to_rule(rule.0, rule.1);
                            let key = rule.generate_key(result.clone())?;
                            Ok((rule.id, key))
                        })
                        .collect::<Result<Vec<_>, DomainError>>()?;

                    // add new related key
                    if !keys.is_empty() {
                        let client = PostgresKeyCommand::new(txn);
                        client.create_many2(result.id.to_string(), keys).await?;
                    }

                    Ok(result)
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }

    async fn update(
        &self,
        id: String,
        attributes: HashMap<String, String>,
    ) -> Result<Object, DomainError> {
        let client = PostgresFieldQuery::new(&self.conn);
        let fields = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        // get exits rules
        let client = PostgresRuleQuery::new(&self.conn);
        let rules = client.find_all().await?;

        let result = self
            .conn
            .transaction::<_, Object, DomainError>(|txn| {
                Box::pin(async move {
                    let client = PostgresObjectCommand::new(txn);
                    let object = client.update(id, json!(attributes)).await?;
                    let result = to_object(object, fields.clone())?;

                    // generate keys
                    let keys = rules
                        .clone()
                        .into_iter()
                        .map(|rule| {
                            let rule = to_rule(rule.0, rule.1);
                            let key = rule.generate_key(result.clone())?;
                            Ok((rule.id, key))
                        })
                        .collect::<Result<Vec<_>, DomainError>>()?;

                    if !keys.is_empty() {
                        let client = PostgresKeyCommand::new(txn);
                        // remove related key
                        for (rule, _) in rules {
                            client.delete_by_rule_id(rule.id).await?;
                        }
                        // add new related key
                        client.create_many2(result.id.to_string(), keys).await?;
                    }

                    Ok(result)
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }

    async fn delete(&self, id: String) -> Result<(), DomainError> {
        let _ = self
            .conn
            .transaction::<_, (), DomainError>(|txn| {
                Box::pin(async move {
                    let client = PostgresObjectCommand::new(txn);
                    client.delete(id.clone()).await?;

                    // delete related keys
                    let client = PostgresKeyCommand::new(txn);
                    client.delete_by_object_id(id).await?;

                    Ok(())
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
