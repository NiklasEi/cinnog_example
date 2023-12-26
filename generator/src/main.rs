use app::pages::blog_post::{BlogYear, DraftPost, Post, TestFontMatter};
use app::pages::home_page::{Age, PersonName};
use app::{App, SiteName};
use bevy_ecs::system::EntityCommands;
use cinnog::loaders::ron::read_ron_files_from_directory;
use cinnog::loaders::markdown::{convert_markdown_to_html, read_markdown_from_directory};
use cinnog::loaders::mark_with;
use cinnog::{default_bundle_from_path, DataLayer, Ingest};
use leptos::serde;
use regex::Regex;
use std::io;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut data = DataLayer::new();
    data.insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()));

    data.run(read_ron_files_from_directory::<PersonData>, "people")?;

    let posts = data.run(read_markdown_from_directory::<PostFrontMatter>, "blog")?;
    data.run(mark_with::<Post>, posts);

    data.run(convert_markdown_to_html, ());

    data.build(App).await
}

#[derive(serde::Deserialize)]
struct PersonData {
    name: String,
    age: u8,
}

impl Ingest for PersonData {
    fn ingest(self, commands: &mut EntityCommands) {
        commands.insert((PersonName(self.name), Age(self.age)));
    }
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct PostFrontMatter {
    pub test: String,
    pub draft: bool,
}

impl Ingest for PostFrontMatter {
    fn ingest(self, commands: &mut EntityCommands) {
        commands.insert(TestFontMatter(self.test));
        if self.draft {
            commands.insert(DraftPost);
        }
    }

    fn ingest_path(&self, commands: &mut EntityCommands, path: &PathBuf) {
        let reg = Regex::new(r"/blog/(<year>[0-9]+)/\.*").unwrap();
        if let Some(caps) = reg.captures(&path.to_string_lossy()) {
            let year = &caps["year"];
            commands.insert(BlogYear(year.to_owned()));
        };
        commands.insert(default_bundle_from_path(path));
    }
}
