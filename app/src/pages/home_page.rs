use crate::components::counter::Counter;
use crate::components::navigation::Navigation;
use bevy_ecs::component::Component;
use leptos::prelude::*;
use leptos::{component, IntoView};
use leptos_router::hooks::use_params_map;

#[derive(Component, Clone)]
pub struct PersonName(pub String);

#[derive(Component)]
pub struct Age(pub u8);

#[component]
pub fn HomePage() -> impl IntoView {
    let params = use_params_map().get();
    let current_person = params.get("person").unwrap_or("Dr. Who".to_string());

    view! {
        <Navigation/>
        <h1>"Hello " {current_person} ", welcome to Leptos!"</h1>
        <Counter/>
    }
}
