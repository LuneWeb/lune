use std::io::Write;

use crate::context::GlobalsContext;
use lune_utils::fmt::{pretty_format_multi_value, Label, ValueFormatConfig};
use mlua::prelude::*;

const FORMAT_CONFIG: ValueFormatConfig = ValueFormatConfig::new()
    .with_max_depth(4)
    .with_colors_enabled(true);

pub fn create<'lua>(lua: &'lua Lua, _: &'lua GlobalsContext) -> LuaResult<LuaValue<'lua>> {
    let f = lua.create_function(|_, args: LuaMultiValue| {
        let formatted = format!(
            "{}\n{}\n",
            Label::Warn,
            pretty_format_multi_value(&args, &FORMAT_CONFIG)
        );
        let mut stdout = std::io::stdout();
        stdout.write_all(formatted.as_bytes())?;
        stdout.flush()?;
        Ok(())
    })?;
    f.into_lua(lua)
}
