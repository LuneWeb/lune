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
        let decompresesd_bin = postcard::to_extend(&Self { scripts }, Vec::new()).unwrap();

        // Append compressed binary
        let compressed_bin = lz4_compression::compress::compress(&decompresesd_bin);
        let compressed_len = &compressed_bin.len();
        patched_bin.extend(compressed_bin);

        // Append length of compressed binary
        let mut patched_bin = postcard::to_extend(compressed_len, patched_bin).unwrap();

        // Append the magic word to the end
        patched_bin.extend_from_slice(MAGIC);

        Ok(patched_bin)
    }

    /**
        Tries to read a standalone binary from the given bytes.
    */
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = bytes.as_ref();

        let length_bytes_len = match bytes.get(bytes.len().saturating_sub(3)..) {
            Some([0, 0, _]) => 1,
            Some([0, _, _]) => 2,
            _ => 3,
        };

        let length_bytes = &bytes[bytes.len() - length_bytes_len..bytes.len()];

        let Ok(length) = postcard::from_bytes::<usize>(length_bytes) else {
            bail!("Failed to get binary length")
        };

        let bytes = &bytes[0..bytes.len() - length_bytes_len];
        let compressed_bin = &bytes[bytes.len() - length..bytes.len()];
        let decompressed_bin = lz4_compression::decompress::decompress(compressed_bin).unwrap();
        let metadata = postcard::from_bytes::<Metadata>(&decompressed_bin);

        if metadata.is_err() {
            bail!("Metadata is not attached: {}", metadata.err().unwrap())
        }

        Ok(metadata.unwrap())
    }
}
