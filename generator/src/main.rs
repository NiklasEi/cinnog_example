use app::pages::blog_post::{BlogYear, DraftPost, Post, PostTitle, TestFontMatter};
use app::pages::home_page::{Age, PersonName};
use app::{shell, SiteName};
use bevy_ecs::system::EntityCommands;
use cinnog::loaders::markdown::{ConvertMarkdownToHtml, MarkdownDataLayer};
use cinnog::loaders::ron::RonDataLayer;
use cinnog::{default_bundle_from_path, DataLayer, Ingest};
use leptos::serde;
use regex::Regex;
use std::io;
use std::path::Path;

#[tokio::main]
async fn main() -> io::Result<()> {
    DataLayer::new()
        .insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()))
        .add_markdown_directory::<PostFrontMatter>("blog")
        .add_ron_directory::<PersonData>("people")
        .add_plugins(ConvertMarkdownToHtml)
        .build(shell)
        .await
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
    pub title: String,
    pub draft: bool,
}

impl Ingest for PostFrontMatter {
    fn ingest(self, commands: &mut EntityCommands) {
        commands.insert((TestFontMatter(self.test), PostTitle(self.title)));
        if self.draft {
            commands.insert(DraftPost);
        }
    }

    fn ingest_path(&self, commands: &mut EntityCommands, path: &Path) {
        commands.insert(Post(
            path.file_stem()
                .expect("Path requires file name")
                .to_string_lossy()
                .to_string(),
        ));
        let reg = Regex::new(r"/blog/(<year>[0-9]+)/\.*").unwrap();
        if let Some(caps) = reg.captures(&path.to_string_lossy()) {
            let year = &caps["year"];
            commands.insert(BlogYear(year.to_owned()));
        };
        commands.insert(default_bundle_from_path(path));
    }
}
