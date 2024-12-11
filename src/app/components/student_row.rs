use crate::app::components::{EditStudentModal, ShowStudentModal, ToastMessage};
use crate::app::models::Student;
use leptos::*;
use std::rc::Rc;

const ROW_STYLE: &str = "bg-[#00356B] rounded px-10 py-5 mb-4 flex flex-row text-left items-left transition-all duration-1000 ease-in-out";

const SHOW_ICON_STYLE: &str = "bg-transparent border-2 border-white px-2.5 mt-2 rounded-full text-white transition-all duration-500 ease-in-out text-xs mr-3 w-7 h-7 hover:w-8 hover:h-8 hover:mb-1";

#[component]
pub fn StudentRow(
    student: Rc<Student>,
    student_resource: Resource<(), Result<Vec<Student>, ServerFnError>>,
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
    let (if_show_info_modal, set_if_show_info_modal) = create_signal(false);
    let (if_show_edit_modal, set_if_show_edit_modal) = create_signal(false);

    let on_show_info = move |_| {
        set_if_show_info_modal(true);
    };

    let on_show_edit = move |_| {
        set_if_show_edit_modal(true);
    };

    let edit_student = student.clone();
    let class_student = student.clone();

    view! {

        <Show when=move || {if_show_info_modal()}>
            <ShowStudentModal
                student=class_student.clone()
                set_if_show_modal=set_if_show_info_modal
                set_if_show_deleted=set_if_show_toast
                student_resource
                set_toast_message
            />
        </Show>

        <Show when=move || {if_show_edit_modal()}>
            <EditStudentModal
                student=edit_student.clone()
                set_if_show_modal=set_if_show_edit_modal
                set_if_show_toast=set_if_show_toast
                student_resource
                set_toast_message
            />
        </Show>
        <div class=ROW_STYLE>
            <div class="flex flex-col w-full max-w-[45rem] bg-[#00356B]">
                <p class="font-bold text-white">{&student.name}</p>
                <p class="text-sm text-white">{&student.grade}</p>
            </div>

            <div class="flex flex-row">
                <button class=SHOW_ICON_STYLE on:click=on_show_info>"i"</button>
                <button class="" on:click=on_show_edit>
                    <img src="assets/edit.png" class="w-[35px] hover:w-[38px] transition-all duration-500" />
                </button>
            </div>
        </div>
    }
}
