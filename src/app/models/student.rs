use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Student {
    pub uuid: String,
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "grade is required"))]
    pub grade: String,
    #[validate(range(min = 0, max = 99999))]
    pub student_id: i32,
    pub joined_date: String,
}

impl Student {
    pub fn new(uuid:String, name: String, grade: String, student_id: i32, joined_date: String) -> Student{
        Student {
            uuid, name, grade, student_id, joined_date
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct AddStudentRequest {
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "grade is required"))]
    pub grade: String,
    #[validate(range(min = 0, max = 99999))]
    pub student_id: i32,
}

impl AddStudentRequest {
    pub fn new(name:String, grade:String, student_id: i32) -> AddStudentRequest {
        AddStudentRequest{name, grade, student_id}
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct EditStudentRequest {
    #[validate(length(min = 1, message = "student id is required"))]
    pub uuid: String,
    #[validate(length(min = 1, message = "student name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "title is required"))]
    pub grade: String,
    #[validate(range(min = 0, max = 99999))]
    pub student_id: i32,
}

impl EditStudentRequest {
    pub fn new(uuid: String, name: String, grade: String, student_id: i32 ) -> EditStudentRequest {
        EditStudentRequest { uuid, name, grade, student_id}
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DeleteStudentRequest {
    #[validate(length(min = 1, message = "student id is required"))]
    pub uuid: String,
}

impl DeleteStudentRequest {
    pub fn new(uuid: String) -> DeleteStudentRequest {
        DeleteStudentRequest{uuid}
    }
}
