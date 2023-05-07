use database::client::fields::PostgresFieldQuery;
use database::client::keys::PostgresKeyCommand;
use database::client::objects::PostgresObjectQuery;
use database::client::rules::{PostgresRuleCommand, PostgresRuleQuery};
use database::{to_object, to_rule};
use domain::error::DomainError;
use domain::rules::{GenerationRule, Rule, RuleRepository};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{DatabaseConnection, TransactionTrait};

#[derive(Debug, Clone)]
pub struct RuleRepositoryImpl {
    conn: DatabaseConnection,
}

impl RuleRepositoryImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl RuleRepository for RuleRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Rule>, DomainError> {
        let client = PostgresRuleQuery::new(&self.conn);
        let response = client.find_all().await?;

        Ok(response
            .into_iter()
            .map(|(rule, fields)| to_rule(rule, fields))
            .collect())
    }

    async fn get(&self, id: String) -> Result<Option<Rule>, DomainError> {
        let client = PostgresRuleQuery::new(&self.conn);
        let response = client.find_by_id(id).await?;

        Ok(response.map(|(rule, field)| to_rule(rule, field)))
    }

    async fn create(
        &self,
        field_id: String,
        rule_type: GenerationRule,
    ) -> Result<Rule, DomainError> {
        let client = PostgresFieldQuery::new(&self.conn);
        let fields = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        // get exits objects
        let client = PostgresObjectQuery::new(&self.conn);
        let objects = client.find_all().await?;
        let objects = objects
            .into_iter()
            .map(|object| to_object(object, fields.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        let result = self
            .conn
            .transaction::<_, Rule, DomainError>(|txn| {
                Box::pin(async move {
                    // add new rule
                    let client = PostgresRuleCommand::new(txn);
                    let result = client.create(field_id, rule_type).await?;
                    let rule = to_rule(result.0, result.1);

                    // TODO: 件数が多い場合時間がかかるため Pub/Sub を使うようにしたい
                    // generate keys
                    let keys = objects
                        .into_iter()
                        .map(|object| {
                            let key = rule.generate_key(object.clone())?;
                            Ok((object.id, key))
                        })
                        .collect::<Result<Vec<_>, DomainError>>()?;

                    // add new related key
                    if !keys.is_empty() {
                        let client = PostgresKeyCommand::new(txn);
                        client.create_many(rule.id.clone(), keys).await?;
                    }

                    Ok(rule)
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(result)
    }

    async fn update(
        &self,
        id: String,
        field_id: String,
        rule_type: GenerationRule,
    ) -> Result<Rule, DomainError> {
        let client = PostgresFieldQuery::new(&self.conn);
        let fields = client
            .find_all()
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        // get exits objects
        let client = PostgresObjectQuery::new(&self.conn);
        let objects = client.find_all().await?;
        let objects = objects
            .into_iter()
            .map(|object| to_object(object, fields.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        let result = self
            .conn
            .transaction::<_, Rule, DomainError>(|txn| {
                Box::pin(async move {
                    // update exists rule
                    let client = PostgresRuleCommand::new(txn);
                    let result = client.update(id.clone(), field_id, rule_type).await?;
                    let rule = to_rule(result.0, result.1);

                    // TODO: 件数が多い場合時間がかかるため Pub/Sub を使うようにしたい
                    // generate keys
                    let keys = objects
                        .into_iter()
                        .map(|object| {
                            let key = rule.generate_key(object.clone())?;
                            Ok((object.id, key))
                        })
                        .collect::<Result<Vec<_>, DomainError>>()?;

                    if !keys.is_empty() {
                        let client = PostgresKeyCommand::new(txn);
                        // remove related key
                        client.delete_by_rule_id(id).await?;
                        // add new related key
                        client.create_many(rule.id.clone(), keys).await?;
                    }

                    Ok(rule)
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
                    let client = PostgresRuleCommand::new(txn);
                    client.delete(id.clone()).await?;

                    // delete related keys
                    let client = PostgresKeyCommand::new(txn);
                    client.delete_by_rule_id(id).await?;

                    Ok(())
                })
            })
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
