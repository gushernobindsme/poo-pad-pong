use crate::api::rules_server::Rules;
use crate::api::{
    add_rule_request, rule, update_rule_request, AddRuleRequest, AddRuleResponse,
    DeleteRuleRequest, DeleteRuleResponse, Equals, GetRuleRequest, GetRuleResponse,
    GetRulesRequest, GetRulesResponse, Regex, Rule, UpdateRuleRequest, UpdateRuleResponse,
};
use crate::error::handle_error;
use domain::rules::{GenerationRule, Rule as RuleModel, RuleRepository};
use tonic::{Code, Request, Response, Status};

#[derive(Debug)]
pub struct RulesServerImpl<T: RuleRepository> {
    repository: T,
}

impl<T: RuleRepository> RulesServerImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl<T: RuleRepository> Rules for RulesServerImpl<T> {
    async fn get_rules(
        &self,
        _request: Request<GetRulesRequest>,
    ) -> Result<Response<GetRulesResponse>, Status> {
        let result = self.repository.find_all().await.map_err(handle_error)?;

        let response = GetRulesResponse {
            rules: result.into_iter().map(Into::into).collect::<Vec<Rule>>(),
        };

        Ok(Response::new(response))
    }

    async fn get_rule(
        &self,
        request: Request<GetRuleRequest>,
    ) -> Result<Response<GetRuleResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .get(request.id)
            .await
            .map_err(handle_error)?;

        let response = GetRuleResponse {
            rule: result.map(Into::into),
        };

        Ok(Response::new(response))
    }

    async fn add_rule(
        &self,
        request: Request<AddRuleRequest>,
    ) -> Result<Response<AddRuleResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .create(
                request.field_id,
                request.rule_type.map(Into::into).ok_or(Status::new(
                    Code::InvalidArgument,
                    "InvalidArgument".to_string(),
                ))?,
            )
            .await
            .map_err(handle_error)?;
        let response = AddRuleResponse {
            rule: Some(result.into()),
        };

        Ok(Response::new(response))
    }

    async fn update_rule(
        &self,
        request: Request<UpdateRuleRequest>,
    ) -> Result<Response<UpdateRuleResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .update(
                request.id,
                request.field_id,
                request.rule_type.map(Into::into).ok_or(Status::new(
                    Code::InvalidArgument,
                    "InvalidArgument".to_string(),
                ))?,
            )
            .await
            .map_err(handle_error)?;
        let response = UpdateRuleResponse {
            rule: Some(result.into()),
        };

        Ok(Response::new(response))
    }

    async fn delete_rule(
        &self,
        request: Request<DeleteRuleRequest>,
    ) -> Result<Response<DeleteRuleResponse>, Status> {
        let request = request.into_inner();
        let _ = self
            .repository
            .delete(request.id)
            .await
            .map_err(handle_error)?;
        let response = DeleteRuleResponse {};

        Ok(Response::new(response))
    }
}

impl From<update_rule_request::RuleType> for GenerationRule {
    fn from(value: update_rule_request::RuleType) -> Self {
        match value {
            update_rule_request::RuleType::Equals(_) => GenerationRule::Equals,
            update_rule_request::RuleType::Regex(Regex { pattern, replacer }) => {
                GenerationRule::Regex { pattern, replacer }
            }
        }
    }
}

impl From<add_rule_request::RuleType> for GenerationRule {
    fn from(value: add_rule_request::RuleType) -> Self {
        match value {
            add_rule_request::RuleType::Equals(_) => GenerationRule::Equals,
            add_rule_request::RuleType::Regex(Regex { pattern, replacer }) => {
                GenerationRule::Regex { pattern, replacer }
            }
        }
    }
}

impl From<RuleModel> for Rule {
    fn from(value: RuleModel) -> Self {
        let rule_type = match value.rule_type {
            GenerationRule::Equals => rule::RuleType::Equals(Equals {}),
            GenerationRule::Regex { pattern, replacer } => {
                rule::RuleType::Regex(Regex { pattern, replacer })
            }
        };
        Self {
            id: value.id,
            field: Some(value.field.into()),
            rule_type: Some(rule_type),
        }
    }
}
