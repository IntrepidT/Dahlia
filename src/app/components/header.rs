use leptos::*;
use leptos_router::*;
use crate::app::components::{ShowAdministerTestModal};

const INPUT_STYLE: &str = "hover:bg-pink-100 flex-initial flex-row border-b-0 border-[#5D4954] rounded font-sans hover:border-b-2 py-2 px-2";
const INPUT_STYLE_SELECTED: &str = "border-b-2 flex-inital hover:bg-pink-100 flex-row border-[#C28CAE] font-sans hover:border-b-2 py-2 px-2";

const basic_head_style: &str = "text-base font-medium text-black underline-offset-8";
#[component]
pub fn Header() -> impl IntoView {

    let (if_show_administer_modal, set_if_show_modal) = create_signal(false);
    //let (toast_message, set_toast_message) = create_signal(ToastMessage::new());
    let on_click = move |_| {
        set_if_show_modal(!if_show_administer_modal());
    }; 

    let(current_path, set_current_path) = create_signal(String::new());
    
    create_effect(move |_| {
        let router_context = use_context::<RouterContext>();
        match router_context {
            Some(route_context) => {
                let path = route_context.pathname().get();
                set_current_path(path);
            }
            None => {
                set_current_path(String::from("/"));
            }
        }
    });

    view!{
        <header class="relative">
            <div class="py-6 item-center ml-10">
                <nav class="fixed relativerounded max-w-8xl mx-auto flex items-center justify-between px-2 sm:px-6">
                    <div class="flex items-center flex-1">
                        <div class="flex items-center justify-between w-full md:w-auto bg-[#00356b] rounded pl-1 pr-2">
                            <A href="/" class="flex items-center space-x-2 font-semibold tracking-tight leading-none">
                                <div class="w-10">
                                    <img src="/assets/dahliano.png" alt="dahlia main page" class="justify-start h-12 w-14 rounded-2xl"/>
                                </div>
                                <div class="text-white">
                                    <div>"Dahlia Software"</div>
                                    <div class="text-base font-light">for Connie Le</div>
                                </div>
                            </A>
                            <div class="-mr-2 flex items-center md:hidden"></div>
                        </div>
                        <div class="hidden space-x-[4rem] md:flex md:ml-[30rem]">
                            <A href="/" class=basic_head_style>"Home Page"</A>
                            <A href="/dataview" class=basic_head_style>"DataView"</A>
                            <button class="text-base font-medium text-black" on:click=on_click>
                                <div>
                                    "Administer Test"
                                    <Show when=move ||{!if_show_administer_modal()}>
                                        <img src="/assets/arrow_down.png" class="inline h-4 w-4"/>
                                    </Show>
                                    <Show when=move || {if_show_administer_modal()}>
                                        <img src="/assets/arrow_up.png" class="inline h-4 w-4" />
                                    </Show>
                                </div>
                            </button>
                            <A href="/activities" class=basic_head_style>"Activities"</A>
                        </div>
                        <div class="flex items-end md:ml-[30rem]  md:flex rounded bg-[#00356B] px-2">
                                <A href="/myaccount" class="text-base font-semibold text-white">
                                    My Account
                                    <img src="/assets/user.png" alt="user icon" class="rounded-2xl bg-[#00356b] h-6 w-6 ml-1 inline"/>
                                </A>
                        </div>
                    </div>
                </nav>
                <Show when=move || {if_show_administer_modal()}>
                    <div class="w-full flex items-center fixed mt-20">
                        <ShowAdministerTestModal
                            set_if_show_modal
                        />
                    </div>
                </Show>
            </div>
        </header>
    }
}

fn get_style_from_url<'a, 'b>(url: &'a str, match_url: &'a str) -> &'b str {
    if url == match_url {
        INPUT_STYLE_SELECTED
    }
    else {
        INPUT_STYLE
    }
}
