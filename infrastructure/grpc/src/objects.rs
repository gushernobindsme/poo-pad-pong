use crate::api::objects_server::Objects;
use crate::api::{
    AddObjectRequest, AddObjectResponse, DeleteObjectRequest, DeleteObjectResponse,
    GetObjectRequest, GetObjectResponse, GetObjectsRequest, GetObjectsResponse, Object,
    UpdateObjectRequest, UpdateObjectResponse,
};
use crate::error::handle_error;
use domain::objects::{Object as ObjectModel, ObjectRepository};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct ObjectsServerImpl<T: ObjectRepository> {
    repository: T,
}

impl<T: ObjectRepository> ObjectsServerImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl<T: ObjectRepository> Objects for ObjectsServerImpl<T> {
    async fn get_objects(
        &self,
        _request: Request<GetObjectsRequest>,
    ) -> Result<Response<GetObjectsResponse>, Status> {
        let result = self.repository.find_all().await.map_err(handle_error)?;

        let response = GetObjectsResponse {
            objects: result.into_iter().map(Into::into).collect::<Vec<Object>>(),
        };

        Ok(Response::new(response))
    }

    async fn get_object(
        &self,
        request: Request<GetObjectRequest>,
    ) -> Result<Response<GetObjectResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .get(request.id)
            .await
            .map_err(handle_error)?;

        let response = GetObjectResponse {
            object: result.map(Into::into),
        };

        Ok(Response::new(response))
    }

    async fn add_object(
        &self,
        request: Request<AddObjectRequest>,
    ) -> Result<Response<AddObjectResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .create(request.attributes)
            .await
            .map_err(handle_error)?;
        let response = AddObjectResponse {
            object: Some(result.into()),
        };

        Ok(Response::new(response))
    }

    async fn update_object(
        &self,
        request: Request<UpdateObjectRequest>,
    ) -> Result<Response<UpdateObjectResponse>, Status> {
        let request = request.into_inner();
        let result = self
            .repository
            .update(request.id, request.attributes)
            .await
            .map_err(handle_error)?;
        let response = UpdateObjectResponse {
            object: Some(result.into()),
        };

        Ok(Response::new(response))
    }

    async fn delete_object(
        &self,
        request: Request<DeleteObjectRequest>,
    ) -> Result<Response<DeleteObjectResponse>, Status> {
        let request = request.into_inner();
        let _ = self
            .repository
            .delete(request.id)
            .await
            .map_err(handle_error)?;
        let response = DeleteObjectResponse {};

        Ok(Response::new(response))
    }
}

impl From<ObjectModel> for Object {
    fn from(value: ObjectModel) -> Self {
        Self {
            id: value.id.to_string(),
            attributes: value.attributes,
        }
    }
}
