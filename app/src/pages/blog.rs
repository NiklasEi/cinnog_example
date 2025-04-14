use crate::components::navigation::Navigation;
use crate::pages::blog_post::{DraftPost, Post, PostTitle};
use bevy_ecs::prelude::{Query, Without};
use bevy_ecs::query::With;
use cinnog::{FileName, run_system};
use leptos::prelude::*;

#[component]
pub fn Blog() -> impl IntoView {
    let posts = run_system(get_posts);

    view! {
        <Navigation/>
        <h1>"Blog posts:"</h1>

        <ul class="people-links">
            {posts
                .into_iter()
                .map(|(title, file_name)| {
                    view! {
                        <li>
                            <a href=format!("/blog/{}", file_name.0)>{title.0}</a>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}

fn get_posts(
    posts: Query<(&PostTitle, &FileName), (With<Post>, Without<DraftPost>)>,
) -> Vec<(PostTitle, FileName)> {
    posts
        .iter()
        .map(|(title, file_name)| (title.clone(), file_name.clone()))
        .collect()
}
