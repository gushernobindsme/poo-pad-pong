use super::pubsub_schema;
use crate::pubsub_schema::sync_keys::Payload;
use anyhow::{anyhow, Result};
use domain::error::DomainError;
use domain::keys::KeyRepository;
use domain::objects::ObjectRepository;
use domain::rules::RuleRepository;

#[derive(Clone, Debug)]
pub struct KeysHandler<R: RuleRepository, O: ObjectRepository, K: KeyRepository> {
    rule_repository: R,
    object_repository: O,
    key_repository: K,
}

impl<R: RuleRepository, O: ObjectRepository, K: KeyRepository> KeysHandler<R, O, K> {
    pub fn new(rule_repository: R, object_repository: O, key_repository: K) -> Self {
        Self {
            rule_repository,
            object_repository,
            key_repository,
        }
    }

    pub async fn main(&self, request: pubsub_schema::SyncKeys) -> Result<()> {
        let payload = request.payload.ok_or(anyhow!("invalid payload"))?;

        match payload {
            Payload::CreateKeysRequest(v) => self.create_keys(v).await,
            Payload::UpdateKeysRequest(v) => self.update_keys(v).await,
            Payload::DeleteKeysRequest(v) => self.delete_keys(v).await,
        }?;

        Ok(())
    }

    async fn create_keys(&self, request: pubsub_schema::CreateKeysRequest) -> Result<()> {
        // get rule and objects
        let rule = self
            .rule_repository
            .get(request.rule_id.clone())
            .await?
            .ok_or(anyhow!("Rule Notfound, id: [{}]", request.rule_id))?;
        let objects = self.object_repository.find_all().await?;

        // generate keys
        let keys = objects
            .into_iter()
            .map(|object| {
                let key = rule.generate_key(object.clone())?;
                Ok((object.id, key))
            })
            .collect::<Result<Vec<_>, DomainError>>()
            .map_err(|e| anyhow!(e.to_string()))?;

        // add new related key
        if !keys.is_empty() {
            self.key_repository.create_by_rule(rule.id, keys).await?;
        }

        Ok(())
    }

    async fn update_keys(&self, request: pubsub_schema::UpdateKeysRequest) -> Result<()> {
        // get rule and objects
        let rule = self
            .rule_repository
            .get(request.rule_id.clone())
            .await?
            .ok_or(anyhow!("Rule Notfound, id: [{}]", request.rule_id))?;
        let objects = self.object_repository.find_all().await?;

        // generate keys
        let keys = objects
            .into_iter()
            .map(|object| {
                let key = rule.generate_key(object.clone())?;
                Ok((object.id, key))
            })
            .collect::<Result<Vec<_>, DomainError>>()
            .map_err(|e| anyhow!(e.to_string()))?;

        // update new related key
        if !keys.is_empty() {
            self.key_repository.update_by_rule(rule.id, keys).await?;
        }

        Ok(())
    }

    async fn delete_keys(&self, request: pubsub_schema::DeleteKeysRequest) -> Result<()> {
        self.key_repository.delete_by_rule(request.rule_id).await?;

        Ok(())
    }
}
