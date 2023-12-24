mod components;
pub mod pages;

use components::navigation::Navigation;
use pages::blog_post::BlogPost;
use pages::home_page::HomePage;
use pages::not_found::NotFound;

use bevy_ecs::prelude::Resource;
use bevy_ecs::query::With;
use bevy_ecs::system::{IntoSystem, Query};
use cinnog::FileName;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::blog_post::Post;
use pages::home_page::PersonName;
use std::sync::{Arc, Mutex};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Meta name="description" content="A static website generated using Leptos and Bevy ECS"/>
        <Stylesheet href="/pkg/cinnog_example.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=HomePage
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(empty_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/404"
                        view=NotFound
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(empty_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/person/*person"
                        view=HomePage
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(people_static_params))),
                        )
                    />

                    <StaticRoute
                        path="/blog/*post"
                        view=BlogPost
                        static_params=Arc::new(
                            Mutex::new(Box::new(IntoSystem::into_system(blog_static_params))),
                        )
                    />

                </Routes>
            </main>
        </Router>
    }
}

fn empty_static_params() -> StaticParamsMap {
    StaticParamsMap::default()
}

fn people_static_params(people: Query<&FileName, With<PersonName>>) -> StaticParamsMap {
    let mut map = StaticParamsMap::default();
    map.insert(
        "person".to_string(),
        people.iter().map(|person| person.0.clone()).collect(),
    );
    map
}

fn blog_static_params(posts: Query<&FileName, With<Post>>) -> StaticParamsMap {
    let mut map = StaticParamsMap::default();
    map.insert(
        "post".to_string(),
        posts.iter().map(|post| post.0.clone()).collect(),
    );
    map
}

#[derive(Resource, Clone)]
pub struct SiteName(pub String);
