use leptos::*;
use crate::app::components::header::Header;

#[component]
pub fn Activities() -> impl IntoView {
    view! {
        <Header />
        <main class="w-full aspect-video absolute z-[-1] grain">
            <div class="mx-auto max-w-8xl sm:px-6 lg:px-8 mt-20">
                <div class="min-w-0 flex-1">
                    <h1 class="text-2xl font-bold leading-7 text-[#00356b] sm:truncate sm:text-3xl sm:tracking-tight ml-10">Activities</h1>
                </div>
                <ul role="list" class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8 mt-8">
                    <li class="relative"> 
                        //a href link to activity
                        //  then a div class 
                        //   which contains both the starter image and a button and title paragraph
                    </li>
                </ul>
            </div>
        </main>
    }
}
