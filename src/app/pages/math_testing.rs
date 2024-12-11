use leptos::*;
use leptos_router::*;
use crate::app::components::Header;

#[component]
pub fn MathTesting() -> impl IntoView {
    view!{
        <Header />
        <p>This is the Math Testing Page</p>
    }
}
