#![allow(clippy::cargo_common_metadata)]

use context::GlobalsContext;
pub use library::inject_lune_standard_libraries as inject_libraries;
use mlua::prelude::*;

pub mod context;
mod global;
mod globals;
pub mod library;
mod luaurc;

pub use self::global::LuneStandardGlobal;
pub use self::globals::version::set_global_version;

/**
    Injects all standard globals into the given Lua state / VM.

    This includes all enabled standard libraries, which can
    be used from Lua with `require("@lune/library-name")`.

    # Errors

    Errors when out of memory, or if *default* Lua globals are missing.
*/
pub fn inject_globals(lua: &Lua, globals_ctx: &GlobalsContext) -> LuaResult<()> {
    for global in LuneStandardGlobal::ALL {
        lua.globals()
            .set(global.name(), global.create(lua, globals_ctx)?)?;
    }

    Ok(())
}
