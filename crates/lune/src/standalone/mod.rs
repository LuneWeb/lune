use std::{
    env::{self, current_dir},
    process::ExitCode,
};

use anyhow::{bail, Result};
use lune::Runtime;
use lune_std::context::GlobalsContextBuilder;

pub(crate) mod metadata;
pub(crate) mod tracer;

use self::metadata::Metadata;

/**
    Returns whether or not the currently executing Lune binary
    is a standalone binary, and if so, the bytes of the binary.
*/
pub async fn check() -> Option<Metadata> {
    Metadata::check_env().await
}

/**
    Discovers, loads and executes the bytecode contained in a standalone binary.
*/
pub async fn run(meta: Metadata) -> Result<ExitCode> {
    // The first argument is the path to the current executable
    let args = env::args().skip(1).collect::<Vec<_>>();

    let mut ctx_builder = GlobalsContextBuilder::new();
    let cwd = current_dir().unwrap();

    for script in &meta.scripts {
        ctx_builder.with_script(cwd.join(script.0.clone()), script.1.clone().into());
    }

    if meta.scripts.is_empty() {
        bail!("Metadata contains 0 bundled scripts")
    }

    let init = &meta.scripts[0];

    let result = Runtime::new(Some(ctx_builder))
        .with_args(args)
        .run(
            cwd.join(init.0.clone()).to_string_lossy().to_string(),
            init.1.clone(),
        )
        .await;

    Ok(match result {
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
        Ok(code) => code,
    })
}
