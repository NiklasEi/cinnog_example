use crate::components::navigation::Navigation;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{In, Query};
use cinnog::loaders::markdown::Html;
use cinnog::run_system_with_input;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[derive(Component, Clone)]
pub struct TestFontMatter(pub String);

#[derive(Component, Clone)]
pub struct PostTitle(pub String);

#[derive(Component, Clone)]
pub struct DraftPost;

#[derive(Component, Clone)]
pub struct BlogYear(pub String);

#[derive(Component, Clone, Default, Debug)]
pub struct Post(pub String);

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map().get();
    let current_post = params.get("post").unwrap();
    let post = run_system_with_input(get_post, current_post);

    view! {
        <Navigation/>
        <div inner_html=post></div>
    }
}

fn get_post(In(post): In<String>, posts: Query<(&Html, &Post)>) -> String {
    let (Html(html), _) = &posts
        .iter()
        .find(|(_, file_name)| file_name.0 == post)
        .unwrap();
    html.clone()
}
