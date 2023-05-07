use domain::error::DomainError;
use tonic::{Code, Status};

pub fn handle_error(error: DomainError) -> Status {
    match error {
        DomainError::NotFound(e) => Status::new(Code::NotFound, e),
        DomainError::Unexpected(e) => Status::new(Code::Internal, e),
    }
}
