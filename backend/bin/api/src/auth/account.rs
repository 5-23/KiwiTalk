use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;

use kiwi_talk_system::get_system_info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedAccount {
    pub profile: String,

    pub name: String,

    pub email: String,

    #[serde(with = "serde_byte_array")]
    pub token: Option<[u8; 64]>,
}

fn file_path() -> PathBuf {
    get_system_info().config_dir.join("saved_account")
}

pub async fn read() -> anyhow::Result<Option<SavedAccount>> {
    spawn_blocking(move || -> anyhow::Result<_> {
        let reader = BufReader::new(File::open(file_path())?);

        Ok(bincode::deserialize_from(reader)?)
    })
    .await?
}

pub async fn write(data: Option<SavedAccount>) -> anyhow::Result<()> {
    spawn_blocking(move || -> anyhow::Result<_> {
        let writer = BufWriter::new(File::create(file_path())?);

        bincode::serialize_into(writer, &data)?;

        Ok(())
    })
    .await?
    .context("cannot save login data")
}
