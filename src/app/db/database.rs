cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {

        use crate::app::models::Student;
        use crate::app::errors::{ErrorMessage, StudentError};
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {

            let _ = DB.connect::<Ws>("127.0.0.1:8000").await;
            let _ = DB.signin(Root {
                username: "root",
                password: "root",
            })
            .await;
            let _ = DB.use_ns("surreal").use_db("student").await;
        }

        pub async fn get_all_students() -> Option<Vec<Student>> {
            
            open_db_connection().await;
            let get_all_students = DB.query("Select * FROM student  ORDER BY joined_date DESC;").await;

            match get_all_students {

                Ok(mut res) => {
                    let found = res.take(0);
                    match found {
                        Ok(found_students) => Some(found_students),
                        Err(_) => None,
                    }
                },
                Err(_) => None,
            }
        }

        pub async fn add_student(new_student: Student) -> Option<Student> {

            open_db_connection().await;
            let results = DB.create(("student", new_student.uuid.clone()))
                .content(new_student)
                .await;
            let _ = DB.invalidate().await;

            match results {
                Ok(created_student) => created_student,
                Err(e) => {
                    println!("error in adding new student: {:?}", e);
                    None
                }
            }
        }

        pub async fn delete_student(student_uuid: String) -> Result<Option<Student>, StudentError> {

            open_db_connection().await;
            let delete_results = DB.delete(("student", student_uuid)).await;
            let _ = DB.invalidate().await;

            match delete_results {
                Ok(deleted_student) => Ok(deleted_student),
                Err(_) => Err(StudentError::StudentDeleteFailure)
            }
        }

        pub async fn update_student(uuid: String, name: String, grade: String, student_id: i32)-> Result<Option<Student>, StudentError> {
            
            open_db_connection().await;

            let find_student: Result<Option<Student>, Error> = 
                DB.select(("student", &uuid)).await;
            match find_student {

                Ok(found) => {

                    match found {
                        Some(found_student) => {

                            let updated_student: Result<Option<Student>, Error> =
                                DB.update(("student", &uuid))
                                .merge(Student::new(
                                        uuid,
                                        name,
                                        grade,
                                        student_id,
                                        found_student.joined_date
                                ))
                                .await;
                            let _ = DB.invalidate().await;
                            match updated_student {
                                Ok(returned_student) => Ok(returned_student),
                                Err(_) => Err(StudentError::StudentUpdateFailure)
                            }
                        },
                        None => Err(StudentError::StudentUpdateFailure)
                    }
                },
                Err(_) => {
                    let _ = DB.invalidate().await;
                    Err(StudentError::StudentNotFound)
                }
            }
        }
    }
}
