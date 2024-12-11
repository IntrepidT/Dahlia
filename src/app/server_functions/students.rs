use crate::app::models::{student::Student, AddStudentRequest, DeleteStudentRequest, EditStudentRequest};
use crate::app::errors::{ErrorMessage, ResponseErrorTrait};
use leptos::*;
use serde::*;

#[server(GetStudents, "/api")]
pub async fn get_students() -> Result<Vec<Student>, ServerFnError> {
    let students = retrieve_all_students().await;
    Ok(students)
}

#[server(AddStudent, "/api")]
pub async fn add_student(add_student_request: AddStudentRequest) -> Result<Student, ServerFnError> {
    let new_student = add_new_student(
        add_student_request.name, 
        add_student_request.grade, 
        add_student_request.student_id
    )
    .await;

    match new_student {
        Some(created_student) => Ok(created_student),
        None => Err(ServerFnError::Args(String::from(
                    "Error in creating person!",
        ))),
    }
}

#[server(DeleteStudent, "/api")]
pub async fn delete_student(
    delete_student_request: DeleteStudentRequest,
) -> Result<Student, ServerFnError> {
    let deleted_results = delete_certain_student(delete_student_request.uuid).await;
    match deleted_results {
        Ok(deleted) => {
            if let Some(deleted_student) = deleted {
                Ok(deleted_student)
            }
            else {
                Err(ServerFnError::Response(ErrorMessage::create(
                            StudentError::StudentDeleteFailure,
                )))
            }
        }
        Err(student_error) => Err(ServerFnError::Response(ErrorMessage::create(student_error))),
    }
}

#[server(EditStudent, "/api")]
pub async fn edit_student(edit_student_request: EditStudentRequest) -> Result<Student, ServerFnError> {
    let updated = edit_certain_student(
        edit_student_request.uuid, 
        edit_student_request.name, 
        edit_student_request.grade, 
        edit_student_request.student_id
    )
    .await;

    match updated {
        Ok(updated_result) => {
            if let Some(updated_student) = updated_result {
                Ok(updated_student)
            }
            else {
                Err(ServerFnError::Args(ErrorMessage::create(
                            StudentError::StudentUpdateFailure,
                )))
            }
        }
        Err(student_error) => Err(ServerFnError::Args(ErrorMessage::create(student_error))),
    }
}

cfg_if::cfg_if! {
    
    if #[cfg(feature = "ssr")] {

        use crate::app::db::database;
        use crate::app::errors::StudentError;
        use chrono::{DateTime, Local};
        use uuid::Uuid;

        pub async fn retrieve_all_students() -> Vec<Student> {
            
            let get_all_students_result = database::get_all_students().await;
            match get_all_students_result {
                Some(found_student) => found_student,
                None => Vec::new()
            }
        }

        pub async fn add_new_student<T> (name: T, grade: T, student_id: i32) 

            -> Option<Student> where T: Into<String> {
            
            let mut buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let current_now = Local::now();
            let current_formatted = current_now.to_string();

            let new_student = Student::new(
                String::from(uuid), 
                name.into(), 
                grade.into(), 
                student_id, 
                current_formatted
            );

            database::add_student(new_student).await
        }

        pub async fn delete_certain_student<T>(uuid: T) -> 
            Result<Option<Student>, StudentError> 
            where T:Into<String> {
            
            database::delete_student(uuid.into()).await
        }

        pub async fn edit_certain_student<T>(uuid: T, name: T, grade: T, student_id: i32) -> Result<Option<Student>,StudentError> where T:Into<String> {

            database::update_student(uuid.into(), name.into(), grade.into(), student_id).await
        }
    }
}
