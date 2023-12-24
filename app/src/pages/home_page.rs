use bevy_ecs::component::Component;
use leptos::*;
use leptos_router::use_route;

use crate::components::counter::Counter;
use crate::components::navigation::Navigation;

#[derive(Component, Clone)]
pub struct PersonName(pub String);

#[derive(Component)]
pub struct Age(pub u8);

#[component]
pub fn HomePage() -> impl IntoView {
    let route = use_route();
    let params = route.params().get();
    let no_person = "Dr. Who".to_owned();
    let current_person = params.0.get("person");
    let current_person = current_person.unwrap_or(&no_person);

    view! {
        <Navigation/>
        <h1>"Hello " {current_person} ", welcome to Leptos!"</h1>
        <Counter/>
    }
}
