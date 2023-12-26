use bevy_ecs::prelude::{Query, Without};
use bevy_ecs::query::With;
use cinnog::{FileName, run_system};
use leptos::{component, IntoView, view};
use crate::pages::blog_post::{DraftPost, Post};
use crate::components::navigation::Navigation;

#[component]
pub fn Blog() -> impl IntoView {
    let posts = run_system(get_posts);

    view! {
        <Navigation/>
        <h1>"Blog posts:"</h1>

            <ul class="people-links">
                {posts
                    .into_iter()
                    .map(|title| {
                        view! {
                            <li>
                                <a href=format!("/blog/{}", title)>{title}</a>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
    }
}

fn get_posts(posts: Query<&FileName, (With<Post>, Without<DraftPost>)>) -> Vec<String> {
    posts
        .iter()
        .map(|file_name| file_name.0.clone()).collect()
}