use leptos::*;
use leptos_router::*;
use crate::app::components::Header;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <Header/>
        <p>This is the Login Page</p>
    }
}
