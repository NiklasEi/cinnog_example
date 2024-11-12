use crate::Navigation;
use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Navigation/>
        <h1>"Not Found from Leptos"</h1>
    }
}
