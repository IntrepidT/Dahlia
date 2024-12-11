use crate::app::components::{Toast, ToastMessage, ToastMessageType};
use leptos::*;
use std::rc::Rc;
use leptos_router::*;

const MODAL_STYLE: &str = "flex flex-col bg-[#00356b] border-t-8 border-white px-6 -t-5 h-[28rem] w-full max-w-[36rem] z-50 -mt-2 fixed top-20 z-50";

#[component]
pub fn ShowAdministerTestModal(set_if_show_modal: WriteSignal<bool>) -> impl IntoView {


 view!{
    <div class="flex flex-col w-full z-49">
        <div class="flex flex-row h-[18rem] bg-white items-center justify-center space-x-6 px-4 py-4">
            <A href="/mathtesting" class="w-1/6 rounded-2xl bg-auto  h-full border-2 border-white bg-[#00356b] hover:bg-[#FFB6C1]">
                <div class="items-start object-center ml-5 mt-5">
                    <div class="relative text-white text-left font-bold text-3xl h-full ml-5 mt-8">
                        Math
                        <img src="/assets/calculator.png" class="bg-white rounded-2xl h-10 w-10 inline"/>
                    </div>
                    <div class="relative text-white text-left font-semibold text-xl h-full ml-5 mt-5">
                        <p>All your math needs,<br/>
                        all one place.
                        </p>
                    </div>
                    <img src="/assets/forward_arrow.png" class="h-6 w-6 mt-10 ml-5"/>
                </div>
            </A>
            <A href="/readingtesting" class="w-1/6 rounded-2xl bg-auto h-full border-2 border-white bg-[#00356b] hover:bg-[#FFB6C1]">
                <div class="items-start object-center ml-5 mt-5">
                    <div class="relative text-white text-left font-bold text-3xl h-full ml-5 mt-8">
                        Reading
                        <img src="/assets/reading.png" id="image" class="bg-white rounded-2xl h-10 w-10 inline"/>
                    </div>
                    <div class="relative text-white text-left font-semibold text-xl h-full ml-5 mt-5">
                        <p>Making your reading and <br/>
                        phonics as simple as ABC.
                        </p>
                    </div>
                    <img src="/assets/forward_arrow.png" class="h-6 w-6 mt-10 ml-5"/>
                </div>
            </A>
            <A href="https://dibels.amplify.com" class="w-1/5 rounded-2xl bg-auto h-full border-2 border-white bg-[#00356b] hover:bg-[#FFB6C1]">
                <div class="items-start object-center ml-5 mt-5">
                    <div class="relative text-white text-left font-bold text-3xl h-full ml-5 mt-8">
                        Dibels Testing
                    </div>
                    <div class="relative text-white text-left font-semibold text-xl h-full ml-5 mt-5">
                        <p>Easy testing and data analytics<br/>
                        powered by Amplify.
                        </p>
                    </div>
                    <img src="/assets/forward_arrow.png" class="h-6 w-6 mt-10 ml-5"/>
                </div>
            </A>

        </div>
    </div>
 }
}
