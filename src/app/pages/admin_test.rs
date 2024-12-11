use leptos::*;
use crate::app::components::header::Header;

#[component]
pub fn AdministerTest() -> impl IntoView {
    view!{
        <Header />
        <main class="w-full absolute z-[-1] grain h-full">
            <div class="mx-auto max-w-8xl sm:px-6 mt-20 h-full">
                <div class="min-w-0 h-3/5">
                    <h1 class="text-2xl font-bold leading-7 text-white bg-[#00356b] rounded-2xl sm:truncate sm:text-3xl sm:tracking-tight ml-10">Administer Test</h1>
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
