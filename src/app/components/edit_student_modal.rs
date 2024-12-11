use crate::app::components::{Toast, ToastMessage, ToastMessageType};
use crate::app::models::{EditStudentRequest, Student};
use crate::app::server_functions::students::edit_student;
use leptos::*;
use std::rc::Rc;
use validator::Validate;

const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white mt-6 outline-none focus:outline-non focus:pl-7 transition-all duration-1000 ease-in-out";

const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-4 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

const UPDATE_BUTTON_STYLE: &str = "mt-10 bg-[#00356b] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

const NO_ERROR_STYLE:&str = "flex flex-col bg-[#222222] border-t-8 border-[#00356b] px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed top-20 z-50";

const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#00356b] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed top-20 z-50";

#[component]
pub fn EditStudentModal(
    student: Rc<Student>,
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    student_resource: Resource<(), Result<Vec<Student>, ServerFnError>>,
) -> impl IntoView {
    let(student_name, set_student_name) = create_signal(student.name.clone());
    let (student_grade, set_student_grade) = create_signal(student.grade.clone());
    let (student_id, set_student_id) = create_signal(format!("{}", student.student_id));
    // for errors
    let (error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);
    //to close the modal if needed
    let on_close= move |_| {
        set_if_show_modal(false);
    };

    let on_click = move |_| {
        let uuid = student.uuid.clone();
        
        let validated_student_id = student_id().parse::<i32>();

        if let Ok(ok_student_id) = validated_student_id {
            let edit_student_request = 
                EditStudentRequest::new(uuid, student_name(), student_grade(), ok_student_id);

            let is_valid = edit_student_request.validate();

            match is_valid {
                Ok(_) => {
                    let _ = spawn_local(async move {
                        let edit_result = edit_student(edit_student_request).await;

                        match edit_result {
                            Ok(_edited_student) => {
                                student_resource.refetch();

                                set_if_show_modal(false);

                                set_toast_message(ToastMessage::create(
                                    ToastMessageType::StudentUpdated,
                                ));

                                set_if_show_toast(true);
                            }
                            Err(_e) => {
                                set_if_error(true);
                                set_error_message(String::from(
                                    "Error Updating Student. Please try again later",
                                ))
                            }
                        };
                    });
                }
                Err(_e) => {
                    set_if_error(true);
                    set_error_message(String::from("All fields are required"))
                }
            }
        } else {
            set_if_error(true);
            set_error_message(String::from("student_id should be numeric"))
        }
    };
    view!{
        <div class="flex flex-col w-full h-full z-50 mx-auto items-center align-center">

            <div class={move || {
                if if_error() {ERROR_STYLE}
                else {NO_ERROR_STYLE}
            }}>
                <Show when=move || {if_error()}>
                    <p class="text-white bg-red-500 rounded w-full h-12 px-5 py-3 transition-all duration-750 ease-in-out">
                        {error_message()}
                    </p>
                </Show>
                <p class="text-white pt-5 text-4xl mb-10">{student_name}</p>

                <input type="text" placeholder="Grade" class=INPUT_STYLE
                    value=student_grade
                    on:input=move |event| {
                        set_student_grade(event_target_value(&event));
                    }
                />
                
                <input type="text" placeholder="Student Id" class=INPUT_STYLE
                    value=student_id
                    on:input=move |event| {
                        set_student_id(event_target_value(&event));
                    }
                />
                <div class="flex flex-row w-full items-right justify-right mt-3">
                    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button on:click=on_click class=UPDATE_BUTTON_STYLE>
                        "Update"
                    </button>
                </div>
            </div>
        </div>
    }
}
