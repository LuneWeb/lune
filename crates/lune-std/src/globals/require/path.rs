use std::path::{Path, PathBuf};

use mlua::prelude::*;
use mlua::Error::ExternalError;

use super::context::*;

pub(super) async fn require<'lua, 'ctx>(
    lua: &'lua Lua,
    ctx: &'ctx RequireContext,
    source: &str,
    path: &str,
) -> LuaResult<LuaMultiValue<'lua>>
where
    'lua: 'ctx,
{
    let (abs_path, rel_path) = RequireContext::resolve_paths(source, path)?;
    require_abs_rel(lua, ctx, abs_path, rel_path).await
}

pub(super) async fn require_abs_rel<'lua, 'ctx>(
    lua: &'lua Lua,
    ctx: &'ctx RequireContext,
    abs_path: PathBuf, // Absolute to filesystem
    rel_path: PathBuf, // Relative to CWD (for displaying)
) -> LuaResult<LuaMultiValue<'lua>>
where
    'lua: 'ctx,
{
    // 0. Try to require from GlobalsContext.scripts
    let script = ctx.global_context.get_script(&abs_path);

    if let Some(content) = script {
        if ctx.is_cached(&abs_path)? {
            return ctx.get_from_cache(lua, abs_path);
        } else if ctx.is_pending(&abs_path)? {
            return ctx.wait_for_cache(lua, abs_path).await;
        }

        return ctx
            .load_with_caching(lua, abs_path, rel_path, Some(content))
            .await;
    }

    // 1. Try to require the exact path
    match require_inner(lua, ctx, &abs_path, &rel_path).await {
        Ok(res) => return Ok(res),
        Err(err) => {
            if !is_file_not_found_error(&err) {
                return Err(err);
            }
        }
    }

    // 2. Try to require the path with an added "luau" extension
    // 3. Try to require the path with an added "lua" extension
    for extension in ["luau", "lua"] {
        match require_inner(
            lua,
            ctx,
            &append_extension(&abs_path, extension),
            &append_extension(&rel_path, extension),
        )
        .await
        {
            Ok(res) => return Ok(res),
            Err(err) => {
                if !is_file_not_found_error(&err) {
                    return Err(err);
                }
            }
        }
    }

    // We didn't find any direct file paths, look
    // for directories with "init" files in them...
    let abs_init = abs_path.join("init");
    let rel_init = rel_path.join("init");

    // 4. Try to require the init path with an added "luau" extension
    // 5. Try to require the init path with an added "lua" extension
    for extension in ["luau", "lua"] {
        match require_inner(
            lua,
            ctx,
            &append_extension(&abs_init, extension),
            &append_extension(&rel_init, extension),
        )
        .await
        {
            Ok(res) => return Ok(res),
            Err(err) => {
                if !is_file_not_found_error(&err) {
                    return Err(err);
                }
            }
        }
    }

    // Nothing left to try, throw an error
    Err(LuaError::runtime(format!(
        "No file exists at the path '{}'",
        rel_path.display()
    )))
}

async fn require_inner<'lua, 'ctx>(
    lua: &'lua Lua,
    ctx: &'ctx RequireContext,
    abs_path: impl AsRef<Path>,
    rel_path: impl AsRef<Path>,
) -> LuaResult<LuaMultiValue<'lua>>
where
    'lua: 'ctx,
{
    let abs_path = abs_path.as_ref();
    let rel_path = rel_path.as_ref();

    if ctx.is_cached(abs_path)? {
        ctx.get_from_cache(lua, abs_path)
    } else if ctx.is_pending(abs_path)? {
        ctx.wait_for_cache(lua, &abs_path).await
    } else {
        ctx.load_with_caching(lua, &abs_path, &rel_path, None).await
    }
}

fn append_extension(path: impl Into<PathBuf>, ext: &'static str) -> PathBuf {
    let mut new = path.into();
    match new.extension() {
        // FUTURE: There's probably a better way to do this than converting to a lossy string
        Some(e) => new.set_extension(format!("{}.{ext}", e.to_string_lossy())),
        None => new.set_extension(ext),
    };
    new
}

fn is_file_not_found_error(err: &LuaError) -> bool {
    if let ExternalError(err) = err {
        err.as_ref().downcast_ref::<std::io::Error>().is_some()
    } else {
        false
    }
}
