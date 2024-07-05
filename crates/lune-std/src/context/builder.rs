use mlua::Result as LuaResult;
use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use super::{GlobalsContext, LuauLibrary, LuauLibraryCreator};

#[derive(Default)]
pub struct GlobalsContextBuilder {
    libraries: Vec<LuauLibrary>,
    scripts: HashMap<PathBuf, Cow<'static, [u8]>>,
}

impl GlobalsContextBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /**
        # Errors

        Errors if the handler errors

        # Example
        ```
        let create_pixels = |lua: &Lua| -> LuaResult<LuaTable> {
            ... // return a lua table
        };

        builder.with_alias("<alias>", |modules| {
            modules.insert("pixels", LuneModuleCreator::LuaTable(create_pixels));

            Ok(())
        })?;
        ```
    */
    pub fn with_alias(
        &mut self,
        name: &'static str,
        handler: fn(&mut HashMap<&str, LuauLibraryCreator>) -> LuaResult<()>,
    ) -> LuaResult<()> {
        let mut modules = HashMap::new();
        handler(&mut modules)?;

        let alias = LuauLibrary {
            alias: name,
            children: modules,
        };

        self.libraries.push(alias);

        Ok(())
    }

    /**
        Define an absolute path for custom requires

        # Example
        ```
        let script = "return 100";

        globals_ctx_builder.with_script(
            current_dir()
                .unwrap()
                .join("path/to/script"),
            Cow::from(script.as_bytes()),
        );

        /*

        -- lua code
        local number = require("path/to/script")
        print(number) -- 100

         */
        ```
    */
    pub fn with_script(&mut self, path: impl Into<PathBuf>, content: Cow<'static, [u8]>) {
        self.scripts.insert(path.into(), content);
    }

    #[must_use]
    pub fn build(self) -> GlobalsContext {
        GlobalsContext {
            libraries: self.libraries,
            scripts: self.scripts,
        }
    }
}
