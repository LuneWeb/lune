use mlua::Result as LuaResult;
use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use super::{GlobalsContext, LuneModule, LuneModuleCreator};

/**
    # Example
    ```
    // our module creator
    let create_pixels = |lua: &Lua| -> LuaResult<LuaTable> {
        ... // return a lua table
    };

    let builder = lune_std::context::GlobalsContextBuilder::new();

    // lua: require("@<alias-name>/pixels")
    builder.with_alias("<alias-name>", |modules| {
        insert_module!(modules, "pixels", LuneModuleCreator::LuaTable(create_pixels));

        Ok(())
    })?;

    lune_std::inject_globals(&lua, builder);
    ```
*/
#[derive(Default)]
pub struct GlobalsContextBuilder {
    modules: Vec<LuneModule>,
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
            // There are multiple ways of inserting a module

            // .1
            modules.insert("pixels", LuneModuleCreator::LuaTable(create_pixels));

            // .2
            // does the exact same thing as .1
            insert_module!(modules, "pixels", LuneModuleCreator::LuaTable(create_pixels));

            // .3
            // does the exact same thing as .1
            // but only if a feature flag with the name of "pixels" is enabled
            insert_feature_only_module!(modules, "pixels", LuneModuleCreator::LuaTable(create_pixels));

            Ok(())
        })?;
        ```
    */
    pub fn with_alias(
        &mut self,
        name: &'static str,
        handler: fn(&mut HashMap<&str, LuneModuleCreator>) -> LuaResult<()>,
    ) -> LuaResult<()> {
        let mut modules = HashMap::new();
        handler(&mut modules)?;

        let alias = LuneModule {
            alias: name,
            children: modules,
        };

        self.modules.push(alias);

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
            modules: self.modules,
            scripts: self.scripts,
        }
    }
}
