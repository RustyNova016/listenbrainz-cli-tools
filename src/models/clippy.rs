use std::fmt::Display;

use color_eyre::owo_colors::OwoColorize;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;

pub trait MbClippyLint: Sized {
    async fn check(
        conn: &mut sqlx::SqliteConnection,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error>;

    fn get_name() -> &'static str;

    async fn get_body(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<impl Display, crate::Error>;

    async fn get_links(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error>;

    async fn get_hints(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error>;
}

pub struct MbClippyLintLink {
    pub name: String,
    pub url: String,
}

impl Display for MbClippyLintLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.url.blue())?;

        Ok(())
    }
}

pub struct MbClippyLintHint {
    text: String,
}

impl Display for MbClippyLintHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "= Hint: {}", self.text)?;

        Ok(())
    }
}