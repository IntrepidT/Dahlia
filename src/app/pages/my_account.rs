use leptos::*;
use crate::app::components::Header;
 #[component] 
pub fn MyAccount() -> impl IntoView {
    view!{
        <Header/>
        <div class="bg-[#00356B] text-white w-full max-w-[64rem] mx-auto items-center justify-center align-center">
            <p>"This is the My Account Page"</p>            
        </div>
    }
}
