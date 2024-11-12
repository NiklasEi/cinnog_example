#![allow(clippy::type_complexity)]
mod components;
pub mod pages;

use components::navigation::Navigation;
use pages::blog::Blog;
use pages::home_page::HomePage;
use pages::not_found::NotFound;

use crate::pages::blog_post::BlogPost;
use bevy_ecs::prelude::Resource;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;
use cinnog::{run_system, FileName};
use leptos::prelude::*;
use leptos::IntoView;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::static_routes::StaticRoute;
use leptos_router::{path, SsrMode};
use pages::blog_post::Post;
use pages::home_page::PersonName;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Stylesheet href="/pkg/cinnog_example.css"/>
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <FlatRoutes fallback>
                    <Route
                        path=path!("/")
                        view=HomePage
                        ssr=SsrMode::Static(
                            StaticRoute::new(),
                        )
                    />

                    <Route
                        path=path!("/404")
                        view=NotFound
                        ssr=SsrMode::Static(
                            StaticRoute::new(),
                        )
                    />

                    <Route
                        path=path!("/person/*person")
                        view=HomePage
                        ssr=SsrMode::Static(
                            StaticRoute::new()
                                .prerender_params(|| async move {
                                    [("person".into(), run_system(people_static_params))]
                                        .into_iter()
                                        .collect()
                                }),)
                    />

                    <Route
                        path=path!("/blog")
                        view=Blog
                        ssr=SsrMode::Static(
                            StaticRoute::new(),
                        )
                    />

                    <Route
                        path=path!("/blog/*post")
                        view=BlogPost
                        ssr=SsrMode::Static(
                            StaticRoute::new()
                                .prerender_params(|| async move {
                                    [("post".into(), run_system(blog_static_params))]
                                        .into_iter()
                                        .collect()
                                }),
                        )
                    />
                </FlatRoutes>
            </main>
        </Router>
    }
}

fn people_static_params(people: Query<&FileName, With<PersonName>>) -> Vec<String> {
    people.iter().map(|person| person.0.clone()).collect()
}

fn blog_static_params(posts: Query<&FileName, With<Post>>) -> Vec<String> {
    posts.iter().map(|post| post.0.clone()).collect()
}

#[derive(Resource, Clone)]
pub struct SiteName(pub String);
