use leptos::*;
use leptos_router::*;
use crate::app::components::{Header};

#[component]
pub fn ReadingTesting() -> impl IntoView {
    view!{
        <Header />
        <div>
            <p>This is the reading testing page</p>
        </div>
    }
}
