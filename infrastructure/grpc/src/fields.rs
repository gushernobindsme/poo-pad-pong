use crate::api::fields_server::Fields;
use crate::api::{
    AddFieldRequest, AddFieldResponse, Field, GetFieldsRequest, GetFieldsResponse,
    UpdateFieldRequest, UpdateFieldResponse,
};
use crate::error::handle_error;
use domain::fields::{Field as FieldModel, FieldRepository};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct FieldsServerImpl<T: FieldRepository> {
    repository: T,
}

impl<T: FieldRepository> FieldsServerImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl<T: FieldRepository> Fields for FieldsServerImpl<T> {
    async fn get_fields(
        &self,
        _request: Request<GetFieldsRequest>,
    ) -> Result<Response<GetFieldsResponse>, Status> {
        let result = self.repository.find_all().await.map_err(handle_error)?;

        let response = GetFieldsResponse {
            fields: result.into_iter().map(Into::into).collect::<Vec<Field>>(),
        };

        Ok(Response::new(response))
    }

    async fn add_field(
        &self,
        request: Request<AddFieldRequest>,
    ) -> Result<Response<AddFieldResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .create(request.data_label, request.label)
            .await
            .map_err(handle_error)?;
        let response = AddFieldResponse {
            field: Some(result.into()),
        };

        Ok(Response::new(response))
    }

    async fn update_field(
        &self,
        request: Request<UpdateFieldRequest>,
    ) -> Result<Response<UpdateFieldResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .update(request.id, request.label)
            .await
            .map_err(handle_error)?;
        let response = UpdateFieldResponse {
            field: Some(result.into()),
        };

        Ok(Response::new(response))
    }
}

impl From<FieldModel> for Field {
    fn from(value: FieldModel) -> Self {
        Self {
            id: value.id.to_string(),
            data_label: value.data_label,
            label: value.label,
        }
    }
}
