use std::fmt::Debug;

use c_to_wasm_compiler::Compiler;
use c_to_wasm_compiler::configuration::{Configuration, Debugging, Profile};
use c_to_wasm_compiler::configuration_builder::ConfigurationBuilder;
use rayon::prelude::*;
use wasmtime::{WasmParams, WasmResults};

const PROFILE_OPTS: &[Profile; 4] = {
    use Profile::{O0, O1, O2, O3};
    &[O0, O1, O2, O3]
};

const DEBUG_OPTS: &[Debugging; 2] = {
    use Debugging::{Disabled, Enabled};
    &[Enabled, Disabled]
};

#[test]
fn test_semver() -> anyhow::Result<()> {
    Compiler::version()?;
    Ok(())
}

#[test]
fn test_different_variants() {
    DEBUG_OPTS.par_iter().for_each(|debug_option| {
        PROFILE_OPTS.par_iter().for_each(|profile_option| {
            let source = r#"
                #include <stdint.h>
                
                __attribute__((export_name("fac")))
                int32_t fac(int32_t n) {
                    if (n == 0) {
                        return 1;
                    } else {
                        return n * fac(n - 1);
                    }
                }
            "#;

            let config = ConfigurationBuilder::init()
                .debugging(*debug_option)
                .profile(*profile_option)
                .source(source.into())
                .build();

            // Assert on the outcome
            assert_outcome(&config, "fac", 5, &120).unwrap();
        });
    });
}

#[test]
fn recursive_input() -> anyhow::Result<()> {
    let source = r#"
        #include <stdint.h>
        
        __attribute__((export_name("fac")))
        int32_t fac(int32_t n) {
            if (n == 0) {
                return 1;
            } else {
                return n * fac(n - 1);
            }
        }
    "#;

    let config = ConfigurationBuilder::init()
        .debugging(Debugging::Enabled)
        .profile(Profile::O0)
        .source(source.into())
        .build();

    assert_outcome(&config, "fac", 5, &120)?;
    Ok(())
}

#[test]
fn unsafe_c_overflow() -> anyhow::Result<()> {
    let source = r#"
        #include <stdint.h>
        
        __attribute__((export_name("overflow")))
        int32_t overflow(int32_t a, int32_t b) {
            return a + b;
        }
    "#;

    let config = ConfigurationBuilder::init()
        .debugging(Debugging::Disabled)
        .profile(Profile::O3)
        .source(source.into())
        .build();

    assert_outcome(&config, "overflow", (i32::MAX, 1), &i32::MIN)?;
    Ok(())
}

#[test]
fn failing_compilation() {
    let config = ConfigurationBuilder::init()
        .debugging(Debugging::Disabled)
        .profile(Profile::O3)
        .source("no c source code".into())
        .build();

    assert!(
        Compiler
            .compile(&config)
            .is_err_and(|err| matches!(err, c_to_wasm_compiler::error::Error::Unsuccesful(_)))
    );
}

fn assert_outcome<Params: WasmParams, Results: WasmResults + Eq + Debug>(
    config: &Configuration,
    exported: &str,
    params: Params,
    results: &Results,
) -> anyhow::Result<()> {
    let output = {
        /* Compiling C source code into Wasm */
        Compiler.compile(config)?
    };

    {
        /* Running the module */
        use wasmtime::{Engine, Instance, Module, Store};
        let engine = Engine::default();
        let module = Module::from_binary(&engine, &output)?;
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;
        let outcome = instance
            .get_typed_func::<Params, Results>(&mut store, exported)?
            .call(&mut store, params)?;

        assert_eq!(outcome, *results);
    };

    Ok(())
}

#[test]
fn configuration_settings() {
    use c_to_wasm_compiler::configuration::Debugging::Disabled;
    use c_to_wasm_compiler::configuration::Profile::O0;

    let config = ConfigurationBuilder::init()
        .debugging(Disabled)
        .profile(O0)
        .source("hi there!".into())
        .build();

    assert_eq!(config.debugging(), &Disabled);
    assert_eq!(config.profile(), &O0);
    assert_eq!(config.source(), "hi there!");
}
