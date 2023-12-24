use app::pages::blog_post::{Post, TestFontMatter};
use app::pages::home_page::{Age, PersonName};
use app::{App, SiteName};
use bevy_ecs::prelude::*;
use cinnog::loaders::{
    convert_markdown_to_html, mark_with, read_markdown_from_directory,
    read_ron_files_from_directory,
};
use cinnog::{DataLayer, ToBundle};
use leptos::serde;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut data = DataLayer::new();
    data.insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()));

    data.run(read_ron_files_from_directory::<PersonData>, "people")?;

    let posts = data.run(read_markdown_from_directory::<PostFrontMatter>, "posts")?;
    data.run(mark_with::<Post>, posts);

    data.run(convert_markdown_to_html, ());

    data.build(App).await
}

#[derive(serde::Deserialize)]
struct PersonData {
    name: String,
    age: u8,
}

impl ToBundle for PersonData {
    fn to_bundle(self) -> impl Bundle {
        (PersonName(self.name), Age(self.age))
    }
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct PostFrontMatter {
    pub test: String,
}

impl ToBundle for PostFrontMatter {
    fn to_bundle(self) -> impl Bundle {
        TestFontMatter(self.test)
    }
}
