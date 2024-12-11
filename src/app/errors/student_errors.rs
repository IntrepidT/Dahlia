use thiserror::Error;

#[derive(Error, Debug)]
pub enum StudentError {
    #[error("student not found")]
    StudentNotFound,
    #[error("failed to update student")]
    StudentUpdateFailure,
    #[error("failed to create student")]
    StudentCreationFailure,
    #[error("failed to delete student")]
    StudentDeleteFailure,
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(student_error: StudentError) -> ErrorMessage; 
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(student_error: StudentError) -> ErrorMessage {
        match student_error {
            StudentError::StudentNotFound => ErrorMessage::from("student not found"),
            StudentError::StudentUpdateFailure => ErrorMessage::from("failed to update student"),
            StudentError::StudentCreationFailure => ErrorMessage::from("failed to create student"),
            StudentError::StudentDeleteFailure => ErrorMessage::from("failed to delete student"),
        }
    }
}
