pub use builder::GlobalsContextBuilder;
use mlua::prelude::*;
use std::{borrow::Cow, collections::HashMap, path::PathBuf};

mod builder;

/**
    Use this enum to determine what type of lua object a module will return
*/
#[derive(Clone, Debug)]
pub enum LuneModuleCreator {
    LuaTable(fn(&Lua) -> LuaResult<LuaTable>),
    LuaValue(fn(&Lua) -> LuaResult<LuaValue>),
}

impl<'lua> IntoLua<'lua> for LuneModuleCreator {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        match self {
            LuneModuleCreator::LuaTable(creator) => creator(lua)?.into_lua(lua),
            LuneModuleCreator::LuaValue(creator) => creator(lua),
        }
    }
}

impl From<GlobalsContextBuilder> for GlobalsContext {
    fn from(val: GlobalsContextBuilder) -> Self {
        val.build()
    }
}

#[derive(Default, Clone, Debug)]
pub struct LuneModule {
    pub children: HashMap<&'static str, LuneModuleCreator>,
    pub alias: &'static str,
}

/**
    This struct provides customizable information to globals at [`LuneStandardGlobal`](../enum.LuneStandardGlobal.html)

    To create one, use [`GlobalsContextBuilder`](struct.GlobalsContextBuilder.html)
*/
#[derive(Default, Clone, Debug)]
pub struct GlobalsContext {
    pub modules: Vec<LuneModule>,
    pub scripts: HashMap<PathBuf, Cow<'static, [u8]>>,
}

impl GlobalsContext {
    #[must_use]
    pub fn get_alias(&self, s: &str) -> Option<&LuneModule> {
        self.modules.iter().find(|x| x.alias == s)
    }

    #[must_use]
    pub fn get_script<T: Into<PathBuf>>(&self, abs_path: T) -> Option<&Cow<'static, [u8]>> {
        let abs_path = abs_path.into();

        self.scripts
            .get(&abs_path)
            .or(self.scripts.get(&abs_path.with_extension("luau")))
            .or(self.scripts.get(&abs_path.with_extension("lua")))
    }
}
