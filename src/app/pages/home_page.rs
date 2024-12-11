use leptos::*;
use leptos_router::*;
use crate::app::components::Header;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <Header />
        <main id="main-content" role="main" class="h-dvh">
            <div class="max-w-8xl mx-auto px-10 h-full">
                <div class="h-5/6 items-center justify-center mt-20 bg-cover bg-[url('/assets/home23.png')] rounded-2xl flex-col">
                    <div class="h-5/6 pt-20 ml-20 mt-30">
                        <h1 class="text-5xl font-extrabold text-left text-white mt-20 mb-10">
                            Simplifying<br/>Standardized<br/>Testing.
                        </h1>
                        <p class="text-2xl font-semibold text-left text-white mt-10">
                            Bringing your testing needs together,<br/>
                            so you can pursue what is important.
                        </p>
                        <div class="flex relative mt-10 font-base justify-start text-center text-white">
                            <A href="/login" class="font-semibold text-center text-white">
                                <div class="bg-[#00356b] hover:before:bg-whiteborder-[#00356b] rounded-2xl border-white border-2 pl-3 pr-3 py-3">
                                    "Log in"
                                    <img src="/assets/arrow.png" alt="arrow" class="inline h-6 w-6 pb-1" />
                                </div>
                            </A>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
