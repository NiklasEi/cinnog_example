use bevy_ecs::component::Component;
use bevy_ecs::prelude::{In, Query};
use cinnog::{run_system_with_input, FileName};
use leptos::*;
use leptos_router::use_params_map;

use crate::components::navigation::Navigation;

#[derive(Component, Clone)]
pub struct TestFontMatter(pub String);

#[derive(Component, Clone)]
pub struct PostTitle(pub String);

#[derive(Component, Clone)]
pub struct DraftPost;

#[derive(Component, Clone)]
pub struct BlogYear(pub String);

#[derive(Component, Clone, Default)]
pub struct Post;

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map().get();
    let current_post = params.get("post").cloned().unwrap();
    let post = run_system_with_input(get_post, current_post);

    view! {
        <Navigation/>
        <div inner_html=post></div>
    }
}

fn get_post(
    In(post): In<String>,
    posts: Query<(&cinnog::loaders::markdown::Html, &FileName)>,
) -> String {
    let post = &posts
        .iter()
        .find(|(_, file_name)| file_name.0 == post)
        .unwrap();
    post.0 .0.clone()
}
