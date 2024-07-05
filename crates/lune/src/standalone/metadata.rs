use std::{env, path::PathBuf};

use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::fs;

pub static CURRENT_EXE: Lazy<PathBuf> =
    Lazy::new(|| env::current_exe().expect("failed to get current exe"));

/*
    TODO: Right now all we do is append the bytecode to the end
    of the binary, but we will need a more flexible solution in
    the future to store many files as well as their metadata.

    The best solution here is most likely to use a well-supported
    and rust-native binary serialization format with a stable
    specification, one that also supports byte arrays well without
    overhead, so the best solution seems to currently be Postcard:

    https://github.com/jamesmunns/postcard
    https://crates.io/crates/postcard
*/

const MAGIC: &[u8; 5] = b"M8G2C";

/**
    Metadata for a standalone Lune executable. Can be used to
    discover and load the bytecode contained in a standalone binary.
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub scripts: Vec<LuauScript>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LuauScript(pub String, pub Vec<u8>);

impl Metadata {
    /**
        Returns whether or not the currently executing Lune binary
        is a standalone binary, and if so, the bytes of the binary.
    */
    pub async fn check_env() -> Result<Option<Metadata>> {
        let contents = fs::read(CURRENT_EXE.to_path_buf())
            .await
            .unwrap_or_default();
        if contents.ends_with(MAGIC) {
            match Self::from_bytes(&contents[0..contents.len() - MAGIC.len()]) {
                Ok(meta) => Ok(Some(meta)),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }

    /**
        Creates a patched standalone binary from the given script contents.
    */
    pub async fn create_env_patched_bin(
        base_exe_path: PathBuf,
        scripts: Vec<LuauScript>,
    ) -> Result<Vec<u8>> {
        let mut patched_bin = fs::read(base_exe_path).await?;

        // Append metadata to the end
        let mut buffer = [0u8; 512000];

        let bytes = postcard::to_slice(&Self { scripts }, &mut buffer).unwrap();
        patched_bin.extend_from_slice(bytes);

        // Append the length of metadata to the end
        let mut buffer = [0u8; 2];
        let length_as_bytes = postcard::to_slice(&bytes.len(), &mut buffer).unwrap();
        patched_bin.extend_from_slice(length_as_bytes);

        // Append the magic word to the end
        patched_bin.extend_from_slice(MAGIC);

        // println!("{length_as_bytes:?}");
        // println!("{}", bytes.len());

        Ok(patched_bin)
    }

    /**
        Tries to read a standalone binary from the given bytes.
    */
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = bytes.as_ref();
        // println!("{:?}", &bytes[bytes.len() - 8..bytes.len()]);

        let Ok(length) = postcard::from_bytes::<usize>(&bytes[bytes.len() - 2..bytes.len()]) else {
            bail!("Failed to get binary length")
        };

        // println!("{length}");

        let bytes = &bytes[0..bytes.len() - 2];
        let bytes = &bytes[bytes.len() - length..bytes.len()];
        let metadata = postcard::from_bytes::<Metadata>(bytes);

        if metadata.is_err() {
            bail!("Metadata is not attached: {}", metadata.err().unwrap())
        }

        Ok(metadata.unwrap())
    }
}
