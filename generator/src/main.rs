use app::{Age, App, PersonName, SiteName};
use bevy_ecs::prelude::*;
use cinnog::loaders::{read_markdown_from_directory, read_ron_files_from_directory};
use cinnog::{DataLayer, ToBundle};
use leptos::serde;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut data = DataLayer::new();
    data.insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()));

    data.run(read_ron_files_from_directory::<PersonData>, "people")?;
    data.run(read_markdown_from_directory, "people");

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
