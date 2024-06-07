#![allow(clippy::missing_panics_doc)]

use std::{
    process::ExitCode,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use lune_std::context::GlobalsContextBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::{Functions, Scheduler};
use self_cell::self_cell;

use super::{RuntimeError, RuntimeResult};

// NOTE: We need to use self_cell to create a self-referential
// struct storing both the Lua VM and the scheduler. The scheduler
// needs to be created at the same time so that we can also create
// and inject the scheduler functions which will be used across runs.
self_cell! {
    struct RuntimeInner {
        owner: Rc<Lua>,
        #[covariant]
        dependent: Scheduler,
    }
}

impl RuntimeInner {
    fn create() -> LuaResult<Self> {
        let lua = Rc::new(Lua::new());

        lua.set_app_data(Rc::downgrade(&lua));
        lua.set_app_data(Vec::<String>::new());

        Self::try_new(lua, |lua| {
            let sched = Scheduler::new(lua);
            let fns = Functions::new(lua)?;

            // Overwrite some globals that are not compatible with our scheduler
            let co = lua.globals().get::<_, LuaTable>("coroutine")?;
            co.set("resume", fns.resume.clone())?;
            co.set("wrap", fns.wrap.clone())?;

            // Inject all the globals that are enabled
            let mut globals_ctx_builder = GlobalsContextBuilder::new();
            lune_std::inject_libraries(&mut globals_ctx_builder)?;

            #[cfg(test)]
            {
                use lune_std::context::LuneModuleCreator;
                use std::{borrow::Cow, env::current_dir};

                globals_ctx_builder.with_alias("custom", |modules| {
                    modules.insert(
                        "number",
                        LuneModuleCreator::LuaValue(|_| Ok(LuaValue::Number(6009.0))),
                    );

                    Ok(())
                })?;

                globals_ctx_builder.with_script(
                    current_dir()
                        .unwrap()
                        .join("tests/globals_ctx/tests/fake_script"),
                    Cow::from("return 2000".as_bytes()),
                );
            }

            let globals_ctx = globals_ctx_builder.build();
            lune_std::inject_globals(lua, &globals_ctx)?;

            // Sandbox the Luau VM and make it go zooooooooom
            lua.sandbox(true)?;

            // _G table needs to be injected again after sandboxing,
            // otherwise it will be read-only and completely unusable
            let g_table = lune_std::LuneStandardGlobal::GTable;
            lua.globals()
                .set(g_table.name(), g_table.create(lua, &globals_ctx)?)?;

            Ok(sched)
        })
    }

    fn lua(&self) -> &Lua {
        self.borrow_owner()
    }

    fn scheduler(&self) -> &Scheduler {
        self.borrow_dependent()
    }
}

/**
    A Lune runtime.
*/
pub struct Runtime {
    inner: RuntimeInner,
}

impl Runtime {
    /**
        Creates a new Lune runtime, with a new Luau VM.

        Injects standard globals and libraries if any of the `std` features are enabled.
    */
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: RuntimeInner::create().expect("Failed to create runtime"),
        }
    }

    /**
        Sets arguments to give in `process.args` for Lune scripts.
    */
    #[must_use]
    pub fn with_args<A, S>(self, args: A) -> Self
    where
        A: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let args = args.into_iter().map(Into::into).collect::<Vec<_>>();
        self.inner.lua().set_app_data(args);
        self
    }

    /**
        Runs a Lune script inside of the current runtime.

        This will preserve any modifications to global values / context.

        # Errors

        This function will return an error if the script fails to run.
    */
    pub async fn run(
        &mut self,
        script_name: impl AsRef<str>,
        script_contents: impl AsRef<[u8]>,
    ) -> RuntimeResult<ExitCode> {
        let lua = self.inner.lua();
        let sched = self.inner.scheduler();

        // Add error callback to format errors nicely + store status
        let got_any_error = Arc::new(AtomicBool::new(false));
        let got_any_inner = Arc::clone(&got_any_error);
        self.inner.scheduler().set_error_callback(move |e| {
            got_any_inner.store(true, Ordering::SeqCst);
            eprintln!("{}", RuntimeError::from(e));
        });

        // Load our "main" thread
        let main = lua
            .load(script_contents.as_ref())
            .set_name(script_name.as_ref());

        // Run it on our scheduler until it and any other spawned threads complete
        sched.push_thread_back(main, ())?;
        sched.run().await;

        // Return the exit code - default to FAILURE if we got any errors
        let exit_code = sched.get_exit_code().unwrap_or({
            if got_any_error.load(Ordering::SeqCst) {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        });

        Ok(exit_code)
    }
}
