use crate::app::components::{Toast, ToastMessage, ToastMessageType};
use crate::app::models::student::AddStudentRequest;
use crate::app::server_functions::students::add_student;
use leptos::*;
use validator::Validate;

#[component]
pub fn AddStudentModal(
    set_if_show_modal: WriteSignal<bool>, 
    set_if_show_added: WriteSignal<bool>, 
    set_toast_message: WriteSignal<ToastMessage>
) -> impl IntoView {
    const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";

    const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

    const ADD_BUTTON_STYLE: &str = "mt-10 bg-[#00356B] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#00356B] px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

    const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#00356B] px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";
    //create and send signals for various data
    let (student_name, set_student_name) = create_signal(String::new());
    let (student_grade, set_student_grade) = create_signal(String::new());
    let (student_id, set_student_id) = create_signal(String::new());
    //create and send signals for error messages
    let(error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);
    //close the modal
    let on_close= move |_| {
        set_if_show_modal(false);
    };
    //add a new person to the modal
    let on_click = move |_| {
        let add_student_request = AddStudentRequest::new(
            student_name(),
            student_grade(),
            student_id().parse::<i32>().expect("Numbers only"),
        );

        let is_valid = add_student_request.validate();

        match is_valid {
            Ok(_) => {
                spawn_local(async move {
                    let add_result = add_student(add_student_request).await;

                    //we get the result back and do something with it
                    match add_result {
                        Ok(_added_student) => {
                            set_if_show_modal(false);

                            set_toast_message(ToastMessage::create(
                                    ToastMessageType::NewStudentAdded,
                            ));

                            //setting this to true will make the Toast
                            //"new member added" appear
                            set_if_show_added(true);
                        }
                        Err(e) => println!("Error adding: {:?}", e),
                    };
                });
            }
            Err(_) => {
                set_if_error(true);
                set_error_message(String::from("All fields are required"))
            }
        }
    };

    view! {
        <div class="flex flex-col w-full h-full z-50 mx-auto items-center align-center">
            <div class={move || {
                if if_error() {ERROR_STYLE}
                else {NO_ERROR_STYLE}
            }}>
                <Show when=move || {if_error() }>
                    <p class="text-white bg-red-500 rounded w-full h-12 px-5 py-3 transition-all duration-750 ease-in-out">
                    {error_message()}
                    </p>
                </Show>
                <p class="text-white pt-5">"Add New Student"</p>
                <input type="text" placeholder="Name of Student"
                    class=INPUT_STYLE
                    value=student_name
                    on:input=move |event| {
                        set_student_name(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Grade"
                    class=INPUT_STYLE
                    value=student_grade
                    on:input=move |event| {
                        set_student_grade(event_target_value(&event));
                    }
                />
               <input type="text" placeholder="Student ID"
                    class=INPUT_STYLE
                    value=student_id
                    on:input=move |event| {
                        set_student_id(event_target_value(&event));
                    }
               />
               <div class="flex flex-row w-full items-right justify-right">
                    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button on:click=on_click class=ADD_BUTTON_STYLE>
                        "Add"
                    </button>
               </div>
            </div>
        </div>
    }
}
