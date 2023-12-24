use app::{Age, App, PersonName, SiteName, TestFontMatter};
use bevy_ecs::prelude::*;
use cinnog::loaders::{
    convert_markdown_to_html, read_markdown_from_directory, read_ron_files_from_directory,
};
use cinnog::{DataLayer, ToBundle};
use leptos::serde;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut data = DataLayer::new();
    data.insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()));

    data.run(read_ron_files_from_directory::<PersonData>, "people")?;
    data.run(read_markdown_from_directory::<PostMetadata>, "posts")?;

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
pub struct PostMetadata {
    pub test: String,
}

impl ToBundle for PostMetadata {
    fn to_bundle(self) -> impl Bundle {
        TestFontMatter(self.test)
    }
}
