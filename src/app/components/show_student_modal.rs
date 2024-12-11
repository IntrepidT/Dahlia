use crate::app::components::{Toast, ToastMessage, ToastMessageType};
use crate::app::models::{DeleteStudentRequest, Student};
use crate::app::server_functions::students::delete_student;
use leptos::*;
use std::rc::Rc;

//styles for fields
const INFO_STYLE: &str = "w-full h-12 pr-4 py-4 mt-6 flex flex-col outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";

const INFO_TITLE_STYLE: &str = "text-stone-400 text-xs";
const INFO_VALUE_STYLE: &str = "text-white";

const CLOSE_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

const DELETE_BUTTON_STYLE: &str = "mt-10 bg-red-800 px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-red-600";

const MODAL_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#00356B] px-6 pt-5 h-[28rem] w-full max-w-[36rem] z-50 -mt-2 fixed top-20 z-50";

#[component]
pub fn ShowStudentModal(
    student: Rc<Student>,
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_deleted: WriteSignal<bool>,
    student_resource: Resource<(), Result<Vec<Student>, ServerFnError>>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
    let this_student = student.clone();

    //to close the MODAL_STYLE
    let on_close = move |_| {
        set_if_show_modal(false);
    };

    //to perform the deletion
    let on_click_delete = move |_| {
        let to_delete_uuid = format!("{}", &this_student.uuid);

        let delete_student_request = DeleteStudentRequest::new(to_delete_uuid);

        let _ = spawn_local(async move {

            let delete_result = delete_student(delete_student_request).await;

            match delete_result {
                Ok(_deleted_student) => {
                    student_resource.refetch();

                    set_toast_message(ToastMessage::create(ToastMessageType::StudentDeleted));

                    set_if_show_deleted(true);

                    set_if_show_modal(false);
                }
                Err(e) => println!("Error deleting = {:?}", e),
            };
        });
    };

    view!{
        <div class="flex flex-col w-full h-full z-49 bag-[#222222/[.06]]">

            <div class="flex flex-col w-full h-full z-50 mx-auto items-center align-center">
                
                <div class=MODAL_STYLE>

                    <p class="text-white pt-5 text-4xl mb-2 mt-2">
                        {&student.name}
                    </p>

                    <div class=INFO_STYLE>
                        <div class=INFO_TITLE_STYLE>"Grade"</div>
                        <div class=INFO_VALUE_STYLE>{&student.grade}</div>
                    </div>

                    <div class=INFO_STYLE>
                        <div class=INFO_TITLE_STYLE>"Student ID"</div>
                        <div class=INFO_VALUE_STYLE>{format!("#{:?}", &student.student_id)}
                        </div>
                    </div>

                    <div class="flex flex-row w-full items-right justify-right mt-3">
                        <button on:click=on_close class=CLOSE_BUTTON_STYLE>
                            "Close"
                        </button>
                        <button on:click=on_click_delete class=DELETE_BUTTON_STYLE>
                            "Delete"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
