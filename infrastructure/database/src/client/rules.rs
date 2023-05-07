use crate::entities::sea_orm_active_enums::GenerationType;
use crate::entities::{fields, fields::Entity as Fields};
use crate::entities::{rules, rules::Entity as Rules};
use anyhow::Result;
use chrono::{FixedOffset, Utc};
use domain::error::DomainError;
use domain::rules::GenerationRule;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, IntoActiveModel,
    ModelTrait, QueryOrder,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostgresRuleQuery<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> PostgresRuleQuery<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn find_by_id(
        &self,
        id: String,
    ) -> std::result::Result<Option<(rules::Model, fields::Model)>, DomainError> {
        let result = Rules::find_by_id(id)
            .find_also_related(Fields)
            .one(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        match result {
            Some((rule, Some(field))) => Ok(Some((rule, field))),
            Some((rule, None)) => Err(DomainError::NotFound(rule.field_id)),
            None => Ok(None),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<(rules::Model, fields::Model)>, DomainError> {
        let results = Rules::find()
            .order_by_asc(rules::Column::CreatedAt)
            .find_also_related(Fields)
            .all(self.conn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        results
            .into_iter()
            .map(|(rule, field)| {
                let field = field.ok_or(DomainError::NotFound(rule.field_id.clone()))?;
                Ok((rule, field))
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Debug)]
pub struct PostgresRuleCommand<'a> {
    txn: &'a DatabaseTransaction,
}

impl<'a> PostgresRuleCommand<'a> {
    pub fn new(txn: &'a DatabaseTransaction) -> Self {
        Self { txn }
    }

    pub async fn create(
        &self,
        field_id: String,
        rule_type: GenerationRule,
    ) -> Result<(rules::Model, fields::Model), DomainError> {
        // create
        let rule_value: RuleValues = rule_type.into();
        let rule = rules::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            field_id: Set(field_id),
            r#type: Set(rule_value.r#type),
            regex_pattern: Set(rule_value.regex_pattern),
            regex_replacer: Set(rule_value.regex_replacer),
            created_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())),
            updated_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())),
        }
        .insert(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        // select related entity
        let field = rule
            .find_related(Fields)
            .one(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .ok_or(DomainError::NotFound(rule.field_id.clone()))?;

        Ok((rule, field))
    }

    pub async fn update(
        &self,
        id: String,
        field_id: String,
        rule_type: GenerationRule,
    ) -> Result<(rules::Model, fields::Model), DomainError> {
        // update
        let target = Rules::find_by_id(id.to_string())
            .one(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .ok_or(DomainError::NotFound(field_id.clone()))?;
        let rule_value: RuleValues = rule_type.into();
        let rule = rules::ActiveModel {
            field_id: Set(field_id),
            r#type: Set(rule_value.r#type),
            regex_pattern: Set(rule_value.regex_pattern),
            regex_replacer: Set(rule_value.regex_replacer),
            updated_at: Set(Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())),
            ..target.into_active_model()
        }
        .update(self.txn)
        .await
        .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        // select related entity
        let field = rule
            .find_related(Fields)
            .one(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?
            .ok_or(DomainError::NotFound(rule.field_id.clone()))?;

        Ok((rule, field))
    }

    pub async fn delete(&self, id: String) -> Result<(), DomainError> {
        let _ = Rules::delete_by_id(id)
            .exec(self.txn)
            .await
            .map_err(|e| DomainError::Unexpected(e.to_string()))?;

        Ok(())
    }
}

struct RuleValues {
    r#type: GenerationType,
    regex_pattern: Option<String>,
    regex_replacer: Option<String>,
}

impl From<GenerationRule> for RuleValues {
    fn from(value: GenerationRule) -> Self {
        match value {
            GenerationRule::Equals => RuleValues {
                r#type: GenerationType::Equals,
                regex_pattern: None,
                regex_replacer: None,
            },
            GenerationRule::Regex { pattern, replacer } => RuleValues {
                r#type: GenerationType::Regex,
                regex_pattern: Some(pattern),
                regex_replacer: Some(replacer),
            },
        }
    }
}
